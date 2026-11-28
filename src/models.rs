pub mod api;
mod especie;
mod nave;
mod pessoa;
mod planeta;
mod recurso;
mod veiculo;

pub use especie::{ClassificacaoEspecie, DesignacaoEspecie, Especie};
pub use nave::Nave;
pub use pessoa::{Genero, Pessoa};
pub use planeta::Planeta;
pub use recurso::{Entity, Recurso};
pub use veiculo::Veiculo;

use std::cmp::Ordering;

use crate::data_structs::btree::RangeIndex;
use crate::data_structs::trie::PrefixSearch;
use crate::data_structs::{BTree, HashMap, Trie, hashmap};

/// Chave `f64` com ordenação total, para indexar valores na B-Tree.
///
/// `f64` não implementa `Ord`; `total_cmp` fornece uma ordem total determinística
/// (trata `NaN`, embora `parse_numero` já descarte valores não finitos).
#[derive(Debug, Clone, Copy)]
pub struct OrdF64(pub f64);

impl PartialEq for OrdF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for OrdF64 {}

impl PartialOrd for OrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

/// Coleção indexada de entidades de um mesmo tipo.
///
/// Mantém as três estruturas da disciplina integradas:
/// - `map` (Tabela Hash) — consulta por `id`;
/// - `names` (Trie) — busca por prefixo de nome (case-insensitive);
/// - `ranges` (B-Tree) — busca por intervalo no campo numérico canônico do tipo.
pub struct Store<T> {
    map: HashMap<usize, T>,
    names: Trie<usize>,
    ranges: BTree<OrdF64, Vec<usize>>,
    range_key: Option<fn(&T) -> Option<f64>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            names: Trie::new(),
            ranges: BTree::new(),
            range_key: None,
        }
    }

    /// Cria um `Store` já configurado com o campo numérico usado na B-Tree.
    pub fn with_range_key(range_key: fn(&T) -> Option<f64>) -> Self {
        Self {
            range_key: Some(range_key),
            ..Self::new()
        }
    }

    pub fn get_by_id(&self, key: usize) -> Option<&T> {
        self.map.get(&key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn collisions(&self) -> usize {
        self.map.collisions_qtt()
    }

    pub fn load_factor(&self) -> f64 {
        self.map.load_factor()
    }

    pub fn iter(&self) -> hashmap::Iter<'_, usize, T> {
        self.map.iter()
    }

    /// Busca por intervalo fechado `[min, max]` no campo configurado,
    /// em ordem crescente do valor. Entidades sem o campo ficam de fora.
    pub fn interval_search(&self, min: f64, max: f64) -> Vec<&T> {
        if min > max {
            return Vec::new();
        }

        let mut result = Vec::new();
        for (_, ids) in self.ranges.interval_search(&OrdF64(min), &OrdF64(max)) {
            for id in ids {
                if let Some(el) = self.map.get(id) {
                    result.push(el);
                }
            }
        }
        result
    }

    /// Valor do campo numérico canônico da entidade (usado na apresentação).
    pub fn range_value(&self, el: &T) -> Option<f64> {
        self.range_key.and_then(|field| field(el))
    }
}

impl<'a, T> IntoIterator for &'a Store<T> {
    type Item = (&'a usize, &'a T);
    type IntoIter = hashmap::Iter<'a, usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Entity> Store<T> {
    pub fn insert(&mut self, key: usize, value: T) {
        if let Some(field) = self.range_key {
            if let Some(valor) = field(&value) {
                let chave = OrdF64(valor);
                let mut ids = self.ranges.search(&chave).cloned().unwrap_or_default();
                ids.push(key);
                self.ranges.put(chave, ids);
            }
        }

        self.names.put(&value.name().to_lowercase(), key);
        self.map.put(key, value);
    }

    /// Busca por prefixo de nome (case-insensitive), em ordem lexicográfica.
    pub fn prefix_search(&self, prefix: &str) -> Vec<&T> {
        self.names
            .prefix_search(&prefix.to_lowercase())
            .into_iter()
            .filter_map(|(_, id)| self.map.get(id))
            .collect()
    }
}

pub struct DataBase {
    pub people: Store<Pessoa>,
    pub planets: Store<Planeta>,
    pub species: Store<Especie>,
    pub vehicles: Store<Veiculo>,
    pub starships: Store<Nave>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            people: Store::with_range_key(|p: &Pessoa| p.height),
            planets: Store::with_range_key(|p: &Planeta| p.diameter),
            species: Store::with_range_key(|e: &Especie| e.average_height),
            vehicles: Store::with_range_key(|v: &Veiculo| v.length),
            starships: Store::with_range_key(|n: &Nave| n.length),
        }
    }
}
pub trait NumeroSwapi: Sized {
    fn from_token(token: &str) -> Option<Self>;
}

impl NumeroSwapi for f64 {
    fn from_token(token: &str) -> Option<Self> {
        token.parse::<f64>().ok().filter(|v| v.is_finite())
    }
}

impl NumeroSwapi for u32 {
    fn from_token(token: &str) -> Option<Self> {
        token.parse().ok()
    }
}

impl NumeroSwapi for u64 {
    fn from_token(token: &str) -> Option<Self> {
        token.parse().ok()
    }
}

pub fn parse_numero<T: NumeroSwapi>(texto: &str) -> Option<T> {
    let limpo = texto.replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    T::from_token(primeiro)
}

pub fn parse_texto_opcional(texto: &str) -> Option<String> {
    let limpo = texto.trim();
    match limpo.to_lowercase().as_str() {
        "" | "unknown" | "n/a" | "none" | "indefinite" => None,
        _ => Some(limpo.to_string()),
    }
}

pub fn parse_lista(texto: &str) -> Vec<String> {
    texto.split(',').filter_map(parse_texto_opcional).collect()
}

pub fn extrair_id(url: &str) -> Option<usize> {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .and_then(|id| id.parse::<usize>().ok())
}

pub fn extrair_ids(urls: &[String]) -> Vec<usize> {
    urls.iter().filter_map(|u| extrair_id(u)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Item {
        id: usize,
        name: String,
        valor: Option<f64>,
    }

    impl Entity for Item {
        fn name(&self) -> &str {
            &self.name
        }

        fn id(&self) -> usize {
            self.id
        }
    }

    fn item(id: usize, name: &str, valor: Option<f64>) -> Item {
        Item {
            id,
            name: name.to_string(),
            valor,
        }
    }

    fn store() -> Store<Item> {
        let mut store = Store::with_range_key(|i: &Item| i.valor);
        store.insert(1, item(1, "Tatooine", Some(10465.0)));
        store.insert(2, item(2, "Tatoo", Some(10.0)));
        store.insert(3, item(3, "Alderaan", None));
        store
    }

    #[test]
    fn prefix_search_usa_a_trie_e_ignora_caixa() {
        let store = store();
        assert_eq!(store.prefix_search("tat").len(), 2);
        assert_eq!(store.prefix_search("TAT").len(), 2);
        assert_eq!(store.prefix_search("tatooine").len(), 1);
        assert!(store.prefix_search("zz").is_empty());
    }

    #[test]
    fn interval_search_usa_a_btree_e_exclui_sem_valor() {
        let store = store();
        let encontrados = store.interval_search(5.0, 20.0);
        assert_eq!(encontrados.len(), 1);
        assert_eq!(encontrados[0].id, 2);

        assert!(store.interval_search(20.0, 5.0).is_empty());
        assert!(store.interval_search(500.0, 1000.0).is_empty());
    }

    #[test]
    fn interval_search_devolve_em_ordem_crescente() {
        let store = store();
        let ids: Vec<usize> = store
            .interval_search(0.0, 20000.0)
            .iter()
            .map(|i| i.id)
            .collect();
        assert_eq!(ids, vec![2, 1]);
    }

    #[test]
    fn range_value_usa_o_campo_configurado() {
        let store = store();
        assert_eq!(store.range_value(&item(9, "x", Some(3.5))), Some(3.5));
        assert_eq!(store.range_value(&item(9, "x", None)), None);
    }
}
