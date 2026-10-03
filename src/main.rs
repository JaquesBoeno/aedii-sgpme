use aedii_sgpme::prompts;
use strum::{EnumIter, EnumMessage};

#[derive(Debug, Clone, Copy, EnumIter, EnumMessage)]
enum Acao {
    #[strum(message = "Criar um novo registro")]
    Create,
    #[strum(message = "Listar registros")]
    List,
    #[strum(message = "Sair")]
    Exit,
}

fn main() {
    println!("Hello World!");
    let _r: usize = prompts::prompt("Digite um numero qualquer");
    let _action: Acao = prompts::choice_prompt("Qual ação deseja fazer?");
}
