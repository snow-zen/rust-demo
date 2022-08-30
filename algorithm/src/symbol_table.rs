#[cfg(test)]
mod tests;

pub trait SymbolTable<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    fn put(&mut self, key: K, value: V);

    fn get(&self, key: K) -> Option<&V>;

    fn delete(&mut self, key: K);

    fn contains(&self, key: K) -> bool;

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;

    fn keys() -> dyn Iterator;
}

pub trait SymbolTableOrd<K, V>: SymbolTable<K, V>
where
    K: PartialOrd + PartialEq,
    V: PartialEq,
{
    fn min(&self) -> Option<&K>;

    fn max(&self) -> Option<&K>;

    fn floor(&self, key: &K) -> Option<&K>;

    fn ceiling(&self, key: &K) -> Option<&K>;

    fn rank(&self, key: &K) -> usize;

    fn select(&self, idx: usize) -> Option<&K>;

    fn delete_min(&mut self);

    fn delete_max(&mut self);

    fn range_size(&self, start: K, end: K);

    fn range_keys(&self, start: K, end: K);
}
