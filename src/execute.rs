use crate::{data_structs::HashMap, models};
mod menu;
struct DataBase {
    peoples: HashMap<usize, models::Pessoa>,
    planets: HashMap<usize, models::Planeta>,
    starships: HashMap<usize, models::Nave>,
    vehicles: HashMap<usize, models::Veiculo>,
    species: HashMap<usize, models::Especie>,
}

pub fn run() {
    // Chamar Função para carregar dados da API aqui
    let DataBase {
        peoples,
        planets,
        starships,
        vehicles,
        species,
    };
    menu::menu(&mut DataBase);
}
