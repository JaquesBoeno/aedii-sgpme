use crate::data_structs::HashMap;
use crate::models::{Especie, Nave, Pessoa, Planeta, Veiculo};
use crate::rest_client::{ClienteSwapi, Resultado};

pub struct TabelasSwapi {
    pub pessoas: HashMap<String, Pessoa>,
    pub planetas: HashMap<String, Planeta>,
    pub especies: HashMap<String, Especie>,
    pub veiculos: HashMap<String, Veiculo>,
    pub naves: HashMap<String, Nave>,
}

pub async fn load_all_data() -> Resultado<TabelasSwapi> {
    let cliente = ClienteSwapi::novo();

    let mut hash_pessoas: HashMap<String, Pessoa> = HashMap::new();
    let mut hash_planetas: HashMap<String, Planeta> = HashMap::new();
    let mut hash_especies: HashMap<String, Especie> = HashMap::new();
    let mut hash_veiculos: HashMap<String, Veiculo> = HashMap::new();
    let mut hash_naves: HashMap<String, Nave> = HashMap::new();

    println!("Buscando dados da SWAPI para preencher as tabelas hash... Isso pode demorar alguns segundos.\n");

    // Preenchendo Pessoas
    let pessoas = cliente.listar_todos::<Pessoa>().await?;
    for pessoa in pessoas {
        hash_pessoas.put(pessoa.url.clone(), pessoa);
    }
    println!(
        "Hash de Pessoas preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_pessoas.len(), hash_pessoas.collisions_qtt(), hash_pessoas.load_factor()
    );

    // Preenchendo Planetas
    let planetas = cliente.listar_todos::<Planeta>().await?;
    for planeta in planetas {
        hash_planetas.put(planeta.url.clone(), planeta);
    }
    println!(
        "Hash de Planetas preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_planetas.len(), hash_planetas.collisions_qtt(), hash_planetas.load_factor()
    );

    // Preenchendo Espécies
    let especies = cliente.listar_todos::<Especie>().await?;
    for especie in especies {
        hash_especies.put(especie.url.clone(), especie);
    }
    println!(
        "Hash de Espécies preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_especies.len(), hash_especies.collisions_qtt(), hash_especies.load_factor()
    );

    // Preenchendo Veículos
    let veiculos = cliente.listar_todos::<Veiculo>().await?;
    for veiculo in veiculos {
        hash_veiculos.put(veiculo.url.clone(), veiculo);
    }
    println!(
        "Hash de Veículos preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_veiculos.len(), hash_veiculos.collisions_qtt(), hash_veiculos.load_factor()
    );

    // Preenchendo Naves
    let naves = cliente.listar_todos::<Nave>().await?;
    for nave in naves {
        hash_naves.put(nave.url.clone(), nave);
    }
    println!(
        "Hash de Naves preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_naves.len(), hash_naves.collisions_qtt(), hash_naves.load_factor()
    );

    Ok(TabelasSwapi {
        pessoas: hash_pessoas,
        planetas: hash_planetas,
        especies: hash_especies,
        veiculos: hash_veiculos,
        naves: hash_naves,
    })
}