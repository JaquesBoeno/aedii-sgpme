use serde::Deserialize;

use crate::models::Recurso;

const URL_BASE: &str = "https://swapi.dev/api";

pub type Resultado<T> = Result<T, reqwest::Error>;

/// Formato que a SWAPI devolve em toda listagem e busca
#[derive(Deserialize)]
struct Pagina<T> {
    next: Option<String>,
    results: Vec<T>,
}

pub struct ClienteSwapi {
    http: reqwest::Client,
}

impl ClienteSwapi {
    pub fn novo() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    /// GET /{recurso}/{id}/
    pub async fn buscar_por_id<T: Recurso>(&self, id: u32) -> Resultado<T> {
        let url = format!("{URL_BASE}/{}/{id}/", T::CAMINHO);
        self.buscar_por_url(&url).await
    }

    // GET em uma URL qualquer
    pub async fn buscar_por_url<T: Recurso>(&self, url: &str) -> Resultado<T> {
        self.http.get(url).send().await?.error_for_status()?.json().await
    }

    // GET /{recurso}/ passando por todas as páginas
    pub async fn listar_todos<T: Recurso>(&self) -> Resultado<Vec<T>> {
        let url = format!("{URL_BASE}/{}/", T::CAMINHO);
        self.ler_paginas(self.http.get(url)).await
    }

    // GET /{recurso}/?search={texto} passando por todas as páginas
    pub async fn pesquisar_por_texto<T: Recurso>(&self, texto: &str) -> Resultado<Vec<T>> {
        let url = format!("{URL_BASE}/{}/", T::CAMINHO);
        self.ler_paginas(self.http.get(url).query(&[("search", texto)])).await
    }

    // Lê a primeira página e segue o campo `next` até ele ser null
    async fn ler_paginas<T: Recurso>(&self, mut requisicao: reqwest::RequestBuilder) -> Resultado<Vec<T>> {
        let mut itens = Vec::new();

        loop {
            let pagina: Pagina<T> = requisicao.send().await?.error_for_status()?.json().await?;
            itens.extend(pagina.results);

            match pagina.next {
                Some(url) => requisicao = self.http.get(url),
                None => break,
            }
        }

        Ok(itens)
    }
}