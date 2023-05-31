#[cfg(test)] mod merge_pairs_test;

pub mod merge_pairs_ext;

use std::iter::Peekable;

/// Merge two **sorted** [`Iterator`] of `(key, value)` pairs in **ascending** order into a single
/// iterator. Duplicate keys are reduce by the provided `merge_value()` function.
///
/// The provided `merge_value(V,V)->MERGED` function determines how to merge two values with the
/// same key.
///
/// # Type Parameters
/// - `K`: The key type, which must implement [`Ord`] for ordering.
/// - `V`: The value type.
/// - `L`: The iterator type for the left input, where `Item = (K, V)`.
/// - `R`: The iterator type for the right input, where `Item = (K, V)`.
/// - `MERGED`: The merge result type. By default it is the same as `V`.
///
/// # Examples
///
/// Merge two `(key, value)` iterators by key, keeping the largest value:
/// ```
/// # use more_iter::MergePairs;
/// let a = [(1, 10), (3, 30), (4, 40)];
/// let b = [(2, 200), (3, 300)];
///
/// let merged = MergePairs::merge(a, Some(b), std::cmp::max).collect::<Vec<_>>();
/// assert_eq!(vec![(1, 10), (2, 200), (3, 300), (4, 40)], merged);
/// ```
pub struct MergePairs<K, V, L, R, MERGED = V>
where
    K: Ord,
    L: Iterator<Item = (K, V)>,
    R: Iterator<Item = (K, V)>,
{
    left: Peekable<L>,
    right: Option<Peekable<R>>,
    merge_value: fn(V, V) -> MERGED,
}

impl<K, V, L, R, MERGED> MergePairs<K, V, L, R, MERGED>
where
    K: Ord,
    L: Iterator<Item = (K, V)>,
    R: Iterator<Item = (K, V)>,
{
    /// Creates a new `Merge` instance that merges the `left` and `right` iterators.
    ///
    /// The `merge` function is used to decide which value to keep when duplicate keys are
    /// encountered.
    pub fn merge<IL, IR>(left: IL, right: Option<IR>, merge_value: fn(V, V) -> MERGED) -> Self
    where
        IL: IntoIterator<Item = (K, V), IntoIter = L>,
        IR: IntoIterator<Item = (K, V), IntoIter = R>,
    {
        MergePairs {
            left: left.into_iter().peekable(),
            right: right.map(|x| x.into_iter().peekable()),
            merge_value,
        }
    }
}

impl<K, V, L, MERGED> MergePairs<K, V, L, L, MERGED>
where
    K: Ord,
    L: Iterator<Item = (K, V)>,
{
    /// Creates a new `Merge` instance that merges the `left` and `right` iterators.
    ///
    /// The `merge` function is used to decide which value to keep when duplicate keys are
    /// encountered.
    pub fn single<IL, IR>(left: IL, merge_value: fn(V, V) -> MERGED) -> Self
    where IL: IntoIterator<Item = (K, V), IntoIter = L> {
        MergePairs {
            left: left.into_iter().peekable(),
            right: None,
            merge_value,
        }
    }
}

impl<K, V, L, R> Iterator for MergePairs<K, V, L, R>
where
    K: Ord,
    L: Iterator<Item = (K, V)>,
    R: Iterator<Item = (K, V)>,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let right = if let Some(right) = &mut self.right {
            right
        } else {
            return self.left.next();
        };

        // Left iterator is exhausted, return right iterator
        let left_key = if let Some((k, _)) = self.left.peek() {
            k
        } else {
            return right.next();
        };

        // Right iterator is exhausted, return left iterator
        let right_key = if let Some((k, _)) = right.peek() {
            k
        } else {
            return self.left.next();
        };

        // Both iterators have values, return the one with smaller key

        if left_key < right_key {
            return self.left.next();
        }

        if left_key > right_key {
            return right.next();
        }

        // Same key, merge the values

        let (k, left_value): (K, V) = self.left.next().unwrap();
        let (_, right_value): (K, V) = right.next().unwrap();

        let v = (self.merge_value)(left_value, right_value);
        Some((k, v))
    }
}
