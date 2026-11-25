use super::element_kind::ElementKinda;
use crate::models::{DataBase, Named, Store};
use crate::prompts::{choice_prompt, prompt};
use std::fmt::Display;

pub fn run(db: &DataBase) {
    let kinda = choice_prompt::<ElementKinda>("Que tipo de elemento você deseja consultar?");
    let prefix = prompt::<String>("Digite o nome desse elemento");

    match kinda {
        ElementKinda::People => pesquisar(&db.people, &prefix),
        ElementKinda::Planets => pesquisar(&db.planets, &prefix),
        ElementKinda::Starships => pesquisar(&db.starships, &prefix),
        ElementKinda::Vehicles => pesquisar(&db.vehicles, &prefix),
        ElementKinda::Species => pesquisar(&db.species, &prefix),
    }
}

fn pesquisar<T: Display + Named>(store: &Store<T>, prefix: &str) {
    for i in store.prefix_search(prefix) {
        println!("{}", i);
    }
}
