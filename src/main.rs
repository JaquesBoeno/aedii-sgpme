use aedii_sgpme::rest_client::SwapiClient;

use aedii_sgpme::execute;

fn main() {
    execute::run();
}

// TESTE DO CLIENT


//reqwest = "0.13.5"#[tokio::main]
//async fn main() {
//    let client = SwapiClient::new();
//
//    println!("Buscando starship id=10 em swapi.dev...");
//    match client.get_starship(10).await {
//        Ok(nave) => println!("OK!\n{:#?}", nave),
//        Err(e) => eprintln!("Falha na chamada à SWAPI: {e}"),
//    }
//}