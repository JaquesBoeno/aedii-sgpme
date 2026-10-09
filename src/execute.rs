use crate::prompts::choice_prompt;
use crate::hash_loader::load_all_data;
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

pub async fn run() {
    // Carrega todos os dados da SWAPI para as tabelas Hash ANTES de iniciar o menu
    println!("Iniciando sistema... Carregando base de dados...");
    
    let tabelas = match load_all_data().await {
        Ok(t) => t,
        Err(e) => {
            println!("Erro crítico ao carregar dados da API: {}", e);
            return;
        }
    };
    
    println!("\nDados carregados com sucesso! Iniciando menu...\n");

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