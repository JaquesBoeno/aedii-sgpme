use crate::models::api::{EspecieApi, NaveApi, PessoaApi, PlanetaApi, VeiculoApi};
use crate::models::{DataBase, Entity, Especie, Nave, Pessoa, Planeta, Recurso, Store, Veiculo};
use crate::rest_client::{ClienteSwapi, Resultado};

fn imprimir_store<T>(nome: &str, store: &Store<T>) {
    println!(
        "Hash de {} preenchida! (Total: {} | Colisões: {} | Fator de Carga: {:.2})",
        nome,
        store.len(),
        store.collisions(),
        store.load_factor()
    );
}

pub fn imprimir_estatisticas(db: &DataBase) {
    imprimir_store("Pessoas", &db.people);
    imprimir_store("Planetas", &db.planets);
    imprimir_store("Espécies", &db.species);
    imprimir_store("Veículos", &db.vehicles);
    imprimir_store("Naves", &db.starships);
}

async fn carregar_recurso<Api, Entidade>(
    cliente: &ClienteSwapi,
    store: &mut Store<Entidade>,
) -> Resultado<()>
where
    Api: Recurso,
    Entidade: From<Api> + Entity,
{
    let itens = cliente.listar_todos::<Api>().await?;
    for item_api in itens {
        let entidade = Entidade::from(item_api);
        store.insert(entidade.id(), entidade);
    }
    Ok(())
}

pub async fn load_all_data(db: &mut DataBase) -> Resultado<()> {
    let cliente = ClienteSwapi::novo();

    println!(
        "A transferir dados da SWAPI para preencher as tabelas hash... Isto pode demorar alguns segundos.\n"
    );

    tokio::try_join!(
        carregar_recurso::<PessoaApi, Pessoa>(&cliente, &mut db.people),
        carregar_recurso::<PlanetaApi, Planeta>(&cliente, &mut db.planets),
        carregar_recurso::<EspecieApi, Especie>(&cliente, &mut db.species),
        carregar_recurso::<VeiculoApi, Veiculo>(&cliente, &mut db.vehicles),
        carregar_recurso::<NaveApi, Nave>(&cliente, &mut db.starships),
    )?;
    //
    // carregar_recurso::<PessoaApi, Pessoa>(&cliente, &mut db.people).await?;
    // carregar_recurso::<PlanetaApi, Planeta>(&cliente, &mut db.planets).await?;
    // carregar_recurso::<EspecieApi, Especie>(&cliente, &mut db.species).await?;
    // carregar_recurso::<VeiculoApi, Veiculo>(&cliente, &mut db.vehicles).await?;
    // carregar_recurso::<NaveApi, Nave>(&cliente, &mut db.starships).await?;
    //
    imprimir_estatisticas(db);

    Ok(())
}
