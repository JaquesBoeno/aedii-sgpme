use crate::models::{DataBase, Planeta};
use crate::prompts::prompt;

pub struct PlanetaAvaliado {
    pub nome: String,
    pub custo: f64,      // Período Orbital
    pub beneficio: f64,  // Diâmetro
    pub razao: f64,      // Benefício / Custo
}

/// Combustível necessário para chegar a um planeta (None se o dado for desconhecido).
/// Usado pelo algoritmo guloso e pela operação adicional, para os dois serem consistentes.
pub fn custo_combustivel(planeta: &Planeta) -> Option<f64> {
    planeta
        .orbital_period
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite() && *v > 0.0)
}

pub fn planejar_missao_extracao(db: &DataBase) {
    println!("\n--- ALGORITMO GULOSO: PLANEAMENTO DE MISSÃO ---");
    let combustivel_maximo: f64 = prompt("Digite a capacidade máxima de combustível da nave (ex: 15000):");

    let mut candidatos: Vec<PlanetaAvaliado> = Vec::new();

    // Extrai os planetas da Hash customizada
    for (_, planeta) in db.planets.map.iter() {
        let Some(custo) = custo_combustivel(planeta) else {
            continue; // desconhecido ou <= 0: evita divisão por zero
        };

        let beneficio = match planeta.diameter.parse::<f64>() {
            Ok(val) => val,
            _ => continue,
        };

        candidatos.push(PlanetaAvaliado {
            nome: planeta.name.clone(),
            custo,
            beneficio,
            razao: beneficio / custo,
        });
    }

    // Ordenação Otimizada: Estritamente decrescente baseada na heurística (Razão)
    candidatos.sort_by(|a, b| b.razao.partial_cmp(&a.razao).unwrap());

    let mut plano_de_voo: Vec<&PlanetaAvaliado> = Vec::new();
    let mut combustivel_restante = combustivel_maximo;
    let mut beneficio_acumulado = 0.0;

    // Triagem (Greedy Choice)
    for candidato in &candidatos {
        if candidato.custo <= combustivel_restante {
            plano_de_voo.push(candidato);
            combustivel_restante -= candidato.custo;
            beneficio_acumulado += candidato.beneficio;
        }
    }

    // Relatório Final
    println!("\n--- RESULTADO DA MISSÃO ---");
    println!("Combustível Inicial: {:.2}", combustivel_maximo);
    println!("Benefício Total Acumulado (Área): {:.2}", beneficio_acumulado);
    println!("Combustível Ocioso (Sobrando): {:.2}", combustivel_restante);

    if plano_de_voo.is_empty() {
        println!("Nenhum planeta coube no orçamento estipulado.");
    } else {
        println!("Destinos escolhidos (Ordem de Viagem):");
        for (i, p) in plano_de_voo.iter().enumerate() {
            println!(
                "  [{}] {} | Custo: {:.2} | Benefício: {:.2} | Eficiência: {:.2}",
                i + 1, p.nome, p.custo, p.beneficio, p.razao
            );
        }
    }
    println!("---------------------------------------\n");

    println!("\n--- ENTENDENDO OS RESULTADOS ---");
    println!("* Custo (Combustível): Derivado do 'Período Orbital' (orbital_period) do planeta na API.");
    println!("  -> Representa o tempo e esforço para estabilizar a nave e realizar a extração.");
    println!("* Benefício (Área): Derivado do 'Diâmetro' (diameter) do planeta na API.");
    println!("  -> Representa a superfície total explorável para mineração de recursos.");
    println!("* Eficiência (Razão): Calculada por [Benefício / Custo].");
    println!("  -> Diz-nos quanta área explorável ganhamos por cada unidade de combustível gasta.");
    println!("* Estratégia Gulosa: O algoritmo ordena os planetas pela Maior Eficiência e vai escolhendo");
    println!("  o melhor candidato atual até que o tanque de combustível se esgote (Mochila 0/1).");
    println!("---------------------------------------\n");
}