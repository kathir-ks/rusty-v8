// Converted from V8 C++ source files:
// Header: container-utils.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::cmp::min;
    use std::iter::Iterator;

    // Returns true iff the {element} is found in the {container}.
    pub fn contains<C, T>(container: &C, element: &T) -> bool
    where
        C: IntoIterator,
        C::Item: PartialEq<T>,
        T: std::cmp::PartialEq<C::Item>,
    {
        container.into_iter().any(|item| item == *element)
    }

    // Returns the first index of {element} in {container}. Returns None if
    // {container} does not contain {element}.
    pub fn index_of<C, T>(container: &C, element: &T) -> Option<usize>
    where
        C: IntoIterator,
        C::Item: PartialEq<T>,
        T: PartialEq<C::Item>,
    {
        container
            .into_iter()
            .position(|item| item == *element)
    }

    // Returns the index of the first element in {container} that satisfies
    // {predicate}. Returns None if no element satisfies {predicate}.
    pub fn index_of_if<C, P>(container: &C, predicate: P) -> Option<usize>
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().position(predicate)
    }

    // Removes {count} elements from {container} starting at {index}. If {count} is
    // larger than the number of elements after {index}, all elements after {index}
    // are removed. Returns the number of removed elements.
    pub fn erase_at<T>(container: &mut Vec<T>, index: usize, count: usize) -> usize {
        if container.len() <= index {
            return 0;
        }

        let count = min(count, container.len() - index);
        container.drain(index..index + count);
        count
    }

    // Removes all elements from {container} that satisfy {predicate}. Returns the
    // number of removed elements.
    pub fn erase_if<C, P>(container: &mut C, predicate: P) -> usize
    where
        C: std::ops::DerefMut<Target = Vec<_>>,
        P: Fn(&C::Item) -> bool,
    {
        let mut removed_count = 0;
        let mut i = 0;
        while i < container.len() {
            if predicate(&container[i]) {
                container.remove(i);
                removed_count += 1;
            } else {
                i += 1;
            }
        }
        removed_count
    }

    // Helper for std::count_if.
    pub fn count_if<C, P>(container: &C, predicate: P) -> usize
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().filter(|item| predicate(item)).count()
    }

    // Helper for std::all_of.
    pub fn all_of<C, P>(container: &C, predicate: P) -> bool
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().all(predicate)
    }

    pub fn all_of_bool<C>(container: &C) -> bool
    where
        C: IntoIterator,
        C::Item: std::convert::From<bool>,
    {
        container
            .into_iter()
            .all(|value| bool::from(value.into()))
    }

    // Helper for std::any_of.
    pub fn any_of<C, P>(container: &C, predicate: P) -> bool
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().any(predicate)
    }

    pub fn any_of_bool<C>(container: &C) -> bool
    where
        C: IntoIterator,
        C::Item: std::convert::From<bool>,
    {
        container
            .into_iter()
            .any(|value| bool::from(value.into()))
    }

    // Helper for std::none_of.
    pub fn none_of<C, P>(container: &C, predicate: P) -> bool
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().all(|item| !predicate(item))
    }

    // Helper for std::sort.
    pub fn sort<C>(container: &mut C)
    where
        C: std::ops::DerefMut<Target = Vec<_>>,
        C::Item: Ord,
    {
        container.sort();
    }

    pub fn sort_by<C, Comp>(container: &mut C, comp: Comp)
    where
        C: std::ops::DerefMut<Target = Vec<_>>,
        Comp: FnMut(&C::Item, &C::Item) -> std::cmp::Ordering,
    {
        container.sort_by(comp);
    }

    // Returns true iff all elements of {container} compare equal using operator==.
    pub fn all_equal<C>(container: &C) -> bool
    where
        C: IntoIterator,
        C::Item: PartialEq,
    {
        let mut iter = container.into_iter();
        if let Some(first) = iter.next() {
            for element in iter {
                if element != first {
                    return false;
                }
            }
            return true;
        }
        true // Empty container is considered all equal
    }

    // Returns true iff all elements of {container} compare equal to {value} using
    // operator==.
    pub fn all_equal_to<C, T>(container: &C, value: &T) -> bool
    where
        C: IntoIterator,
        C::Item: PartialEq<T>,
        T: PartialEq<C::Item>,
    {
        container.into_iter().all(|item| item == *value)
    }

    // Appends to vector {v} all the elements in the range {std::begin(container)}
    // and {std::end(container)}.
    pub fn vector_append<V, C, T>(v: &mut V, container: &C)
    where
        V: std::ops::DerefMut<Target = Vec<T>>,
        C: IntoIterator<Item = T>,
        T: Clone,
    {
        v.extend(container.into_iter());
    }
}
