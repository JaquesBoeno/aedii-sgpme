use std::io::{self, Write};

use strum::{EnumMessage, IntoEnumIterator};

pub fn prompt<T: std::str::FromStr>(message: &str) -> T {
    loop {
        println!("{message}");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(response) = input.trim().parse::<T>() {
            return response;
        }

        println!("Resposta mal formatada! Tente novamente.");
    }
}

pub fn choice_prompt<T: IntoEnumIterator + EnumMessage + Copy>(message: &str) -> T {
    loop {
        println!("{message}");
        let variants: Vec<T> = T::iter().collect();
        for (i, v) in variants.iter().enumerate() {
            let msg = v.get_message().unwrap_or("—");
            print!("  [{}] {}", i, msg);
        }

        print!("\nDigite sua Escolha: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(idx) = input.trim().parse::<usize>()
            && idx < variants.len()
        {
            return variants[idx];
        }

        println!("Opção inválida, tente de novo.");
    }
}

/*
 * EXEMPLO DE USO TEMPORARIO, REWELL, PLS MOVER PARA A DOCUMENTAÇÂO EM MD
 * QUANDO TU IMPLEMENTAR
 * use aedii_sgpme::prompts;
 * use strum::{EnumIter, EnumMessage};
 *
 * #[derive(Debug, Clone, Copy, EnumIter, EnumMessage)]
 * enum Acao {
 *     #[strum(message = "Criar um novo registro")]
 *     Create,
 *     #[strum(message = "Listar registros")]
 *     List,
 *     #[strum(message = "Sair")]
 *     Exit,
 * }
 *
 * // Para usar as funções você PRECISA definir o tipo de variavel com o : Tipo (: usize), caso
 * // contrario a função não sabe para qual tipo deve fazer o casting do input
 * fn main() {
 *     println!("Hello World!");
 *     let _r: usize = prompts::prompt("Digite um numero qualquer");
 *     let _action: Acao = prompts::choice_prompt("Qual ação deseja fazer?");
 * }
 *
 */
