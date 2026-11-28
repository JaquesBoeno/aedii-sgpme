use std::cmp::Ordering;

/// Operações mínimas que a B-Tree deve implementar.
///
/// Ajuste em relação à Parte 1: `search` e `interval_search` passam a devolver
/// referências (`&V` / `(&K, &V)`) em vez de valores possuídos. Justificativa:
/// a consulta não deve exigir `Clone` nem mover valores da estrutura, e o
/// consumidor (`Store::interval_search`) devolve `Vec<&T>`.
pub trait RangeIndex<K: Ord, V> {
    /// Insere `value` sob `key` e devolve o valor anterior, se existia.
    fn put(&mut self, key: K, value: V) -> Option<V>;

    /// Busca exata por `key`.
    fn search(&self, key: &K) -> Option<&V>;

    /// Devolve todas as entradas com chave no intervalo fechado `[min, max]`,
    /// em ordem crescente de chave.
    fn interval_search(&self, min: &K, max: &K) -> Vec<(&K, &V)>;
}

#[derive(Debug)]
struct Node<K: Ord, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<Node<K, V>>>,
    leaf: bool,
}

impl<K: Ord, V> Node<K, V> {
    fn new(leaf: bool) -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            leaf,
        }
    }
}

/// B-Tree própria de grau mínimo `t`.
///
/// Invariantes (CLRS): todo nó (exceto a raiz) tem entre `t-1` e `2t-1` chaves;
/// a raiz tem entre 1 e `2t-1`; um nó interno com `n` chaves tem `n+1` filhos.
/// Implementação própria, sem bibliotecas prontas de estrutura de árvore B.
#[derive(Debug)]
pub struct BTree<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
    t: usize,
    len: usize,
}

impl<K: Ord, V> BTree<K, V> {
    /// Cria uma B-Tree com grau mínimo 2 (máx. 3 chaves por nó).
    pub fn new() -> Self {
        Self::with_t(2)
    }

    /// Cria uma B-Tree com o grau mínimo informado (`t >= 2`).
    pub fn with_t(t: usize) -> Self {
        assert!(t >= 2, "grau mínimo da B-Tree deve ser >= 2");
        Self {
            root: None,
            t,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl<K: Ord, V> RangeIndex<K, V> for BTree<K, V> {
    fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.root.is_none() {
            let mut n = Node::new(true);
            n.keys.push(key);
            n.values.push(value);
            self.root = Some(Box::new(n));
            self.len = 1;
            return None;
        }

        if self.root.as_ref().unwrap().keys.len() == 2 * self.t - 1 {
            self.split_root();
        }

        let old = insert_nonfull(self.root.as_mut().unwrap(), self.t, key, value);
        if old.is_none() {
            self.len += 1;
        }
        old
    }

    fn search(&self, key: &K) -> Option<&V> {
        let mut node = self.root.as_deref()?;
        loop {
            match node.keys.binary_search(key) {
                Ok(i) => return Some(&node.values[i]),
                Err(i) => {
                    if node.leaf {
                        return None;
                    }
                    node = node.children[i].as_ref();
                }
            }
        }
    }

    fn interval_search(&self, min: &K, max: &K) -> Vec<(&K, &V)> {
        if min > max {
            return Vec::new();
        }
        let mut out = Vec::new();
        if let Some(root) = self.root.as_deref() {
            collect_range(root, min, max, &mut out);
        }
        out
    }
}

impl<K: Ord, V> BTree<K, V> {
    /// Divide a raiz cheia e eleva a árvore em um nível.
    fn split_root(&mut self) {
        let mut y = self.root.take().unwrap();
        let y_is_leaf = y.leaf;

        let median_key = y.keys.remove(self.t - 1);
        let median_val = y.values.remove(self.t - 1);

        let mut z = Box::new(Node::new(y_is_leaf));
        z.keys = y.keys.split_off(self.t - 1);
        z.values = y.values.split_off(self.t - 1);
        if !y_is_leaf {
            z.children = y.children.split_off(self.t);
        }

        let mut new_root = Node::new(false);
        new_root.keys.push(median_key);
        new_root.values.push(median_val);
        new_root.children.push(y);
        new_root.children.push(z);

        self.root = Some(Box::new(new_root));
    }
}

/// Divide o i-ésimo filho (cheio) de `parent` em dois nós de `t-1` chaves,
/// promovendo a mediana para o pai.
fn split_child<K: Ord, V>(parent: &mut Node<K, V>, i: usize, t: usize) {
    let mut y = std::mem::replace(&mut parent.children[i], Box::new(Node::new(true)));
    let mut z = Box::new(Node::new(y.leaf));

    let median_key = y.keys.remove(t - 1);
    let median_val = y.values.remove(t - 1);

    z.keys = y.keys.split_off(t - 1);
    z.values = y.values.split_off(t - 1);
    if !y.leaf {
        z.children = y.children.split_off(t);
    }

    parent.keys.insert(i, median_key);
    parent.values.insert(i, median_val);
    parent.children[i] = y;
    parent.children.insert(i + 1, z);
}

/// Insere em um nó garantidamente não cheio, devolvendo o valor anterior se a
/// chave já existia.
fn insert_nonfull<K: Ord, V>(node: &mut Node<K, V>, t: usize, key: K, value: V) -> Option<V> {
    let i = node.keys.partition_point(|k| k < &key);

    if node.leaf {
        if i < node.keys.len() && node.keys[i] == key {
            return Some(std::mem::replace(&mut node.values[i], value));
        }
        node.keys.insert(i, key);
        node.values.insert(i, value);
        return None;
    }

    // nó interno: chave pode já existir no próprio nó (mediana promovida)
    if i < node.keys.len() && node.keys[i] == key {
        return Some(std::mem::replace(&mut node.values[i], value));
    }

    if node.children[i].keys.len() == 2 * t - 1 {
        split_child(node, i, t);
        // após a divisão, `node.keys[i]` é a mediana promovida
        match key.cmp(&node.keys[i]) {
            Ordering::Equal => Some(std::mem::replace(&mut node.values[i], value)),
            Ordering::Greater => insert_nonfull(node.children[i + 1].as_mut(), t, key, value),
            Ordering::Less => insert_nonfull(node.children[i].as_mut(), t, key, value),
        }
    } else {
        insert_nonfull(node.children[i].as_mut(), t, key, value)
    }
}

/// Percorre a subárvore em ordem, coletando as chaves em `[min, max]`.
fn collect_range<'a, K: Ord, V>(
    node: &'a Node<K, V>,
    min: &K,
    max: &K,
    out: &mut Vec<(&'a K, &'a V)>,
) {
    if node.leaf {
        for j in 0..node.keys.len() {
            if node.keys[j] < *min {
                continue;
            }
            if node.keys[j] > *max {
                break;
            }
            out.push((&node.keys[j], &node.values[j]));
        }
        return;
    }

    for j in 0..node.keys.len() {
        // Se a chave atual já ultrapassa `max`, ainda podemos ter valores
        // válidos no filho à esquerda (todos < keys[j]); depois paramos.
        if node.keys[j] > *max {
            collect_range(node.children[j].as_ref(), min, max, out);
            return;
        }

        collect_range(node.children[j].as_ref(), min, max, out);
        if node.keys[j] >= *min {
            out.push((&node.keys[j], &node.values[j]));
        }
    }
    // filho mais à direita
    collect_range(node.children[node.keys.len()].as_ref(), min, max, out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insere_e_busca() {
        let mut b = BTree::new();
        b.put(1, "a");
        b.put(2, "b");
        assert_eq!(b.search(&1), Some(&"a"));
        assert_eq!(b.search(&3), None);
    }

    #[test]
    fn substitui_valor_existente() {
        let mut b = BTree::new();
        b.put(1, "a");
        assert_eq!(b.put(1, "z"), Some("a"));
        assert_eq!(b.search(&1), Some(&"z"));
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn intervalo_basico() {
        let mut b = BTree::new();
        for i in 0..10 {
            b.put(i, i * 10);
        }
        let res = b.interval_search(&3, &7);
        let keys: Vec<i32> = res.iter().map(|(k, _)| **k).collect();
        assert_eq!(keys, vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn intervalo_sem_resultado() {
        let mut b = BTree::new();
        b.put(1, "a");
        assert!(b.interval_search(&5, &10).is_empty());
    }

    #[test]
    fn intervalo_invertido_vazio() {
        let mut b = BTree::new();
        b.put(1, "a");
        assert!(b.interval_search(&10, &1).is_empty());
    }

    #[test]
    fn limites_inclusivos() {
        let mut b = BTree::new();
        for i in 5..=10 {
            b.put(i, i);
        }
        let res = b.interval_search(&5, &10);
        assert_eq!(res.len(), 6);
        assert_eq!(res.first().unwrap().0, &5);
        assert_eq!(res.last().unwrap().0, &10);
    }

    #[test]
    fn muitas_insercoes_forcam_splits() {
        let mut b = BTree::new(); // t = 2
        for i in 0..1000 {
            b.put(i, i * 2);
        }
        assert_eq!(b.len(), 1000);
        for i in 0..1000 {
            assert_eq!(b.search(&i), Some(&(i * 2)));
        }
        let res = b.interval_search(&100, &199);
        assert_eq!(res.len(), 100);
        assert_eq!(res.first().unwrap().0, &100);
        assert_eq!(res.last().unwrap().0, &199);
    }

    #[test]
    fn grau_minimo_maior() {
        let mut b = BTree::with_t(4);
        for i in 0..500 {
            b.put(i, format!("v{i}"));
        }
        assert_eq!(b.len(), 500);
        assert_eq!(b.search(&250), Some(&"v250".to_string()));
    }

    #[test]
    fn arvore_vazia() {
        let b: BTree<i32, i32> = BTree::new();
        assert!(b.is_empty());
        assert_eq!(b.search(&1), None);
        assert!(b.interval_search(&0, &100).is_empty());
    }
}
