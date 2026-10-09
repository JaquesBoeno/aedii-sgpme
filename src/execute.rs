use crate::{data_structs::HashMap, hash_loader::load_all_data, models};
mod menu;

struct DataBase {
    peoples: HashMap<u32, models::Pessoa>,
    planets: HashMap<u32, models::Planeta>,
    starships: HashMap<u32, models::Nave>,
    vehicles: HashMap<u32, models::Veiculo>,
    species: HashMap<u32, models::Especie>,
}

pub async fn run() {
    println!("Iniciando sistema... Carregando base de dados...");

    let tabelas = match load_all_data().await {
        Ok(t) => t,
        Err(e) => {
            println!("Erro crítico ao carregar dados da API: {}", e);
            return;
        }
    };

    println!("\nDados carregados com sucesso! Iniciando menu...\n");

    let db = DataBase {
        peoples: tabelas.pessoas,
        planets: tabelas.planetas,
        starships: tabelas.naves,
        vehicles: tabelas.veiculos,
        species: tabelas.especies,
    };

    menu::menu(&db);
}
