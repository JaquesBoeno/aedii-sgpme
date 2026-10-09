use strum::{EnumIter, EnumMessage};

#[derive(Debug, Clone, Copy, EnumIter, EnumMessage)]
pub enum ElementKinda {
    #[strum(message = "Pessoa")]
    People,

    #[strum(message = "Planeta")]
    Planets,

    #[strum(message = "Espaço-Nave")]
    Starships,

    #[strum(message = "Espécie")]
    Species,

    #[strum(message = "Veiculo")]
    Vehicles,
}
