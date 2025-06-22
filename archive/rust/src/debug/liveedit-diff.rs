// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod liveedit_diff {
    use std::cmp::{max, min};
    use std::collections::HashMap;
    use std::ops::{Index, IndexMut};

    /// Trait for comparing elements in the input sequences.
    pub trait ComparatorInput {
        fn get_length1(&self) -> usize;
        fn get_length2(&self) -> usize;
        fn equals(&self, index1: usize, index2: usize) -> bool;
    }

    /// Trait for writing the diff result.
    pub trait ComparatorOutput {
        fn add_chunk(&mut self, start1: usize, start2: usize, length1: usize, length2: usize);
    }

    /// Calculates the difference between two sequences and writes the result
    /// to the given output.
    pub fn calculate_difference(input: &dyn ComparatorInput, result_writer: &mut dyn ComparatorOutput) {
        MyersDiffer::myers_diff(input, result_writer);
    }

    // Implements Myer's Algorithm from
    // "An O(ND) Difference Algorithm and Its Variations", particularly the
    // linear space refinement mentioned in section 4b.
    //
    // The differ is input agnostic.
    //
    // The algorithm works by finding the shortest edit string (SES) in the edit
    // graph. The SES describes how to get from a string A of length N to a string
    // B of length M via deleting from A and inserting from B.
    //
    // Example: A = "abbaa", B = "abab"
    //
    //                  A
    //
    //          a   b   b   a    a
    //        o---o---o---o---o---o
    //      a | \ |   |   | \ | \ |
    //        o---o---o---o---o---o
    //      b |   | \ | \ |   |   |
    //  B     o---o---o---o---o---o
    //      a | \ |   |   | \ | \ |
    //        o---o---o---o---o---o
    //      b |   | \ | \ |   |   |
    //        o---o---o---o---o---o
    //
    // The edit graph is constructed with the characters from string A on the x-axis
    // and the characters from string B on the y-axis. Starting from (0, 0) we can:
    //
    //     - Move right, which is equivalent to deleting from A
    //     - Move downwards, which is equivalent to inserting from B
    //     - Move diagonally if the characters from string A and B match, which
    //       means no insertion or deletion.
    //
    // Any path from (0, 0) to (N, M) describes a valid edit string, but we try to
    // find the path with the most diagonals, conversely that is the path with the
    // least insertions or deletions.
    // Note that a path with "D" insertions/deletions is called a D-path.
    struct MyersDiffer<'a> {
        input_: &'a dyn ComparatorInput,
        output_: &'a mut dyn ComparatorOutput,

        // Stores the x-value of the furthest reaching path for each k-diagonal.
        // k-diagonals are numbered from '-height' to 'width', centered on (0,0) and
        // are defined by y(x) = x - k.
        fr_forward_: FurthestReaching,

        // Stores the x-value of the furthest reaching reverse path for each
        // l-diagonal. l-diagonals are numbered from '-width' to 'height' and centered
        // on 'bottom_right' of the edit graph area.
        // k-diagonals and l-diagonals represent the same diagonals. While we refer to
        // the diagonals as k-diagonals when calculating SES from (0,0), we refer to
        // the diagonals as l-diagonals when calculating SES from (M,N).
        // The corresponding k-diagonal name of an l-diagonal is: k = l + delta
        // where delta = width -height.
        fr_reverse_: FurthestReaching,
    }

    impl<'a> MyersDiffer<'a> {
        fn new(input: &'a dyn ComparatorInput, output: &'a mut dyn ComparatorOutput) -> Self {
            let len1 = input.get_length1();
            let len2 = input.get_length2();
            MyersDiffer {
                input_: input,
                output_: output,
                fr_forward_: FurthestReaching::new(len1 + len2 + 1),
                fr_reverse_: FurthestReaching::new(len1 + len2 + 1),
            }
        }

        fn find_edit_path(&mut self) -> Option<Path> {
            self.find_edit_path_between(Point { x: 0, y: 0 }, Point { x: self.input_.get_length1() as i32, y: self.input_.get_length2() as i32 })
        }

        // Returns the path of the SES between `from` and `to`.
        fn find_edit_path_between(&mut self, from: Point, to: Point) -> Option<Path> {
            // Divide the area described by `from` and `to` by finding the
            // middle snake ...
            let snake = self.find_middle_snake(from, to);

            if snake.is_none() {
                return None;
            }

            let snake = snake.unwrap();

            // ... and then conquer the two resulting sub-areas.
            let head = self.find_edit_path_between(from, snake.from);
            let tail = self.find_edit_path_between(snake.to, to);

            // Combine `head` and `tail` or use the snake start/end points for
            // zero-size areas.
            let mut result = Path { points: Vec::new() };
            if let Some(h) = head {
                result.add(h);
            } else {
                result.add(Path { points: vec![snake.from] });
            }

            if let Some(t) = tail {
                result.add(t);
            } else {
                result.add(Path { points: vec![snake.to] });
            }
            Some(result)
        }

        // Returns the snake in the middle of the area described by `from` and `to`.
        //
        // Incrementally calculates the D-paths (starting from 'from') and the
        // "reverse" D-paths (starting from 'to') until we find a "normal" and a
        // "reverse" path that overlap. That is we first calculate the normal
        // and reverse 0-path, then the normal and reverse 1-path and so on.
        //
        // If a step from a (d-1)-path to a d-path overlaps with a reverse path on
        // the same diagonal (or the other way around), then we consider that step
        // our middle snake and return it immediately.
        fn find_middle_snake(&mut self, from: Point, to: Point) -> Option<Snake> {
            let area = EditGraphArea { top_left: from, bottom_right: to };
            if area.size() == 0 {
                return None;
            }

            // Initialise the furthest reaching vectors with an "artificial" edge
            // from (0, -1) -> (0, 0) and (N, -M) -> (N, M) to serve as the initial
            // snake when d = 0.
            self.fr_forward_[1] = area.top_left.x;
            self.fr_reverse_[-1] = area.bottom_right.x;

            for d in 0..=(area.size() as f32 / 2.0).ceil() as i32 {
                if let Some(snake) = self.shortest_edit_forward(area, d) {
                    return Some(snake);
                }
                if let Some(snake) = self.shortest_edit_reverse(area, d) {
                    return Some(snake);
                }
            }

            None
        }

        // Greedily calculates the furthest reaching `d`-paths for each k-diagonal
        // where k is in [-d, d].  For each k-diagonal we look at the furthest
        // reaching `d-1`-path on the `k-1` and `k+1` depending on which is further
        // along the x-axis we either add an insertion from the `k+1`-diagonal or
        // a deletion from the `k-1`-diagonal. Then we follow all possible diagonal
        // moves and finally record the result as the furthest reaching path on the
        // k-diagonal.
        fn shortest_edit_forward(&mut self, area: EditGraphArea, d: i32) -> Option<Snake> {
            let mut from: Point;
            let mut to: Point;
            // We alternate between looking at odd and even k-diagonals. That is
            // because when we extend a `d-path` by a single move we can at most move
            // one diagonal over. That is either move from `k-1` to `k` or from `k+1` to
            // `k`. That is if `d` is even (odd) then we require only the odd (even)
            // k-diagonals calculated in step `d-1`.
            for k in (-d..=d).step_by(2) {
                if k == -d || (k != d && self.fr_forward_[k - 1] < self.fr_forward_[k + 1]) {
                    // Move downwards, i.e. add an insertion, because either we are at the
                    // edge and downwards is the only way we can move, or because the
                    // `d-1`-path along the `k+1` diagonal reaches further on the x-axis
                    // than the `d-1`-path along the `k-1` diagonal.
                    from = Point { x: self.fr_forward_[k + 1], y: 0 }; // y is not actually used here
                    to = Point { x: self.fr_forward_[k + 1], y: 0 }; // y is not actually used here
                } else {
                    // Move right, i.e. add a deletion.
                    from = Point { x: self.fr_forward_[k - 1], y: 0 }; // y is not actually used here
                    to = Point { x: self.fr_forward_[k - 1] + 1, y: 0 }; // y is not actually used here
                }

                // Calculate y via y = x - k. We need to adjust k though since the k=0
                // diagonal is centered on `area.top_left` and not (0, 0).
                to.y = area.top_left.y + (to.x - area.top_left.x) - k;
                from.y = if d == 0 || from.x != to.x { to.y } else { to.y - 1 };

                // Extend the snake diagonally as long as we can.
                while to < area.bottom_right && self.input_.equals(to.x as usize, to.y as usize) {
                    to.x += 1;
                    to.y += 1;
                }

                self.fr_forward_[k] = to.x;

                // Check whether there is a reverse path on this k-diagonal which we
                // are overlapping with. If yes, that is our snake.
                let odd = area.delta() % 2 != 0;
                let l = k - area.delta();
                if odd && l >= (-d + 1) && l <= d - 1 && to.x >= self.fr_reverse_[l] {
                    return Some(Snake { from, to });
                }
            }
            None
        }

        // Greedily calculates the furthest reaching reverse `d`-paths for each
        // l-diagonal where l is in [-d, d].
        // Works the same as `ShortestEditForward` but we move upwards and left
        // instead.
        fn shortest_edit_reverse(&mut self, area: EditGraphArea, d: i32) -> Option<Snake> {
            let mut from: Point;
            let mut to: Point;
            // We alternate between looking at odd and even l-diagonals. That is
            // because when we extend a `d-path` by a single move we can at most move
            // one diagonal over. That is either move from `l-1` to `l` or from `l+1` to
            // `l`. That is if `d` is even (odd) then we require only the odd (even)
            // l-diagonals calculated in step `d-1`.
            for l in ((-d..=d).rev()).step_by(2) {
                if l == d || (l != -d && self.fr_reverse_[l - 1] > self.fr_reverse_[l + 1]) {
                    // Move upwards, i.e. add an insertion, because either we are at the
                    // edge and upwards is the only way we can move, or because the
                    // `d-1`-path along the `l-1` diagonal reaches further on the x-axis
                    // than the `d-1`-path along the `l+1` diagonal.
                    from = Point { x: self.fr_reverse_[l - 1], y: 0 };
                    to = Point { x: self.fr_reverse_[l - 1], y: 0 };
                } else {
                    // Move left, i.e. add a deletion.
                    from = Point { x: self.fr_reverse_[l + 1], y: 0 };
                    to = Point { x: self.fr_reverse_[l + 1] - 1, y: 0 };
                }

                // Calculate y via y = x - k. We need to adjust k though since the k=0
                // diagonal is centered on `area.top_left` and not (0, 0).
                let k = l + area.delta();
                to.y = area.top_left.y + (to.x - area.top_left.x) - k;
                from.y = if d == 0 || from.x != to.x { to.y } else { to.y + 1 };

                // Extend the snake diagonally as long as we can.
                while area.top_left < to && self.input_.equals((to.x - 1) as usize, (to.y - 1) as usize) {
                    to.x -= 1;
                    to.y -= 1;
                }

                self.fr_reverse_[l] = to.x;

                // Check whether there is a path on this k-diagonal which we
                // are overlapping with. If yes, that is our snake.
                let even = area.delta() % 2 == 0;
                if even && k >= -d && k <= d && to.x <= self.fr_forward_[k] {
                    // Invert the points so the snake goes left to right, top to bottom.
                    return Some(Snake { from: to, to: from });
                }
            }
            None
        }

        // Takes an edit path and "fills in the blanks". That is we notify the
        // `ResultWriter` after each single downwards, left or diagonal move.
        fn write_result(&mut self, path: &Path) {
            let mut writer = ResultWriter {
                output_: self.output_,
                change_is_ongoing_: false,
                change_start_: None,
            };

            for i in 1..path.points.len() {
                let mut p1 = path.points[i - 1];
                let p2 = path.points[i];

                p1 = self.walk_diagonal(&mut writer, p1, p2);
                let cmp = (p2.x - p1.x) - (p2.y - p1.y);
                if cmp == -1 {
                    writer.record_insertion_or_deletion(p1);
                    p1.y += 1;
                } else if cmp == 1 {
                    writer.record_insertion_or_deletion(p1);
                    p1.x += 1;
                }

                p1 = self.walk_diagonal(&mut writer, p1, p2);
                debug_assert!(p1.x == p2.x && p1.y == p2.y);
            }

            // Write one diagonal in the end to flush out any open chunk.
            writer.record_no_modification(path.points.last().unwrap());
        }

        fn walk_diagonal(&mut self, writer: &mut ResultWriter, mut p1: Point, p2: Point) -> Point {
            while p1.x < p2.x && p1.y < p2.y && self.input_.equals(p1.x as usize, p1.y as usize) {
                writer.record_no_modification(p1);
                p1.x += 1;
                p1.y += 1;
            }
            p1
        }

        fn myers_diff(input: &dyn ComparatorInput, output: &mut dyn ComparatorOutput) {
            let mut differ = MyersDiffer::new(input, output);
            let result = differ.find_edit_path();

            if let Some(path) = result {
                differ.write_result(&path);
            } // Empty input doesn't produce a path
        }
    }

    // A point in the edit graph.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl PartialOrd for Point {
        fn partial_cmp(&self, other: Option<&Self>) -> Option<std::cmp::Ordering> {
            match other {
                Some(other) => {
                    if self.x < other.x && self.y < other.y {
                        Some(std::cmp::Ordering::Less)
                    } else if self.x > other.x && self.y > other.y {
                        Some(std::cmp::Ordering::Greater)
                    } else if self == other {
                        Some(std::cmp::Ordering::Equal)
                    }
                    else {
                        None
                    }
                },
                None => None,
            }
        }

        fn lt(&self, other: &Self) -> bool {
            self.x < other.x && self.y < other.y
        }

        fn le(&self, other: &Self) -> bool {
            (self.x < other.x && self.y < other.y) || (self.x == other.x && self.y == other.y)
        }

        fn gt(&self, other: &Self) -> bool {
            self.x > other.x && self.y > other.y
        }

        fn ge(&self, other: &Self) -> bool {
            (self.x > other.x && self.y > other.y) || (self.x == other.x && self.y == other.y)
        }
    }

    // Describes a rectangle in the edit graph.
    #[derive(Debug, Copy, Clone)]
    struct EditGraphArea {
        top_left: Point,
        bottom_right: Point,
    }

    impl EditGraphArea {
        fn width(&self) -> i32 {
            self.bottom_right.x - self.top_left.x
        }
        fn height(&self) -> i32 {
            self.bottom_right.y - self.top_left.y
        }
        fn size(&self) -> i32 {
            self.width() + self.height()
        }
        fn delta(&self) -> i32 {
            self.width() - self.height()
        }
    }

    // A path or path-segment through the edit graph. Not all points along
    // the path are necessarily listed since it is trivial to figure out all
    // the concrete points along a snake.
    #[derive(Debug, Clone)]
    struct Path {
        points: Vec<Point>,
    }

    impl Path {
        fn add(&mut self, p: Path) {
            self.points.extend(p.points.iter());
        }
    }

    // A snake is a path between two points that is either:
    //
    //     - A single right or down move followed by a (possibly empty) list of
    //       diagonals (in the normal case).
    //     - A (possibly empty) list of diagonals followed by a single right or
    //       or down move (in the reverse case).
    #[derive(Debug, Copy, Clone)]
    struct Snake {
        from: Point,
        to: Point,
    }

    // A thin wrapper around std::vec::Vec<i32> that allows negative indexing.
    //
    // This class stores the x-value of the furthest reaching path
    // for each k-diagonal. k-diagonals are numbered from -M to N and defined
    // by y(x) = x - k.
    //
    // We only store the x-value instead of the full point since we can
    // calculate y via y = x - k.
    struct FurthestReaching {
        v_: Vec<i32>,
    }

    impl FurthestReaching {
        fn new(size: usize) -> Self {
            FurthestReaching { v_: vec![0; size] }
        }
    }

    impl Index<i32> for FurthestReaching {
        type Output = i32;

        fn index(&self, index: i32) -> &Self::Output {
            let idx = if index >= 0 { index as usize } else { (self.v_.len() as i32 + index) as usize };
            &self.v_[idx]
        }
    }

    impl IndexMut<i32> for FurthestReaching {
        fn index_mut(&mut self, index: i32) -> &mut Self::Output {
            let idx = if index >= 0 { index as usize } else { (self.v_.len() as i32 + index) as usize };
            &mut self.v_[idx]
        }
    }

    // Small helper class that converts a "shortest edit script" path into a
    // source mapping. The result is a list of "chunks" where each "chunk"
    // describes a range in the input string and where it can now be found
    // in the output string.
    //
    // The list of chunks can be calculated in a simple pass over all the points
    // of the edit path:
    //
    //     - For any diagonal we close and report the current chunk if there is
    //       one open at the moment.
    //     - For an insertion or deletion we open a new chunk if none is ongoing.
    struct ResultWriter<'a> {
        output_: &'a mut dyn ComparatorOutput,
        change_is_ongoing_: bool,
        change_start_: Option<Point>,
    }

    impl<'a> ResultWriter<'a> {
        fn record_no_modification(&mut self, from: Point) {
            if !self.change_is_ongoing_ {
                return;
            }

            // We close the current chunk, going from `change_start_` to `from`.
            debug_assert!(self.change_start_.is_some());
            let change_start = self.change_start_.unwrap();
            self.output_.add_chunk(
                change_start.x as usize,
                change_start.y as usize,
                (from.x - change_start.x) as usize,
                (from.y - change_start.y) as usize,
            );
            self.change_is_ongoing_ = false;
        }

        fn record_insertion_or_deletion(&mut self, from: Point) {
            if self.change_is_ongoing_ {
                return;
            }

            // We start a new chunk beginning at `from`.
            self.change_start_ = Some(from);
            self.change_is_ongoing_ = true;
        }
    }
}