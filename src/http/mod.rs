use crate::{Error, Result, http::models::{PineconeErrorResponse, IndexDescription}};

mod client;
pub use client::Client;
pub use client::Credentials;

mod index;
pub use index::*;

pub mod models;
use reqwest::{RequestBuilder, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

/// A common genric GET request to pinecones api returning a generic `T` response type or a normal
/// [Error::ReqwestError]
///
/// This function should to be called with the turbofish syntax ::<> as the type of T can not be
/// infered otherwise. `T` is the type / model that a successfull get request will return
pub(crate) async fn try_pincone_get_request<T>(client: &Client, path: impl AsRef<str>) -> Result<T> 
where
    T: DeserializeOwned
{
    let response = base_request(client, Method::GET, path)
        .send()
        .await;

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
pub(crate) async fn try_pinecone_post_request<'a, T, S>(index: &mut Index<'_>, index_url: bool, path: impl AsRef<str>, data_struct: &'a T) -> Result<S> 
where
    T: Serialize,
    S: DeserializeOwned
{
    let request;
    if index_url {
        let description: &IndexDescription;
        if let Some(desc) = index.description() {
            description = desc;
        } else {
            description = index.describe().await?;
        }
        match description.status.host {
            Some(ref url) => {
                let uri = url.to_string();
                request = index_base_request(index.client(), Method::POST, uri, path);
            },
            None => return Err(Error::URLNotAvailable)
        }
    } else {
        request = base_request(index.client(), Method::POST, path);
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
pub(crate) fn base_request(client: &Client, method: Method, path: impl AsRef<str>) -> RequestBuilder {
    client.client().request(method, format!("https://controller.{}.pinecone.io{}", client.credentials().environment(), path.as_ref()))
        .header("Api-Key", client.credentials().api_key())
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
}


pub(crate) fn index_base_request(client: &Client, method: Method, url: impl Into<String>, path: impl AsRef<str>) -> RequestBuilder {
    client.client().request(method, format!("https://{}{}", url.into(), path.as_ref()))
        .header("Api-Key", client.credentials().api_key())
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
}
