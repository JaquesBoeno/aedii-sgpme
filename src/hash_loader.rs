use crate::models::{DataBase, Especie, Nave, Pessoa, Planeta, Store, Veiculo};
use crate::rest_client::{ClienteSwapi, Resultado};

// Versão simplificada: Pega no penúltimo elemento entre as barras e tenta converter
fn extrair_id(url: &str) -> usize {
    url.split('/')
        .nth_back(1)
        .and_then(|id| id.parse::<usize>().ok())
        .unwrap_or(0)
}

fn imprimir_store<T>(nome: &str, store: &Store<T>) {
    println!(
        "Hash de {} preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})",
        nome,
        store.map.len(),
        store.map.collisions_qtt(),
        store.map.load_factor()
    );
}

pub fn imprimir_estatisticas(db: &DataBase) {
    imprimir_store("Pessoas", &db.people);
    imprimir_store("Planetas", &db.planets);
    imprimir_store("Espécies", &db.species);
    imprimir_store("Veículos", &db.vehicles);
    imprimir_store("Naves", &db.starships);
}

pub async fn load_all_data(db: &mut DataBase) -> Resultado<()> {
    let cliente = ClienteSwapi::novo();

    println!(
        "A transferir dados da SWAPI para preencher as tabelas hash... Isto pode demorar alguns segundos.\n"
    );

    let pessoas = cliente.listar_todos::<Pessoa>().await?;
    for pessoa in pessoas {
        db.people.insert(extrair_id(&pessoa.url), pessoa);
    }

    let planetas = cliente.listar_todos::<Planeta>().await?;
    for planeta in planetas {
        db.planets.insert(extrair_id(&planeta.url), planeta);
    }

    let especies = cliente.listar_todos::<Especie>().await?;
    for especie in especies {
        db.species.insert(extrair_id(&especie.url), especie);
    }

    let veiculos = cliente.listar_todos::<Veiculo>().await?;
    for veiculo in veiculos {
        db.vehicles.insert(extrair_id(&veiculo.url), veiculo);
    }

    let naves = cliente.listar_todos::<Nave>().await?;
    for nave in naves {
        db.starships.insert(extrair_id(&nave.url), nave);
    }

    imprimir_estatisticas(db);

    Ok(())
}
