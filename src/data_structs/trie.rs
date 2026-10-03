/// A funções que a Arvore Trie deve implementar são estas:
pub trait PrefixSearch<V> {
    fn put(&mut self, key: &str, value: V) -> Option<V>;

    fn prefix_search(&self, prefix: &str) -> Vec<(&str, V)>;

    fn search(&self, key: &str) -> Option<V>;
}
