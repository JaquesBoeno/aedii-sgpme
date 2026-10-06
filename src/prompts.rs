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
            print!("  [{}] {}\t", i, msg);

            if (i + 1) % 3 == 0 {
                println!();
            }
        }

        print!("\nDigite sua Escolha: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!();

        if let Ok(idx) = input.trim().parse::<usize>()
            && idx < variants.len()
        {
            return variants[idx];
        }

        println!("Opção inválida, tente de novo.");
    }
}