
use std::error::Error;

use json::JsonValue;

use crate::models::*;
use crate::parser::*;

// aparentemente pra nao travar tudo precisa desse aglomerador de erros
type Resultado<T> = Result<T, Box<dyn Error>>;

pub struct SwapiClient {
    http: reqwest::Client, // reutilizar o mesmo client mantém conexões abertas
    base_url: String,
}

impl SwapiClient {
    pub fn new() -> Self {
        SwapiClient {
            http: reqwest::Client::new(),
            base_url: "https://swapi.dev/api".to_string(),
        }
    }

    // GET em uma URL e devolve o JSON já parseado
    async fn get_json(&self, url: &str) -> Resultado<JsonValue> {
        let body = self
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()? // transforma 404/500 em erro generico
            .text()
            .await?;
        Ok(json::parse(&body)?)
    }

    // GET /{recurso}/{id}/
    async fn buscar_um<T>(&self, recurso: &str, id: u32, parse: fn(&JsonValue) -> T) -> Resultado<T> {
        let url = format!("{}/{}/{}/", self.base_url, recurso, id);
        let json = self.get_json(&url).await?;
        Ok(parse(&json))
    }

    // GET /{recurso}/?page={n}
    async fn buscar_pagina<T>(&self, recurso: &str, pagina: u32, parse: fn(&JsonValue) -> T) -> Resultado<Vec<T>> {
        let url = format!("{}/{}/?page={}", self.base_url, recurso, pagina);
        let json = self.get_json(&url).await?;
        Ok(parse_results(&json, parse))
    }

    // GET /{recurso}/?search={texto}
    async fn buscar_texto<T>(&self, recurso: &str, texto: &str, parse: fn(&JsonValue) -> T) -> Resultado<Vec<T>> {
        // reqwest cuida de codificar a query (espaços, acentos...)
        let url = format!("{}/{}/", self.base_url, recurso);
        let body = self
            .http
            .get(&url)
            .query(&[("search", texto)])
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let json = json::parse(&body)?;
        Ok(parse_results(&json, parse))
    }

    // pessoas

    pub async fn get_person(&self, id: u32) -> Resultado<Person> {
        self.buscar_um("people", id, parse_person).await
    }

    pub async fn list_people(&self, pagina: u32) -> Resultado<Vec<Person>> {
        self.buscar_pagina("people", pagina, parse_person).await
    }

    pub async fn search_people(&self, texto: &str) -> Resultado<Vec<Person>> {
        self.buscar_texto("people", texto, parse_person).await
    }

    // planetas

    pub async fn get_planet(&self, id: u32) -> Resultado<Planet> {
        self.buscar_um("planets", id, parse_planet).await
    }

    pub async fn list_planets(&self, pagina: u32) -> Resultado<Vec<Planet>> {
        self.buscar_pagina("planets", pagina, parse_planet).await
    }

    pub async fn search_planets(&self, texto: &str) -> Resultado<Vec<Planet>> {
        self.buscar_texto("planets", texto, parse_planet).await
    }

    // especies

    pub async fn get_species(&self, id: u32) -> Resultado<Species> {
        self.buscar_um("species", id, parse_species).await
    }

    pub async fn list_species(&self, pagina: u32) -> Resultado<Vec<Species>> {
        self.buscar_pagina("species", pagina, parse_species).await
    }

    // naves

    pub async fn get_starship(&self, id: u32) -> Resultado<Starship> {
        self.buscar_um("starships", id, parse_starship).await
    }

    pub async fn list_starships(&self, pagina: u32) -> Resultado<Vec<Starship>> {
        self.buscar_pagina("starships", pagina, parse_starship).await
    }

    pub async fn get_person_by_url(&self, url: &str) -> Resultado<Person> {
        let json = self.get_json(url).await?;
        Ok(parse_person(&json))
    }

    pub async fn get_planet_by_url(&self, url: &str) -> Resultado<Planet> {
        let json = self.get_json(url).await?;
        Ok(parse_planet(&json))
    }

    pub async fn get_starship_by_url(&self, url: &str) -> Resultado<Starship> {
        let json = self.get_json(url).await?;
        Ok(parse_starship(&json))
    }
}