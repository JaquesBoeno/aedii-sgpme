//! Testes de propriedade (proptest) das estruturas de dados.
//!
//! Cada teste gera entradas aleatórias e compara a estrutura própria com um
//! modelo de referência (`BTreeMap` da std). Se falhar, o proptest reduz a
//! entrada ao menor caso que quebra a propriedade (*shrinking*).

use std::collections::BTreeMap;

use aedii_sgpme::data_structs::btree::RangeIndex;
use aedii_sgpme::data_structs::trie::PrefixSearch;
use aedii_sgpme::data_structs::{BTree, Trie};
use proptest::prelude::*;

/// Gera chaves de teste em `[a-z]{0..6}` (inclui a string vazia).
fn chave() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::char::range('a', 'z'), 0..6)
        .prop_map(|chars| chars.into_iter().collect())
}

proptest! {
    #[test]
    fn trie_concorda_com_modelo(
        entradas in prop::collection::vec((chave(), any::<i64>()), 0..40),
        prefixo in chave(),
    ) {
        let mut trie = Trie::new();
        let mut modelo = BTreeMap::new();

        for (k, v) in &entradas {
            trie.put(k, *v);
            modelo.insert(k.clone(), *v);
        }

        prop_assert_eq!(trie.len(), modelo.len());

        // busca exata bate com o modelo
        for (k, v) in &modelo {
            prop_assert_eq!(trie.search(k), Some(v));
        }

        // prefix_search devolve exatamente as chaves do modelo que começam
        // com o prefixo, em ordem lexicográfica
        let esperado: Vec<(String, i64)> = modelo
            .iter()
            .filter(|(k, _)| k.starts_with(&prefixo))
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        let obtido: Vec<(String, i64)> = trie
            .prefix_search(&prefixo)
            .into_iter()
            .map(|(k, v)| (k, *v))
            .collect();

        prop_assert_eq!(obtido, esperado);
    }

    #[test]
    fn btree_concorda_com_modelo(
        entradas in prop::collection::vec((any::<i32>(), any::<i64>()), 0..120),
        lo in any::<i32>(),
        hi in any::<i32>(),
    ) {
        let mut btree = BTree::new();
        let mut modelo = BTreeMap::new();

        for (k, v) in &entradas {
            btree.put(*k, *v);
            modelo.insert(*k, *v);
        }

        prop_assert_eq!(btree.len(), modelo.len());

        // busca exata bate com o modelo
        for (k, v) in &modelo {
            prop_assert_eq!(btree.search(k), Some(v));
        }

        // interval_search devolve o mesmo intervalo fechado do modelo
        let (min, max) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        let esperado: Vec<(i32, i64)> =
            modelo.range(min..=max).map(|(k, v)| (*k, *v)).collect();

        let obtido: Vec<(i32, i64)> = btree
            .interval_search(&min, &max)
            .into_iter()
            .map(|(k, v)| (*k, *v))
            .collect();

        prop_assert_eq!(obtido, esperado);
    }
}
