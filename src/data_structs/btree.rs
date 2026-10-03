// Essas são as operações minimas que a Btree Deve implementar
pub trait RangeIndex<K: Ord, V> {
    fn put(&mut self, key: K, value: V) -> Option<V>;

    fn search(&self, key: &K) -> Option<V>;

    fn interval_search(&self, min: &K, max: &K) -> Vec<(K, V)>;
}
