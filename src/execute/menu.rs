use super::actions::Actions;
use super::{additional, greedy, list, metrics, query, search};
use crate::models::DataBase;
use crate::prompts::choice_prompt;

pub fn menu(db: &DataBase) {
    loop {
        println!();

        match choice_prompt::<Actions>("Qual operação você deseja realizar?") {
            Actions::Query => query::run(db),
            Actions::Search => search::run(db),
            Actions::List => list::run(),
            Actions::AditionalFunction => additional::run(),
            Actions::AlgGuloso => greedy::run(db),
            Actions::Metricas => metrics::run(),
            Actions::Exit => break,
        }
    }
}
