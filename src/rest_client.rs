use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::models::Recurso;

const URL_BASE: &str = "https://swapi.dev/api";

// reqwest já decodifica o JSON, então só existe um tipo de erro
pub type Resultado<T> = std::result::Result<T, reqwest::Error>;

/// Envelope que a SWAPI usa em toda listagem e busca
#[derive(Deserialize)]
struct Pagina<T> {
    #[serde(rename = "next")]
    proxima_pagina: Option<String>,
    #[serde(rename = "results")]
    resultados: Vec<T>,
}

#[derive(Default)]
pub struct ClienteSwapi {
    cliente_http: reqwest::Client, // reutilizar o mesmo client tipo um singleton
}

impl ClienteSwapi {
    pub fn novo() -> Self {
        Self::default()
    }

    /// GET /{recurso}/{id}/
    pub async fn buscar_por_id<T: Recurso>(&self, id: u32) -> Resultado<T> {
        self.buscar_por_url(&format!("{URL_BASE}/{}/{id}/", T::CAMINHO))
            .await
    }

    /// GET em uma URL qualquer
    pub async fn buscar_por_url<T: Recurso>(&self, url: &str) -> Resultado<T> {
        Self::requisitar_e_decodificar(self.cliente_http.get(url)).await
    }

    /// GET /{recurso}/ seguindo `next` até acabar as páginas
    pub async fn listar_todos<T: Recurso>(&self) -> Resultado<Vec<T>> {
        self.juntar_todas_as_paginas(self.cliente_http.get(Self::url_do_recurso::<T>()))
            .await
    }

    /// GET /{recurso}/?search={texto} (a query é codificada pelo reqwest)
    pub async fn pesquisar_por_texto<T: Recurso>(&self, texto: &str) -> Resultado<Vec<T>> {
        self.juntar_todas_as_paginas(
            self.cliente_http
                .get(Self::url_do_recurso::<T>())
                .query(&[("search", texto)]),
        )
        .await
    }

    fn url_do_recurso<T: Recurso>() -> String {
        format!("{URL_BASE}/{}/", T::CAMINHO)
    }

    /// Envia a requisição, falha se o status não for 2xx e converte o JSON para o tipo pedido
    async fn requisitar_e_decodificar<D: DeserializeOwned>(
        requisicao: reqwest::RequestBuilder,
    ) -> Resultado<D> {
        requisicao.send().await?.error_for_status()?.json().await
    }

    /// Pede a primeira página e continua pedindo a `proxima_pagina` até ela ser null
    async fn juntar_todas_as_paginas<T: Recurso>(
        &self,
        primeira_requisicao: reqwest::RequestBuilder,
    ) -> Resultado<Vec<T>> {
        let mut itens = Vec::new();
        let mut requisicao_atual = Some(primeira_requisicao);

        while let Some(requisicao) = requisicao_atual {
            let pagina: Pagina<T> = Self::requisitar_e_decodificar(requisicao).await?;
            itens.extend(pagina.resultados);
            requisicao_atual = pagina
                .proxima_pagina
                .map(|url| self.cliente_http.get(url));
        }

        Ok(itens)
    }
}