pub mod api;
mod especie;
mod nave;
mod pessoa;
mod planeta;
mod recurso;
mod veiculo;

pub use especie::{ClassificacaoEspecie, DesignacaoEspecie, Especie};
pub use nave::Nave;
pub use pessoa::{Genero, Pessoa};
pub use planeta::Planeta;
pub use recurso::{Named, Recurso};
pub use veiculo::Veiculo;

use crate::data_structs::HashMap;

pub struct Store<T> {
    pub map: HashMap<usize, T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: usize, value: T) {
        self.map.put(key, value);
    }

    pub fn get_by_id(&self, key: usize) -> Option<&T> {
        self.map.get(&key)
    }

    pub fn interval_search<F>(&self, min: f64, max: f64, field: F) -> Vec<&T>
    where
        F: Fn(&T) -> Option<f64>,
    {
        let mut vec: Vec<(f64, &T)> = self
            .map
            .iter()
            .filter_map(|(_, el)| field(el).map(|valor| (valor, el)))
            .filter(|(valor, _)| *valor >= min && *valor <= max)
            .collect();

        vec.sort_by(|a, b| a.0.total_cmp(&b.0));

        vec.into_iter().map(|(_, el)| el).collect()
    }
}

impl<T: Named> Store<T> {
    pub fn prefix_search(&self, prefix: &str) -> Vec<&T> {
        let mut resultado = Vec::new();
        for (_, el) in self.map.iter() {
            if el.name().to_lowercase().starts_with(&prefix.to_lowercase()) {
                resultado.push(el);
            }
        }
        resultado
    }
}

pub struct DataBase {
    pub people: Store<Pessoa>,
    pub planets: Store<Planeta>,
    pub species: Store<Especie>,
    pub vehicles: Store<Veiculo>,
    pub starships: Store<Nave>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            people: Store::new(),
            planets: Store::new(),
            species: Store::new(),
            vehicles: Store::new(),
            starships: Store::new(),
        }
    }
}
pub fn parse_numero(texto: &str) -> Option<f64> {
    let limpo = texto.trim().replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    primeiro.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn parse_u64(texto: &str) -> Option<u64> {
    let limpo = texto.trim().replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    primeiro.parse::<u64>().ok()
}

pub fn parse_u32(texto: &str) -> Option<u32> {
    let limpo = texto.trim().replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    primeiro.parse::<u32>().ok()
}

pub fn parse_texto_opcional(texto: &str) -> Option<String> {
    let limpo = texto.trim();
    match limpo.to_lowercase().as_str() {
        "" | "unknown" | "n/a" | "none" | "indefinite" => None,
        _ => Some(limpo.to_string()),
    }
}

pub fn parse_lista(texto: &str) -> Vec<String> {
    texto
        .split(',')
        .filter_map(parse_texto_opcional)
        .collect()
}

pub fn extrair_id(url: &str) -> Option<usize> {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .and_then(|id| id.parse::<usize>().ok())
}

pub fn extrair_ids(urls: &[String]) -> Vec<usize> {
    urls.iter().filter_map(|u| extrair_id(u)).collect()
}

#[cfg(test)]
mod tests {
    use super::api::{EspecieApi, NaveApi, PessoaApi, PlanetaApi, VeiculoApi};
    use super::*;

    #[test]
    fn test_pessoa_from_api() {
        let api = PessoaApi {
            name: "Luke Skywalker".into(),
            height: "172".into(),
            mass: "77".into(),
            hair_color: "blond".into(),
            skin_color: "fair".into(),
            eye_color: "blue".into(),
            birth_year: "19BBY".into(),
            gender: "male".into(),
            homeworld: "https://swapi.dev/api/planets/1/".into(),
            films: vec!["https://swapi.dev/api/films/1/".into()],
            species: vec![],
            vehicles: vec!["https://swapi.dev/api/vehicles/14/".into()],
            starships: vec!["https://swapi.dev/api/starships/12/".into()],
            url: "https://swapi.dev/api/people/1/".into(),
        };

        let pessoa = Pessoa::from(api);
        assert_eq!(pessoa.id, 1);
        assert_eq!(pessoa.height, Some(172.0));
        assert_eq!(pessoa.mass, Some(77.0));
        assert_eq!(pessoa.gender, Genero::Masculino);
        assert_eq!(pessoa.homeworld_id, Some(1));
        assert_eq!(pessoa.film_ids, vec![1]);
        assert_eq!(pessoa.vehicle_ids, vec![14]);
        assert_eq!(pessoa.starship_ids, vec![12]);
    }

    #[test]
    fn test_planeta_from_api_com_campos_desconhecidos() {
        let api = PlanetaApi {
            name: "Tatooine".into(),
            rotation_period: "23".into(),
            orbital_period: "304".into(),
            diameter: "10,465".into(),
            climate: "arid, hot".into(),
            gravity: "1 standard".into(),
            terrain: "desert".into(),
            surface_water: "1".into(),
            population: "200,000".into(),
            residents: vec!["https://swapi.dev/api/people/1/".into()],
            films: vec![],
            url: "https://swapi.dev/api/planets/1/".into(),
        };

        let planeta = Planeta::from(api);
        assert_eq!(planeta.id, 1);
        assert_eq!(planeta.diameter, Some(10465.0));
        assert_eq!(planeta.gravity, Some(1.0));
        assert_eq!(planeta.population, Some(200_000));
        assert_eq!(planeta.climate, vec!["arid", "hot"]);
        assert_eq!(planeta.resident_ids, vec![1]);
    }

    #[test]
    fn test_especie_veiculo_nave_from_api() {
        let especie = Especie::from(EspecieApi {
            name: "Human".into(),
            classification: "mammal".into(),
            designation: "sentient".into(),
            average_height: "180".into(),
            skin_colors: "caucasian, black".into(),
            hair_colors: "blonde, brown".into(),
            eye_colors: "brown, blue".into(),
            average_lifespan: "120".into(),
            homeworld: None,
            language: "Galactic Basic".into(),
            people: vec![],
            films: vec![],
            url: "https://swapi.dev/api/species/1/".into(),
        });
        assert_eq!(especie.classification, ClassificacaoEspecie::Mamifero);
        assert_eq!(especie.designation, DesignacaoEspecie::Senciente);
        assert_eq!(especie.average_height, Some(180.0));
        assert_eq!(especie.homeworld_id, None);

        let veiculo = Veiculo::from(VeiculoApi {
            name: "Sand Crawler".into(),
            model: "Digger Crawler".into(),
            manufacturer: "Corellia Mining Corporation".into(),
            cost_in_credits: "150000".into(),
            length: "36.8 ".into(),
            max_atmosphering_speed: "30".into(),
            crew: "46".into(),
            passengers: "30".into(),
            cargo_capacity: "50000".into(),
            consumables: "2 months".into(),
            vehicle_class: "wheeled".into(),
            pilots: vec![],
            films: vec![],
            url: "https://swapi.dev/api/vehicles/4/".into(),
        });
        assert_eq!(veiculo.id, 4);
        assert_eq!(veiculo.length, Some(36.8));
        assert_eq!(veiculo.crew, Some(46));

        let nave = Nave::from(NaveApi {
            name: "Millennium Falcon".into(),
            model: "YT-1300".into(),
            manufacturer: "Corellian".into(),
            cost_in_credits: "100000".into(),
            length: "34.37".into(),
            max_atmosphering_speed: "1050".into(),
            crew: "4".into(),
            passengers: "6".into(),
            cargo_capacity: "100000".into(),
            consumables: "2 months".into(),
            hyperdrive_rating: "0.5".into(),
            mglt: "75".into(),
            starship_class: "Light freighter".into(),
            pilots: vec![],
            films: vec![],
            url: "https://swapi.dev/api/starships/10/".into(),
        });
        assert_eq!(nave.id, 10);
        assert_eq!(nave.hyperdrive_rating, Some(0.5));
        assert_eq!(nave.mglt, Some(75));
    }
}
