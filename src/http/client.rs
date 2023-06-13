use reqwest::{StatusCode, Method};
use crate::{
    Result, 
    Index, models::{CollectionDescription, CreateCollectionRequest}, http::{try_pinecone_request_json, try_pinecone_request_text}

};

use super::{Credentials, Connection};

/// An (unauthenticated) handle to talk with Pinecone. This is where you first go when you need a
/// connection. 
pub struct Client{
    client: reqwest::Client,
    creds: Credentials 
}

impl Client {

    pub fn new<D>(api_key: D, environment: D) -> Client
    where
        D: Into<String>
    {
        Client{
            client: reqwest::Client::new(),
            creds: Credentials{
                api_key: api_key.into(),
                environment: environment.into()
            }
        }
    }

    /// Will list all the indexes associated with the given [`Credentials`]
    pub async fn list_indexes(&self) -> Result<Vec<String>> {
        try_pinecone_request_json::<Client, String, Vec<String>>(self, Method::GET, StatusCode::OK, None::<String>, "/databases", None).await
    }

    /// Will list all the collections associated with the given [`Credentialsc`].
    pub async fn list_collections(&self) -> Result<Vec<String>> {
        try_pinecone_request_json::<Client, String, Vec<String>>(self, Method::GET, StatusCode::OK, None::<String>, "/collections", None).await
    }

    pub async fn create_collection(&self, name: impl Into<String>, source_index: impl AsRef<str>) -> Result<String> {
        let request = CreateCollectionRequest{
            name: name.into(),
            source: source_index.as_ref().to_string()
        };
        try_pinecone_request_text::<Client, CreateCollectionRequest>(self, Method::POST, StatusCode::CREATED, None::<String>, "/collections", Some(&request)).await
    }

    /// Describes a specific collection returning specific information that can be referenced at
    /// the pinecone api reference (https://docs.pinecone.io/reference/describe_collection)
    pub async fn describe_collection(&self, name: impl AsRef<str>) -> Result<CollectionDescription> {
        try_pinecone_request_json::<Client, String, CollectionDescription>(self, Method::GET, StatusCode::OK, None::<String>, format!("/collections/{}", name.as_ref()), None).await
    }

    /// This will delete a collection
    pub async fn delete_collection(&self, name: impl AsRef<str>) -> Result<String> {
        try_pinecone_request_text::<Client, String>(self, Method::DELETE, StatusCode::ACCEPTED, None::<String>, format!("/collections/{}", name.as_ref()), None).await
    }


    /// Creates and returns an Index object that can be used to run index specific operations, it
    /// is the primary way you interface with the Index api.
    pub fn index(&self, name: impl Into<String>) -> Index {
        Index::new::<Self>(self, name)
    }
}

impl Connection for Client {
    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn credentials(&self) -> &Credentials {
        &self.creds
    }
}


#[cfg(test)]
mod client_test {

    use super::*;
   // use crate::http::Metric;
    use wasm_bindgen_test::*;
    use crate::Error;

    fn create_client() -> Client {
        Client::new(
            env!("PINECONE_API_KEY"),
            env!("PINECONE_ENV")
        )
    }

    #[wasm_bindgen_test]
    fn test_create_env_set() {
        Client::new(
            env!("PINECONE_API_KEY"),
            env!("PINECONE_ENV")
        );
        assert!(true)
    }

    #[wasm_bindgen_test]
    async fn test_list_indexes() {
        let client = create_client();
        match client.list_indexes().await {
            Ok(list) => {
                assert!(!list.is_empty(), "working? {:?}", list);
            },
            Err(error) => panic!("Unable to list indexes: {:?}", error)
        }
    }

    #[wasm_bindgen_test]
    async fn test_create_collection() {
        let client = create_client();
        match client.create_collection("testcollection", env!("PINECONE_INDEX_NAME")).await {
            Ok(_) => {
                assert!(true)
            },
            Err(error) => {
                match error {
                    Error::PineconeResponseError(code,typ,msg) => {
                        if code == StatusCode::BAD_REQUEST {
                            assert!(true);
                            return;
                        }
                        panic!("Unable to create collection: {:?}", Error::PineconeResponseError(code, typ, msg))
                    },
                    _ => {
                        panic!("Unable to create collection: {:?}", error)
                    }
                }
            }
        }
    }


    #[wasm_bindgen_test]
    async fn test_delete_collection(){
        let client = create_client();
        match client.delete_collection("testcollection").await {
            Ok(_) => {
                assert!(true)
            },
            Err(error) => {
                match error {
                    Error::PineconeResponseError(code,typ,msg) => {
                        if code == StatusCode::BAD_REQUEST {
                            assert!(true);
                            return;
                        }
                        panic!("Unable to delete collection: {:?}", Error::PineconeResponseError(code, typ, msg))
                    },
                    _ => {
                        panic!("Unable to delete collection: {:?}", error)
                    }
                }
            }
        }
    }
} 

