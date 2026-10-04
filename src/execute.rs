use crate::prompts::choice_prompt;
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

    #[strum(message = "Sair")]
    Exit,
}

pub fn run() {
    loop {
        match choice_prompt::<Actions>("Qual operação você deseja realizar?") {
            Actions::Query => {
                println!("Busca requerida seção 6.1\nTalvez fazer busca exata por ID");
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
            Actions::Exit => {
                break;
            }
        }
        println!();
    }
}
