use crate::{Error, Result, http::models::PineconeErrorResponse};

mod client;
pub use client::Client;


mod index;
pub use index::Index;

pub mod models;
use reqwest::{RequestBuilder, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

/// An abstraction around [`Index`] and [`Client`] created make them work alongside eachother in
/// [`try_pinecone_get_request`]
pub(crate) trait Connection {
    fn client(&self) -> &reqwest::Client;
    fn credentials(&self) -> &Credentials;
}

/// Holds the private credentials for a basic pinecone connection. 
#[derive(Clone)]
pub(crate) struct Credentials {
    pub(crate) api_key: String,
    pub(crate) environment: String
}

/// A common genric GET request to pinecones api returning a generic `T` response type or a normal
/// [Error::ReqwestError]
///
/// This function should to be called with the turbofish syntax ::<> as the type of T can not be
/// infered otherwise. `T` is the type / model that a successfull get request will return
pub(crate) async fn try_pinecone_get_request<C, T>(con: &C, path: impl AsRef<str>, url: Option<impl Into<String>>) -> Result<T> 
where
    C: Connection,
    T: DeserializeOwned,
{
    let request;
    if let Some(url) = url {
        request = url_base_request(con, Method::GET, url, path)
    } else {
        request = base_request(con, Method::GET, path)
    }
    let response = request.send().await;

    match response {
        Ok(resp) => {
            let p_list = resp.json::<T>().await;
            match p_list {
                Ok(list) => Ok(list),
                Err(error) => Err(Error::ReqwestError(error))
            }
        },
        Err(err) => {
            Err(Error::ReqwestError(err))
        }
    }
}

/// A common generic POST request to pinecones api taking in a specicific `data_struct` and
/// wraping around the possible response of [`PineconeResponseModel`]
///
/// This function should be called using the turbofish syntax unless the compiler can figure out
/// the type of S. S is a model / struct that represnts a successfull post request. If this
/// function fails it will attempt to construct a [`Error::PineconeResponseError`] using the
/// [`PineconeErrorResponse`], if it fails to do so it will return a normal
/// [`Error::ReqwestError`]
pub(crate) async fn try_pinecone_post_request<T, S>(index: &mut Index, index_url: bool, path: impl AsRef<str>, data_struct: &T) -> Result<S> 
where
    T: Serialize,
    S: DeserializeOwned
{
    let request;
    if index_url {
        index.cached_then_normal_describe().await?;
        if let Some(url) = index.url() {
            request = url_base_request(index, Method::POST, url, path);
        } else {
            return Err(Error::URLNotAvailable);
        }
    } else {
        request = base_request(index, Method::POST, path);
    }

    let response = request.json(data_struct)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let code = resp.status();
            if resp.status() == StatusCode::OK {
                match resp.json::<S>().await {
                    Ok(success_val) => return Ok(success_val),

                    Err(err) => return Err(Error::ReqwestResponseError(code, err))
                }
            }
            match resp.json::<PineconeErrorResponse>().await {
                Ok(f) => Err(Error::PineconeResponseError(f)),
                Err(err) => Err(Error::ReqwestResponseError(code, err))
            }
        },
        Err(err) => Err(Error::ReqwestError(err))
    }
}


/// Creates a basic incomplete request to pinecone and populates it with the api key and the
/// accepted type
pub(crate) fn base_request<C, A>(con: &C, method: Method, path: A) -> RequestBuilder 
where
    C: Connection,
    A: AsRef<str>
{
    con.client().request(method, format!("https://controller.{}.pinecone.io{}", con.credentials().environment, path.as_ref()))
        .header("Api-Key", &con.credentials().api_key)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
}


pub(crate) fn url_base_request<C, U, A>(con: &C, method: Method, url: U, path: A) -> RequestBuilder 
where
    C: Connection,
    U: Into<String>,
    A: AsRef<str>
{
    con.client().request(method, format!("https://{}{}", url.into(), path.as_ref()))
        .header("Api-Key", &con.credentials().api_key)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
}
