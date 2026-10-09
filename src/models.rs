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
pub use recurso::Recurso;
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
