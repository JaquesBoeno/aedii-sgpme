use crate::data_structs::HashMap;
use crate::models::{Especie, Nave, Pessoa, Planeta, Veiculo};
use crate::rest_client::{ClienteSwapi, Resultado};

pub struct TabelasSwapi {
    pub pessoas: HashMap<u32, Pessoa>,
    pub planetas: HashMap<u32, Planeta>,
    pub especies: HashMap<u32, Especie>,
    pub veiculos: HashMap<u32, Veiculo>,
    pub naves: HashMap<u32, Nave>,
}

// Versão simplificada: Pega no penúltimo elemento entre as barras e tenta converter
fn extrair_id(url: &str) -> u32 {
    url.split('/')
       .nth_back(1)
       .and_then(|id| id.parse::<u32>().ok())
       .unwrap_or(0)
}

pub async fn load_all_data() -> Resultado<TabelasSwapi> {
    let cliente = ClienteSwapi::novo();

    let mut hash_pessoas: HashMap<u32, Pessoa> = HashMap::new();
    let mut hash_planetas: HashMap<u32, Planeta> = HashMap::new();
    let mut hash_especies: HashMap<u32, Especie> = HashMap::new();
    let mut hash_veiculos: HashMap<u32, Veiculo> = HashMap::new();
    let mut hash_naves: HashMap<u32, Nave> = HashMap::new();

    println!("A transferir dados da SWAPI para preencher as tabelas hash... Isto pode demorar alguns segundos.\n");

    let pessoas = cliente.listar_todos::<Pessoa>().await?;
    for pessoa in pessoas {
        hash_pessoas.put(extrair_id(&pessoa.url), pessoa);
    }
    println!(
        "Hash de Pessoas preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_pessoas.len(), hash_pessoas.collisions_qtt(), hash_pessoas.load_factor()
    );

    let planetas = cliente.listar_todos::<Planeta>().await?;
    for planeta in planetas {
        hash_planetas.put(extrair_id(&planeta.url), planeta);
    }
    println!(
        "Hash de Planetas preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_planetas.len(), hash_planetas.collisions_qtt(), hash_planetas.load_factor()
    );

    let especies = cliente.listar_todos::<Especie>().await?;
    for especie in especies {
        hash_especies.put(extrair_id(&especie.url), especie);
    }
    println!(
        "Hash de Espécies preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_especies.len(), hash_especies.collisions_qtt(), hash_especies.load_factor()
    );

    let veiculos = cliente.listar_todos::<Veiculo>().await?;
    for veiculo in veiculos {
        hash_veiculos.put(extrair_id(&veiculo.url), veiculo);
    }
    println!(
        "Hash de Veículos preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})", 
        hash_veiculos.len(), hash_veiculos.collisions_qtt(), hash_veiculos.load_factor()
    );

    let naves = cliente.listar_todos::<Nave>().await?;
    for nave in naves {
        hash_naves.put(extrair_id(&nave.url), nave);
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