use json::JsonValue;

use crate::models::*;

// campo de texto nao null
fn texto(json: &JsonValue, chave: &str) -> String {
    json[chave].as_str().unwrap_or("").to_string()
}

// campo de texto que pode ser null
fn texto_opcional(json: &JsonValue, chave: &str) -> Option<String> {
    json[chave].as_str().map(String::from)
}

// lista de urls pra outros recursos que vem em array de strings (os que levam a outras paginas)
fn lista_de_urls(json: &JsonValue, chave: &str) -> Vec<String> {
    json[chave]
        .members()
        .filter_map(|item| item.as_str().map(String::from))
        .collect()
}

pub fn converter_pagina<T>(json: &JsonValue, conversor: fn(&JsonValue) -> T) -> Vec<T> {
    json["results"].members().map(conversor).collect()
}

// parseia cada tipo (recurso) um por um

pub fn converter_pessoa(json: &JsonValue) -> Pessoa {
    Pessoa {
        name: texto(json, "name"),
        height: texto(json, "height"),
        mass: texto(json, "mass"),
        hair_color: texto(json, "hair_color"),
        skin_color: texto(json, "skin_color"),
        eye_color: texto(json, "eye_color"),
        birth_year: texto(json, "birth_year"),
        gender: texto(json, "gender"),
        homeworld: texto(json, "homeworld"),
        films: lista_de_urls(json, "films"),
        species: lista_de_urls(json, "species"),
        vehicles: lista_de_urls(json, "vehicles"),
        starships: lista_de_urls(json, "starships"),
        url: texto(json, "url"),
    }
}

pub fn converter_planeta(json: &JsonValue) -> Planeta {
    Planeta {
        name: texto(json, "name"),
        rotation_period: texto(json, "rotation_period"),
        orbital_period: texto(json, "orbital_period"),
        diameter: texto(json, "diameter"),
        climate: texto(json, "climate"),
        gravity: texto(json, "gravity"),
        terrain: texto(json, "terrain"),
        surface_water: texto(json, "surface_water"),
        population: texto(json, "population"),
        residents: lista_de_urls(json, "residents"),
        films: lista_de_urls(json, "films"),
        url: texto(json, "url"),
    }
}

pub fn converter_especie(json: &JsonValue) -> Especie {
    Especie {
        name: texto(json, "name"),
        classification: texto(json, "classification"),
        designation: texto(json, "designation"),
        average_height: texto(json, "average_height"),
        skin_colors: texto(json, "skin_colors"),
        hair_colors: texto(json, "hair_colors"),
        eye_colors: texto(json, "eye_colors"),
        average_lifespan: texto(json, "average_lifespan"),
        homeworld: texto_opcional(json, "homeworld"),
        language: texto(json, "language"),
        people: lista_de_urls(json, "people"),
        films: lista_de_urls(json, "films"),
        url: texto(json, "url"),
    }
}

pub fn converter_veiculo(json: &JsonValue) -> Veiculo {
    Veiculo {
        name: texto(json, "name"),
        model: texto(json, "model"),
        manufacturer: texto(json, "manufacturer"),
        cost_in_credits: texto(json, "cost_in_credits"),
        length: texto(json, "length"),
        max_atmosphering_speed: texto(json, "max_atmosphering_speed"),
        crew: texto(json, "crew"),
        passengers: texto(json, "passengers"),
        cargo_capacity: texto(json, "cargo_capacity"),
        consumables: texto(json, "consumables"),
        vehicle_class: texto(json, "vehicle_class"),
        pilots: lista_de_urls(json, "pilots"),
        films: lista_de_urls(json, "films"),
        url: texto(json, "url"),
    }
}

pub fn converter_nave(json: &JsonValue) -> Nave {
    Nave {
        name: texto(json, "name"),
        model: texto(json, "model"),
        manufacturer: texto(json, "manufacturer"),
        cost_in_credits: texto(json, "cost_in_credits"),
        length: texto(json, "length"),
        max_atmosphering_speed: texto(json, "max_atmosphering_speed"),
        crew: texto(json, "crew"),
        passengers: texto(json, "passengers"),
        cargo_capacity: texto(json, "cargo_capacity"),
        consumables: texto(json, "consumables"),
        hyperdrive_rating: texto(json, "hyperdrive_rating"),
        mglt: texto(json, "MGLT"), // na API o nome vem em maiúsculas
        starship_class: texto(json, "starship_class"),
        pilots: lista_de_urls(json, "pilots"),
        films: lista_de_urls(json, "films"),
        url: texto(json, "url"),
    }
}