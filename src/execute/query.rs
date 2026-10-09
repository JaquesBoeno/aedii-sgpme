use super::element_kind::ElementKinda;
use crate::models::{DataBase, Store};
use crate::prompts::{choice_prompt, prompt};
use std::fmt::Display;

pub fn run(db: &DataBase) {
    let kinda = choice_prompt::<ElementKinda>("Que tipo de elemento você deseja consultar?");
    let id = prompt::<usize>("Digite o ID desse elemento");

    match kinda {
        ElementKinda::People => consultar(&db.people, id),
        ElementKinda::Planets => consultar(&db.planets, id),
        ElementKinda::Starships => consultar(&db.starships, id),
        ElementKinda::Vehicles => consultar(&db.vehicles, id),
        ElementKinda::Species => consultar(&db.species, id),
    }
}

fn consultar<T: Display>(mapa: &Store<T>, id: usize) {
    match mapa.get_by_id(id) {
        Some(x) => println!("{x}"),
        None => println!("Elemento não encontrado!"),
    }
}
