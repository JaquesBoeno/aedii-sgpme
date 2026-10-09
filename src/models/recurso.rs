use serde::de::DeserializeOwned;

/// como a api segue um padrao de url, basta mapear a struct para o caminho do recurso
pub trait Recurso: DeserializeOwned {
    const CAMINHO: &'static str;
}
