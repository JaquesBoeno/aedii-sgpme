use crate::missao::custo_combustivel;
use crate::models::{DataBase, Planeta};
use crate::prompts::prompt;

/// Operação adicional (6.4): combustível mínimo para chegar a um planeta.
///
/// Usa o mesmo modelo de custo do algoritmo guloso (`custo_combustivel`),
/// e a consulta final é feita direto na Tabela Hash (`get_by_id`).
pub fn run(db: &DataBase) {
    println!("\n--- COMBUSTÍVEL MÍNIMO ATÉ UM PLANETA ---");

    // Coleta (id, planeta) da Hash e ordena por id para a listagem ficar estável
    let mut planetas: Vec<(usize, &Planeta)> = db
        .planets
        .map
        .iter()
        .map(|(id, planeta)| (*id, planeta))
        .collect();
    planetas.sort_by_key(|(id, _)| *id);

    if planetas.is_empty() {
        println!("Nenhum planeta carregado na base de dados.");
        return;
    }

    println!("Planetas disponíveis:");
    for (id, planeta) in &planetas {
        match custo_combustivel(planeta) {
            Some(_) => println!("  [{:>2}] {:<22}", id, planeta.name),
            None => println!(
                "  [{:>2}] {:<22} (dados insuficientes para calcular)",
                id, planeta.name
            ),
        }
    }

    let id = prompt::<usize>("\nDigite o ID do planeta de destino:");

    match db.planets.get_by_id(id) {
        None => println!("Planeta não encontrado!"),
        Some(planeta) => match custo_combustivel(planeta) {
            Some(custo) => {
                println!("\n--- RESULTADO ---");
                println!("Destino: {}", planeta);
                println!("Combustível mínimo necessário: {:.2}", custo);
                println!(
                    "(modelo: combustível = período orbital do planeta, o mesmo custo usado no algoritmo guloso)"
                );
            }
            None => println!(
                "Não é possível calcular: '{}' tem período orbital desconhecido na API.",
                planeta.name
            ),
        },
    }
    println!("---------------------------------------\n");
}
