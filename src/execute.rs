use crate::{hash_loader::load_all_data, models::DataBase};

mod actions;
mod element_kind;
mod greedy;
mod list;
mod menu;
mod metrics;
mod min_fuel;
mod query;
mod search;

pub async fn run() {
    println!("Iniciando sistema... Carregando base de dados...");

    let mut db = DataBase::new();

    if let Err(e) = load_all_data(&mut db).await {
        println!("Erro crítico ao carregar dados da API: {}", e);
        return;
    }

    println!("\nDados carregados com sucesso! Iniciando menu...\n");

    menu::menu(&db);
}