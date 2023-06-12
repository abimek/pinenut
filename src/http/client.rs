use crate::{
    Result, 
    Index, models::CollectionDescription
};

use super::{Credentials, Connection, try_pinecone_get_request};

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
        try_pinecone_get_request::<Self, Vec<String>>(self, "/databases", None::<String>).await
    }

    /// Will list all the collections associated with the given [`Credentialsc`].
    pub async fn list_collections(&self) -> Result<Vec<String>> {
        try_pinecone_get_request::<Self, Vec<String>>(self, "/collections", None::<String>).await
    }

    /// Describes a specific collection returning specific information that can be referenced at
    /// the pinecone api reference (https://docs.pinecone.io/reference/describe_collection)
    pub async fn describe_collection(&self, name: impl AsRef<str>) -> Result<CollectionDescription> {
        try_pinecone_get_request::<Self, CollectionDescription>(self, format!("/collections/{}", name.as_ref()), None::<String>).await
    }

    /// Creates and returns an Index object that can be used to run index specific operations, it
    /// is the primary way you interface with the Index api.
    pub async fn index(self, name: impl Into<String>) -> Index {
        Index::new::<Self>(&self, name)
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
/*
    #[wasm_bindgen_test]
    async fn test_validate_credentials() {
        let client = Client::new(
            env!("PINECONE_API_KEY"),
            env!("PINECONE_ENV")
        );
        match client.validate_credentials().await {
            Ok(_) => assert!(true),
            Err(err) => panic!("failed to validate configuration settings: {:?}", err)
        }
    }

    #[wasm_bindgen_test]
    #[should_panic]
    async fn test_validate_credentials_panic() {
        let client = Client::new(
            "",
            ""
        );
        match client {
            Ok(client) => {
                match client.validate_credentials().await {
                    Ok(_) => assert!(true),
                    Err(err) => panic!("failed to validate configuration settings: {:?}", err)
                }
            },
            Err(_) => panic!("unable to create client")
        }
    }*/

    /*
    #[wasm_bindgen_test]
    async fn test_create_index() {
        let client = create_client();
        match client.create_index("hoogle", 32, Metric::EUCLIDEAN).await {
            Ok(_) => assert!(true),
            Err(err) => panic!("failed to create index: {:?}", err)
        }
    }*/

    #[wasm_bindgen_test]
    async fn test_list_indexes() {
        let client = create_client();
        match client.list_indexes().await {
            Ok(list) => {
                assert!((list.len() > 0), "working? {:?}", list);
            },
            Err(error) => panic!("Unable to list indexes: {:?}", error)
        }
    }
} 

