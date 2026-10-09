use super::element_kind::ElementKinda;
use crate::models::{DataBase, Store, parse_numero};
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
            listar(&db.people, |p| parse_numero(&p.height))
        }
        ElementKinda::Planets => {
            println!(
                "Campo usado: diâmetro (diameter). Exemplo: intervalo 10000 a 11000 retorna Tatooine (10465)."
            );
            listar(&db.planets, |p| parse_numero(&p.diameter))
        }
        ElementKinda::Species => {
            println!(
                "Campo usado: altura média (average_height). Exemplo: intervalo 170 a 190 retorna Humanos (180)."
            );
            listar(&db.species, |e| parse_numero(&e.average_height))
        }
        ElementKinda::Vehicles => {
            println!(
                "Campo usado: comprimento (length). Exemplo: intervalo 30 a 40 retorna Sand Crawler (36.8)."
            );
            listar(&db.vehicles, |v| parse_numero(&v.length))
        }
        ElementKinda::Starships => {
            println!(
                "Campo usado: comprimento (length). Exemplo: intervalo 30 a 40 retorna Millennium Falcon (34.37)."
            );
            listar(&db.starships, |n| parse_numero(&n.length))
        }
    }
}

fn listar<T: Display, F>(store: &Store<T>, campo: F)
where
    F: Fn(&T) -> Option<f64>,
{
    let min = prompt::<f64>("Qual valor minimo?");
    let max = prompt::<f64>("Qual valor maximo?");

    if min > max {
        println!("O valor mínimo não pode ser maior que o máximo.");
        return;
    }

    let encontrados = store.interval_search(min, max, &campo);

    if encontrados.is_empty() {
        println!("Nenhum elemento encontrado nesse intervalo.");
        return;
    }

    for el in encontrados {
        if let Some(valor) = campo(el) {
            println!("{el} | {valor}");
        }
    }
}
