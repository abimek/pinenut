use reqwest::{StatusCode, Method};
use crate::{
    Result, 
    Index, models::{CollectionDescription, CreateCollectionRequest, IndexCreateRequest, ClientInfo}, http::{try_pinecone_request_json, try_pinecone_request_text}

};
use super::{Credentials, Connection};

/// An (authenticated) handle to talk with Pinecone. This is where you first go when you need a
/// connection. Specific method descriptions and details can be refered to at [Pinecone](https://docs.pinecone.io/reference/list_collections).
pub struct Client{
    client: reqwest::Client,
    creds: Credentials,
    info: ClientInfo
}

impl Client {

    /// Attempts to validate credentials and return a [`Client`]. 
    ///
    /// If validated it will generate a [`ClientInfo`] which holds the required information
    /// for valid requests.
    pub async fn new<D>(api_key: D, environment: D) -> Result<Client>
    where
        D: Into<String>
    {
        let mut c = Client{
            client: reqwest::Client::new(),
            creds: Credentials{
                api_key: api_key.into(),
                environment: environment.into()
            },
            info: ClientInfo::default()
        };
        let r = try_pinecone_request_json::<Client, String, ClientInfo>(&c, Method::GET, StatusCode::OK, None::<String>, "/actions/whoami", None).await;
        c.info = r?;
        Ok(c)
    }

    /// Returns the [`ClientInfo`] generated during on the [`Client::new`] call.
    pub fn info(&self) -> &ClientInfo {
        &self.info
    }

    /// Will list all the indexes associated with the given instance of [`Client`].
    pub async fn list_indexes(&self) -> Result<Vec<String>> {
        try_pinecone_request_json::<Client, String, Vec<String>>(self, Method::GET, StatusCode::OK, None::<String>, "/databases", None).await
    }

    /// Lists all the collections associated with the given instance [`Client`].
    pub async fn list_collections(&self) -> Result<Vec<String>> {
        try_pinecone_request_json::<Client, String, Vec<String>>(self, Method::GET, StatusCode::OK, None::<String>, "/collections", None).await
    }

    /// Creates a new collection. 
    /// 
    /// For more information on Collections vist [Pinecone](https://docs.pinecone.io/docs/collections).
    pub async fn create_collection(&self, name: impl Into<String>, source_index: impl AsRef<str>) -> Result<String> {
        let request = CreateCollectionRequest{
            name: name.into(),
            source: source_index.as_ref().to_string()
        };
        try_pinecone_request_text::<Client, CreateCollectionRequest>(self, Method::POST, StatusCode::CREATED, None::<String>, "/collections", Some(&request)).await
    }

    /// Attempts to get a description of a collection. 
    ///
    /// # Error
    ///
    /// This function will error if the collection does not exist
    pub async fn describe_collection(&self, name: impl AsRef<str>) -> Result<CollectionDescription> {
        try_pinecone_request_json::<Client, String, CollectionDescription>(self, Method::GET, StatusCode::OK, None::<String>, format!("/collections/{}", name.as_ref()), None).await
    }

    /// Deletes a given collection.
    pub async fn delete_collection(&self, name: impl AsRef<str>) -> Result<String> {
        try_pinecone_request_text::<Client, String>(self, Method::DELETE, StatusCode::ACCEPTED, None::<String>, format!("/collections/{}", name.as_ref()), None).await
    }

    /// Creates a collection.
    ///
    /// The index create operation will take time even after the response is [`StatusCode::CREATED`] and index 
    /// operations will not function until this is the case. If creating an index ensure that you
    /// add a resonable delay.
    ///
    /// This will error with status code 409 [`StatusCode::CONFLICT`] if the name already exists.
    /// This should be checked for if you're trying to validate if a given index exists before
    /// doing operations.
    ///
    pub async fn create_index(&self, data: IndexCreateRequest) -> Result<String> {
        try_pinecone_request_text::<Client, IndexCreateRequest>(self, Method::POST, StatusCode::CREATED, None::<String>, "/databases", Some(&data)).await
    }
    /// Creates and returns an Index object that can be used to run index specific operations, it
    /// is the primary way you interface with the Index Api. The index created will not be a
    /// validated index and therefor should be validated using the [`Index::describe`] method.
    pub fn index(&self, name: impl Into<String>) -> Index {
        Index::new::<Self>(self, name, self.info())
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
    use crate::{Error, models::Metric};

    async fn create_client() -> Client {
        Client::new(
            env!("PINECONE_API_KEY"),
            env!("PINECONE_ENV")
        ).await.unwrap()
    }

    #[wasm_bindgen_test]
    async fn test_create_client() {
        let c = Client::new(
            env!("PINECONE_API_KEY"),
            env!("PINECONE_ENV")
        ).await;
        match c {
            Ok(_) => assert!(true),
            Err(err) => panic!("failed to create client {:?}", err)
        }
        assert!(true)
    }

    #[wasm_bindgen_test]
    async fn test_list_indexes() {
        let client = create_client().await;
        match client.list_indexes().await {
            Ok(list) => {
                assert!(!list.is_empty(), "working? {:?}", list);
            },
            Err(error) => panic!("Unable to list indexes: {:?}", error)
        }
    }

    #[wasm_bindgen_test]
    async fn test_create_index() {
        let client = create_client().await;
        match client.create_index(IndexCreateRequest{
            name: env!("PINECONE_INDEX_NAME").to_string(),
            dimension: 32,
            metric: Metric::EUCLIDEAN.to_string()
        }).await {
            Ok(_) => assert!(true),
            Err(error) => {
                match error {
                    Error::PineconeResponseError(code,typ,msg) => {
                        if code == StatusCode::CONFLICT || code == StatusCode::BAD_REQUEST {
                            assert!(true);
                            return;
                        }
                        panic!("Unable to create index: {:?}", Error::PineconeResponseError(code, typ, msg))
                    },
                    _ => {
                        panic!("Unable to create index: {:?}", error)
                    }
                }
            }

        }
    }

    #[wasm_bindgen_test]
    async fn test_create_collection() {
        let client = create_client().await;
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
        let client = create_client().await;
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

