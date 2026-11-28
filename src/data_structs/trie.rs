use std::collections::BTreeMap;

/// Operações mínimas que a Árvore Trie deve implementar.
///
/// Ajuste em relação à Parte 1: `search` e `prefix_search` passam a devolver
/// referências (`&V`) em vez de valores possuídos (`V`). Justificativa: a
/// busca não deve exigir `Clone` nem mover/remover valores da estrutura, e o
/// consumidor (`Store::prefix_search`) devolve `Vec<&T>`. As chaves retornadas
/// por `prefix_search` são reconstruídas durante a travessia, logo são `String`
/// possuídas.
pub trait PrefixSearch<V> {
    /// Insere `value` sob `key` e devolve o valor anterior, se existia.
    fn put(&mut self, key: &str, value: V) -> Option<V>;

    /// Busca exata por `key`.
    fn search(&self, key: &str) -> Option<&V>;

    /// Devolve todas as entradas cuja chave começa com `prefix`, em ordem
    /// lexicográfica (pares `(chave_completa, valor)`).
    fn prefix_search(&self, prefix: &str) -> Vec<(String, &V)>;
}

#[derive(Debug)]
struct Node<V> {
    children: BTreeMap<char, Node<V>>,
    value: Option<V>,
}

impl<V> Default for Node<V> {
    fn default() -> Self {
        Self {
            children: BTreeMap::new(),
            value: None,
        }
    }
}

/// Trie genérica que armazena `(chave: str, valor: V)`.
///
/// Os filhos de cada nó ficam em um `BTreeMap<char, _>`, o que garante ordem
/// lexicográfica determinística na busca por prefixo (a Trie em si é própria;
/// o `BTreeMap` é apenas o contêiner interno dos filhos).
#[derive(Debug)]
pub struct Trie<V> {
    root: Node<V>,
    len: usize,
}

impl<V> Default for Trie<V> {
    fn default() -> Self {
        Self {
            root: Node::default(),
            len: 0,
        }
    }
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn node(&self, key: &str) -> Option<&Node<V>> {
        let mut current = &self.root;
        for ch in key.chars() {
            current = current.children.get(&ch)?;
        }
        Some(current)
    }
}

impl<V> PrefixSearch<V> for Trie<V> {
    fn put(&mut self, key: &str, value: V) -> Option<V> {
        let mut current = &mut self.root;
        for ch in key.chars() {
            current = current.children.entry(ch).or_default();
        }

        let old = current.value.replace(value);
        if old.is_none() {
            self.len += 1;
        }
        old
    }

    fn search(&self, key: &str) -> Option<&V> {
        self.node(key).and_then(|n| n.value.as_ref())
    }

    fn prefix_search(&self, prefix: &str) -> Vec<(String, &V)> {
        let Some(start) = self.node(prefix) else {
            return Vec::new();
        };

        let mut result = Vec::new();
        let mut buffer = String::with_capacity(prefix.len() + 4);
        buffer.push_str(prefix);

        collect(start, &mut buffer, &mut result);

        result
    }
}

fn collect<'a, V>(node: &'a Node<V>, buffer: &mut String, out: &mut Vec<(String, &'a V)>) {
    if let Some(value) = &node.value {
        out.push((buffer.clone(), value));
    }
    for (ch, child) in &node.children {
        buffer.push(*ch);
        collect(child, buffer, out);
        buffer.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insere_e_busca() {
        let mut t = Trie::new();
        assert_eq!(t.put("tatooine", 1), None);
        assert_eq!(t.search("tatooine"), Some(&1));
        assert_eq!(t.search("tatoo"), None); // prefixo, mas não é chave
        assert_eq!(t.search("x"), None);
        assert!(t.is_empty() == false);
    }

    #[test]
    fn substitui_valor_existente() {
        let mut t = Trie::new();
        t.put("a", 1);
        assert_eq!(t.put("a", 2), Some(1));
        assert_eq!(t.search("a"), Some(&2));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn prefix_search_em_ordem_lexicografica() {
        let mut t = Trie::new();
        t.put("tatooine", 1);
        t.put("tatoo", 2);
        t.put("tat", 3);
        t.put("alderaan", 4);

        let res = t.prefix_search("tat");
        let keys: Vec<&str> = res.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["tat", "tatoo", "tatooine"]);
    }

    #[test]
    fn prefix_search_inclui_a_propria_chave() {
        let mut t = Trie::new();
        t.put("tat", 3);
        t.put("tatoo", 2);

        let res = t.prefix_search("tat");
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn prefix_search_sem_resultado() {
        let mut t = Trie::new();
        t.put("alderaan", 1);
        assert!(t.prefix_search("zzz").is_empty());
    }

    #[test]
    fn prefix_vazio_retorna_tudo() {
        let mut t = Trie::new();
        t.put("a", 1);
        t.put("b", 2);
        assert_eq!(t.prefix_search("").len(), 2);
    }

    #[test]
    fn chave_vazia() {
        let mut t = Trie::new();
        t.put("", 42);
        assert_eq!(t.search(""), Some(&42));
        assert_eq!(t.prefix_search("").len(), 1);
    }
}
