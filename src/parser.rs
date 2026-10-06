use json::JsonValue;

use crate::models::*;

fn text(v: &JsonValue, key: &str) -> String {
    v[key].as_str().unwrap_or("").to_string()
}

fn list(v: &JsonValue, key: &str) -> Vec<String> {
    v[key]
        .members()
        .filter_map(|x| x.as_str().map(String::from))
        .collect()
}

pub fn parse_results<T>(v: &JsonValue, parse: fn(&JsonValue) -> T) -> Vec<T> {
    v["results"].members().map(parse).collect()
}

// parse por tipo

pub fn parse_person(v: &JsonValue) -> Person {
    Person {
        name: text(v, "name"),
        height: text(v, "height"),
        mass: text(v, "mass"),
        hair_color: text(v, "hair_color"),
        skin_color: text(v, "skin_color"),
        eye_color: text(v, "eye_color"),
        birth_year: text(v, "birth_year"),
        gender: text(v, "gender"),
        homeworld: text(v, "homeworld"),
        species: list(v, "species"),
        vehicles: list(v, "vehicles"),
        starships: list(v, "starships"),
        url: text(v, "url"),
    }
}

pub fn parse_planet(v: &JsonValue) -> Planet {
    Planet {
        name: text(v, "name"),
        rotation_period: text(v, "rotation_period"),
        orbital_period: text(v, "orbital_period"),
        diameter: text(v, "diameter"),
        climate: text(v, "climate"),
        gravity: text(v, "gravity"),
        terrain: text(v, "terrain"),
        surface_water: text(v, "surface_water"),
        population: text(v, "population"),
        residents: list(v, "residents"),
        url: text(v, "url"),
    }
}

pub fn parse_species(v: &JsonValue) -> Species {
    Species {
        name: text(v, "name"),
        classification: text(v, "classification"),
        designation: text(v, "designation"),
        average_height: text(v, "average_height"),
        skin_colors: text(v, "skin_colors"),
        hair_colors: text(v, "hair_colors"),
        eye_colors: text(v, "eye_colors"),
        average_lifespan: text(v, "average_lifespan"),
        homeworld: v["homeworld"].as_str().map(|s| s.to_string()),
        language: text(v, "language"),
        people: list(v, "people"),
        url: text(v, "url"),
    }
}

pub fn parse_vehicle(v: &JsonValue) -> Vehicle {
    Vehicle {
        name: text(v, "name"),
        model: text(v, "model"),
        manufacturer: text(v, "manufacturer"),
        cost_in_credits: text(v, "cost_in_credits"),
        length: text(v, "length"),
        max_atmosphering_speed: text(v, "max_atmosphering_speed"),
        crew: text(v, "crew"),
        passengers: text(v, "passengers"),
        cargo_capacity: text(v, "cargo_capacity"),
        consumables: text(v, "consumables"),
        vehicle_class: text(v, "vehicle_class"),
        pilots: list(v, "pilots"),
        url: text(v, "url"),
    }
}

pub fn parse_starship(v: &JsonValue) -> Starship {
    Starship {
        name: text(v, "name"),
        model: text(v, "model"),
        manufacturer: text(v, "manufacturer"),
        cost_in_credits: text(v, "cost_in_credits"),
        length: text(v, "length"),
        max_atmosphering_speed: text(v, "max_atmosphering_speed"),
        crew: text(v, "crew"),
        passengers: text(v, "passengers"),
        cargo_capacity: text(v, "cargo_capacity"),
        consumables: text(v, "consumables"),
        hyperdrive_rating: text(v, "hyperdrive_rating"),
        mglt: text(v, "MGLT"),
        starship_class: text(v, "starship_class"),
        pilots: list(v, "pilots"),
        url: text(v, "url"),
    }
}