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

    #[strum(message = "Função adicional (I6.4)")]
    AditionalFunction,

    #[strum(message = "Sair")]
    Exit,
}

pub fn run() {
    loop {
        match choice_prompt::<Actions>("Qual operação você deseja realizar?") {
            Actions::Query => {
                todo!("Query");
            }
            Actions::Search => {
                todo!("Search");
            }
            Actions::List => {
                todo!("List");
            }
            Actions::AditionalFunction => {
                todo!("Função Adicional");
            }
            Actions::Exit => {
                break;
            }
        }
        // println!();
    }
}
