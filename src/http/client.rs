use crate::{
    Result, 
    http::try_pincone_get_request
};
use super::{
    Index,
    models::{IndexDescription, CollectionDescription}
};


pub struct Credentials {
    api_key: String,
    environment: String
}


impl Credentials {
    pub(super) fn api_key(&self) -> &str {
        &self.api_key
    }

    pub(super) fn environment(&self) -> &str {
        &self.environment
    }
}

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

    pub(super) fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub(super) fn credentials(&self) -> &Credentials {
        &self.creds
    }

    /// returns a list of vector indexes
    pub async fn list_indexes(&self) -> Result<Vec<String>> {
        return try_pincone_get_request::<Vec<String>>(&self, "/databases").await;
    }

    pub async fn describe_index(&self, name: impl AsRef<str>) -> Result<IndexDescription> {
        return try_pincone_get_request::<IndexDescription>(&self, format!("/databases/{}", name.as_ref())).await;
    }

    pub async fn list_collections(&self) -> Result<Vec<String>> {
        return try_pincone_get_request::<Vec<String>>(&self, "/collections").await;
    }

    pub async fn describe_collection(&self, name: impl AsRef<str>) -> Result<CollectionDescription> {
        return try_pincone_get_request::<CollectionDescription>(&self, format!("/collections/{}", name.as_ref())).await;
    }

    /// name is the name of the index and project is the name of the project, project can be seen
    /// in the link given in the pinecone indexes list, it's the content behind the name of the
    /// index but before .svc
    pub async fn index<'a>(&'a self, name: impl Into<String>) -> Index<'a> {
        Index::new(self, name)
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
                println!("{:?}", list);
                assert!((list.len() >= 0), "working? {:?}", list);
            },
            Err(error) => panic!("Unable to list indexes: {:?}", error)
        }
    }
} 

