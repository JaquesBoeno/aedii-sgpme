use super::element_kind::ElementKinda;
use crate::models::{DataBase, Store};
use crate::prompts::{choice_prompt, prompt};
use std::fmt::Display;

pub fn run(db: &DataBase) {
    let kinda =
        choice_prompt::<ElementKinda>("Que tipo de elemento você deseja listar por intervalo?");

    match kinda {
        ElementKinda::People => {
            println!(
                "Campo usado: altura (height). Exemplo: intervalo 150 a 180 retorna Luke Skywalker (172)."
            );
            listar(&db.people)
        }
        ElementKinda::Planets => {
            println!(
                "Campo usado: diâmetro (diameter). Exemplo: intervalo 10000 a 11000 retorna Tatooine (10465)."
            );
            listar(&db.planets)
        }
        ElementKinda::Species => {
            println!(
                "Campo usado: altura média (average_height). Exemplo: intervalo 170 a 190 retorna Humanos (180)."
            );
            listar(&db.species)
        }
        ElementKinda::Vehicles => {
            println!(
                "Campo usado: comprimento (length). Exemplo: intervalo 30 a 40 retorna Sand Crawler (36.8)."
            );
            listar(&db.vehicles)
        }
        ElementKinda::Starships => {
            println!(
                "Campo usado: comprimento (length). Exemplo: intervalo 30 a 40 retorna Millennium Falcon (34.37)."
            );
            listar(&db.starships)
        }
    }
}

fn listar<T: Display>(store: &Store<T>) {
    let min = prompt::<f64>("Qual valor minimo?");
    let max = prompt::<f64>("Qual valor maximo?");

    if min > max {
        println!("O valor mínimo não pode ser maior que o máximo.");
        return;
    }

    let encontrados = store.interval_search(min, max);

    if encontrados.is_empty() {
        println!("Nenhum elemento encontrado nesse intervalo.");
        return;
    }

    for el in encontrados {
        if let Some(valor) = store.range_value(el) {
            println!("{el} | {valor}");
        }
    }
}
