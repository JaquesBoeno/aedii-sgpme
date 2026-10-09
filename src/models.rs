mod especie;
mod nave;
mod pessoa;
mod planeta;
mod recurso;
mod veiculo;

pub use especie::Especie;
pub use nave::Nave;
pub use pessoa::Pessoa;
pub use planeta::Planeta;
pub use recurso::{Named, Recurso};
pub use veiculo::Veiculo;

use crate::data_structs::HashMap;

pub struct Store<T> {
    pub map: HashMap<usize, T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: usize, value: T) {
        self.map.put(key, value);
    }

    pub fn get_by_id(&self, key: usize) -> Option<&T> {
        self.map.get(&key)
    }

    pub fn interval_search<F>(&self, min: f64, max: f64, field: F) -> Vec<&T>
    where
        F: Fn(&T) -> Option<f64>,
    {
        let mut vec: Vec<(f64, &T)> = self
            .map
            .iter()
            .filter_map(|(_, el)| field(el).map(|valor| (valor, el)))
            .filter(|(valor, _)| *valor >= min && *valor <= max)
            .collect();

        vec.sort_by(|a, b| a.0.total_cmp(&b.0));

        vec.into_iter().map(|(_, el)| el).collect()
    }
}

impl<T: Named> Store<T> {
    pub fn prefix_search(&self, prefix: &str) -> Vec<&T> {
        let mut resultado = Vec::new();
        for (_, el) in self.map.iter() {
            if el.name().to_lowercase().starts_with(&prefix.to_lowercase()) {
                resultado.push(el);
            }
        }
        resultado
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
            people: Store::new(),
            planets: Store::new(),
            species: Store::new(),
            vehicles: Store::new(),
            starships: Store::new(),
        }
    }
}
pub fn parse_numero(texto: &str) -> Option<f64> {
    let limpo = texto.trim().replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    primeiro.parse::<f64>().ok()
}
