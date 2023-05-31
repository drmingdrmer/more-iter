use crate::MergePairs;

pub trait MergePairsExt<K, V>
where
    K: Ord,
    Self: Iterator<Item = (K, V)> + Sized,
{
    fn merge<MERGED, R>(self, right: Option<R>, merge_value: fn(V, V) -> MERGED) -> MergePairs<K, V, Self, R, MERGED>
    where
        Self: Sized,
        R: Iterator<Item = (K, V)>,
    {
        MergePairs::merge(self, right, merge_value)
    }
}

impl<K, V, I> MergePairsExt<K, V> for I
where
    I: Iterator<Item = (K, V)> + Sized,
    K: Ord,
{
}
