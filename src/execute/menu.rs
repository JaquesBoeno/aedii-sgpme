use super::DataBase;
use crate::data_structs::HashMap;
use crate::prompts::{choice_prompt, prompt};
use std::fmt::Display;
use strum::{EnumIter, EnumMessage};

#[derive(Debug, Clone, Copy, EnumIter, EnumMessage)]
enum Actions {
    #[strum(message = "Consultar (Por Id)")]
    Query,

    #[strum(message = "Buscar (Por Campo)")]
    Search,

    #[strum(message = "Listar (Por Criterio)")]
    List,

    #[strum(message = "Função adicional  ")]
    AditionalFunction,

    #[strum(message = "Algoritimo Guloso ")]
    AlgGuloso,

    #[strum(message = "Metricas do HashMap")]
    Metricas,

    #[strum(message = "Sair")]
    Exit,
}

#[derive(Debug, Clone, Copy, EnumIter, EnumMessage)]
enum ElementKinda {
    #[strum(message = "Pessoa")]
    People,

    #[strum(message = "Planeta")]
    Planets,

    #[strum(message = "Espaço-Nave")]
    Starships,

    #[strum(message = "Espécie")]
    Species,

    #[strum(message = "Veiculo")]
    Vehicles,
}

pub fn menu(db: &DataBase) {
    loop {
        println!();

        match choice_prompt::<Actions>("Qual operação você deseja realizar?") {
            Actions::Query => {
                query(db);
            }
            Actions::Search => {
                println!(
                    "Procurar requerida seção 6.2\nBusca lexicografica, futuramente usando trie"
                );
            }
            Actions::List => {
                println!(
                    "Listagem requerida seção 6.3\nListar todos elementos que batam com alguma caracteristica"
                );
            }
            Actions::AditionalFunction => {
                println!("Função Adicional requerida seção 6.4\nAinda a se decidir");
            }
            Actions::AlgGuloso => {
                println!("Algoritimo Guloso requerido seção 7")
            }
            Actions::Metricas => {
                println!("Exibir metricas do hashmap");
            }
            Actions::Exit => {
                break;
            }
        }
    }
}

fn consultar<T: Display>(mapa: &HashMap<u32, T>, id: u32) {
    match mapa.get(&id) {
        Some(x) => println!("{x}"),
        None => println!("Elemento não encontrado!"),
    }
}

fn query(db: &DataBase) {
    let kinda = choice_prompt::<ElementKinda>("Que tipo de elemento você deseja consultar?");
    let id = prompt::<u32>("Digite o ID desse elemento");

    match kinda {
        ElementKinda::People => consultar(&db.peoples, id),
        ElementKinda::Planets => consultar(&db.planets, id),
        ElementKinda::Starships => consultar(&db.starships, id),
        ElementKinda::Vehicles => consultar(&db.vehicles, id),
        ElementKinda::Species => consultar(&db.species, id),
    }
}
