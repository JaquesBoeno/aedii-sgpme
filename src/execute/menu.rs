use crate::models::{DataBase, Named, Store};
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
                search(db);
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

fn query(db: &DataBase) {
    let kinda = choice_prompt::<ElementKinda>("Que tipo de elemento você deseja consultar?");
    let id = prompt::<usize>("Digite o ID desse elemento");

    match kinda {
        ElementKinda::People => consultar(&db.people, id),
        ElementKinda::Planets => consultar(&db.planets, id),
        ElementKinda::Starships => consultar(&db.starships, id),
        ElementKinda::Vehicles => consultar(&db.vehicles, id),
        ElementKinda::Species => consultar(&db.species, id),
    }
}

fn consultar<T: Display>(mapa: &Store<T>, id: usize) {
    match mapa.get_by_id(id) {
        Some(x) => println!("{x}"),
        None => println!("Elemento não encontrado!"),
    }
}

fn search(db: &DataBase) {
    let kinda = choice_prompt::<ElementKinda>("Que tipo de elemento você deseja consultar?");
    let prefix = prompt::<String>("Digite o nome desse elemento");

    match kinda {
        ElementKinda::People => pesquisar(&db.people, &prefix),
        ElementKinda::Planets => pesquisar(&db.planets, &prefix),
        ElementKinda::Starships => pesquisar(&db.starships, &prefix),
        ElementKinda::Vehicles => pesquisar(&db.vehicles, &prefix),
        ElementKinda::Species => pesquisar(&db.species, &prefix),
    }
}

fn pesquisar<T: Display + Named>(store: &Store<T>, prefix: &str) {
    for i in store.prefix_search(prefix) {
        println!("{}", i);
    }
}
