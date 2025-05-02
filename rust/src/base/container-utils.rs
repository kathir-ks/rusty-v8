pub mod container_utils {
    /// Returns true iff the `element` is found in the `container`.
    pub fn contains<C, T>(container: &C, element: &T) -> bool
    where
        C: IntoIterator,
        C::Item: PartialEq<T>,
        T: PartialEq<C::Item>,
    {
        container.into_iter().any(|x| x == *element)
    }

    /// Returns the first index of `element` in `container`. Returns `None` if
    /// `container` does not contain `element`.
    pub fn index_of<C, T>(container: &C, element: &T) -> Option<usize>
    where
        C: IntoIterator,
        C::Item: PartialEq<T>,
        T: PartialEq<C::Item>,
    {
        container
            .into_iter()
            .position(|x| x == *element)
    }

    /// Returns the index of the first element in `container` that satisfies
    /// `predicate`. Returns `None` if no element satisfies `predicate`.
    pub fn index_of_if<C, P>(container: &C, predicate: P) -> Option<usize>
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().position(predicate)
    }

    /// Removes `count` elements from `container` starting at `index`. If `count` is
    /// larger than the number of elements after `index`, all elements after `index`
    /// are removed. Returns the number of removed elements.
    pub fn erase_at<T>(container: &mut Vec<T>, index: usize, count: usize) -> usize {
        if container.len() <= index {
            return 0;
        }

        let count = std::cmp::min(count, container.len() - index);
        container.drain(index..index + count);
        count
    }

    /// Removes all elements from `container` that satisfy `predicate`. Returns the
    /// number of removed elements.
    pub fn erase_if<T, P>(container: &mut Vec<T>, predicate: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        let initial_len = container.len();
        container.retain(|x| !predicate(x));
        initial_len - container.len()
    }

    /// Helper for `count_if`.
    pub fn count_if<C, P>(container: &C, predicate: P) -> usize
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().filter(|x| predicate(x)).count()
    }

    /// Helper for `all_of`.
    pub fn all_of<C, P>(container: &C, predicate: P) -> bool
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().all(predicate)
    }

    /// Helper for `all_of` where predicate is implicit conversion to bool.
    pub fn all_of_bool<C, T>(container: &C) -> bool
    where
        C: IntoIterator<Item = T>,
        T: std::convert::AsRef<bool>,
    {
        container.into_iter().all(|value| *value.as_ref())
    }

    /// Helper for `any_of`.
    pub fn any_of<C, P>(container: &C, predicate: P) -> bool
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().any(predicate)
    }

    /// Helper for `any_of` where predicate is implicit conversion to bool.
    pub fn any_of_bool<C, T>(container: &C) -> bool
    where
        C: IntoIterator<Item = T>,
        T: std::convert::AsRef<bool>,
    {
        container.into_iter().any(|value| *value.as_ref())
    }

    /// Helper for `none_of`.
    pub fn none_of<C, P>(container: &C, predicate: P) -> bool
    where
        C: IntoIterator,
        P: Fn(&C::Item) -> bool,
    {
        container.into_iter().all(|x| !predicate(x))
    }

    /// Helper for `sort`.
    pub fn sort<T: Ord>(container: &mut [T]) {
        container.sort();
    }

    /// Helper for `sort` with custom comparator.
    pub fn sort_by<T, F>(container: &mut [T], compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        container.sort_by(compare);
    }

    /// Returns true iff all elements of `container` compare equal using `==`.
    pub fn all_equal<C, T>(container: &C) -> bool
    where
        C: IntoIterator<Item = T>,
        T: PartialEq,
    {
        let mut iter = container.into_iter();
        if let Some(first) = iter.next() {
            iter.all(|v| v == first)
        } else {
            true
        }
    }

    /// Returns true iff all elements of `container` compare equal to `value` using
    /// `==`.
    pub fn all_equal_value<C, T>(container: &C, value: &T) -> bool
    where
        C: IntoIterator<Item = T>,
        T: PartialEq,
    {
        container.into_iter().all(|v| v == *value)
    }

    /// Appends to vector `v` all the elements in `container`.
    pub fn vector_append<T>(v: &mut Vec<T>, container: &impl IntoIterator<Item = T>)
    where T: Clone
    {
        v.extend(container.into_iter().cloned());
    }
}