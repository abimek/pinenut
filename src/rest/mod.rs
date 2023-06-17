use crate::{Error, Result};

mod client;
pub use client::Client;


mod index;
pub use index::Index;

pub mod models;
use models::PineconeErrorResponse;
use reqwest::{RequestBuilder, Method, StatusCode, Response};
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

pub(crate) enum AcceptType {
    Text,
    Json
}

impl ToString for AcceptType {
    fn to_string(&self) -> String {
        match self {
            AcceptType::Text => String::from("text/plain"),
            AcceptType::Json => String::from("application/json")
        }
    }
}

async fn pinecone_request<T, C>(con: &C, method: Method, accept_type: AcceptType, index_url: Option<impl Into<String>>, path: impl AsRef<str>, data_struct: Option<&T>) -> Result<Response>
where
    C: Connection,
    T: Serialize
{
    let request;
    if let Some(url) = index_url {
        request = url_base_request(con, method.clone(), accept_type, url, path);
    } else {
        request = base_request(con, method.clone(), accept_type, path);
    }
    let response;
    match method {
        Method::DELETE => {
            response = request 
                .send()
                .await;
        },
        Method::POST | Method::PATCH => {
            let data;
            match data_struct {
                Some(ref val) => {data=val;},
                None => return Err(Error::ArgumentError {name: "data_struct".to_string(), found: "None".to_string(), expected: "a valuec".to_string()})
            }
            response = request
                .json(data)
                .send()
                .await;
        },
        Method::GET => {
            response = request 
                .send()
                .await;
        },
        method => return Err(Error::UnsupportedMethod{method})
    }
    match response {
        Ok(resp) => Ok(resp),
        Err(err) => Err(Error::ReqwestError(err))
    }
}

/// A common generic POST request to pinecones api taking in a specicific `data_struct` which
/// represents a request being sent and wraping around the possible response of
/// [`PineconeResponseModel`]
///
/// This function should be called using the turbofish syntax unless the compiler can figure out
/// the type of S. S is a model / struct that represnts a successfull post request. If this
/// function fails it will attempt to construct a [`Error::PineconeResponseError`] using the
/// [`PineconeErrorResponse`], if it fails to do so it will return a normal
/// [`Error::ReqwestError`]
pub(crate) async fn try_pinecone_request_json<C, T, S>(con: &C, method: Method, success_code: StatusCode, index_url: Option<impl Into<String>>, path: impl AsRef<str>, data_struct: Option<&T>) -> Result<S> 
where
    C: Connection,
    T: Serialize,
    S: DeserializeOwned,
{
    let resp = pinecone_request(con, method, AcceptType::Json, index_url, path, data_struct).await?;
    let code = resp.status();
    if resp.status() == success_code {
        match resp.json::<S>().await {
            Ok(success_val) => return Ok(success_val),
            Err(err) => return Err(Error::ReqwestResponseError(code, err))
        }
        
    }
    match resp.json::<PineconeErrorResponse>().await {
        Ok(f) => Err(Error::PineconeResponseError(code, Some(f), None)),
        Err(err) => Err(Error::ReqwestResponseError(code, err))
    }
}

pub(crate) async fn try_pinecone_request_text<C, T>(con: &C, method: Method, success_code: StatusCode, index_url: Option<impl Into<String>>, path: impl AsRef<str>, data_struct: Option<&T>) -> Result<String> 
where
    C: Connection,
    T: Serialize
{
    let resp = pinecone_request(con, method, AcceptType::Text, index_url, path, data_struct).await?;
    let code = resp.status();
    if resp.status() == success_code {
        match resp.text().await {
            Ok(success_val) => return Ok(success_val),
            Err(err) => return Err(Error::ReqwestResponseError(code, err))
        }
        
    }
    match resp.text().await {
        Ok(f) => Err(Error::PineconeResponseError(code,None, Some(f))),
        Err(err) => Err(Error::ReqwestResponseError(code, err))
    }
}


/// Creates a basic incomplete request to pinecone and populates it with the api key and the
/// accepted type
pub(crate) fn base_request<C, A>(con: &C, method: Method, accept_type: AcceptType, path: A) -> RequestBuilder 
where
    C: Connection,
    A: AsRef<str>
{
    con.client().request(method, format!("https://controller.{}.pinecone.io{}", con.credentials().environment, path.as_ref()))
        .header("Api-Key", &con.credentials().api_key)
        .header("accept", accept_type.to_string())
        .header("content-type", "application/json")
}


pub(crate) fn url_base_request<C, U, A>(con: &C, method: Method, accept_type: AcceptType, url: U, path: A) -> RequestBuilder 
where
    C: Connection,
    U: Into<String>,
    A: AsRef<str>
{
    con.client().request(method, format!("{}{}", url.into(), path.as_ref()))
        .header("Api-Key", &con.credentials().api_key)
        .header("accept", accept_type.to_string())
        .header("content-type", "application/json")
}
