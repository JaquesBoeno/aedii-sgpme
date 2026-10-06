use aedii_sgpme::execute;

use aedii_sgpme::models::{Filme, Nave, Pessoa, Planeta};
use aedii_sgpme::rest_client::{ClienteSwapi, Resultado};

fn main() {
    execute::run();
}


// TESTE DAS FUNCOES DO CLIENT

/*
#[tokio::main]
async fn main() -> Resultado<()> {
    let cliente = ClienteSwapi::novo();

    let luke = cliente.buscar_por_id::<Pessoa>(1).await?;
    println!("{:#?}\n", luke);

    let nave = cliente.buscar_por_id::<Nave>(10).await?;
    println!("{:#?}\n", nave);

    let planeta = cliente.buscar_por_url::<Planeta>(&luke.homeworld).await?;
    println!("{:#?}\n", planeta);

    let filmes = cliente.listar_todos::<Filme>().await?;
    for filme in &filmes {
        println!("Episódio {}: {}", filme.episode_id, filme.title);
    }
    println!("Total de filmes: {}\n", filmes.len());

    let pessoas = cliente.listar_todos::<Pessoa>().await?;
    println!("Total de pessoas: {}\n", pessoas.len());

    let skywalkers = cliente.pesquisar_por_texto::<Pessoa>("skywalker").await?;
    for pessoa in &skywalkers {
        println!("{} ({})", pessoa.name, pessoa.birth_year);
    }

    let vazio = cliente.pesquisar_por_texto::<Pessoa>("zzzzz").await?;
    println!("Resultados para zzzzz: {}", vazio.len());

    Ok(())
}*/