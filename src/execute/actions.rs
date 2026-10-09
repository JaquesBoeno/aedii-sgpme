use strum::{EnumIter, EnumMessage};

#[derive(Debug, Clone, Copy, EnumIter, EnumMessage)]
pub enum Actions {
    #[strum(message = "Consultar (Por Id)")]
    Query,

    #[strum(message = "Buscar (Por Campo)")]
    Search,

    #[strum(message = "Listar (Por Criterio)")]
    List,

    #[strum(message = "Função adicional  ")]
    AditionalFunction,

    #[strum(message = "Algoritmo Guloso ")]
    AlgGuloso,

    #[strum(message = "Metricas do HashMap")]
    Metricas,

    #[strum(message = "Sair")]
    Exit,
}
