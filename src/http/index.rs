use reqwest::{StatusCode, Method};
use crate::{Result, Error, http::{try_pinecone_request_json, try_pinecone_request_text}, models::ConfigureIndexRequest}; 

use super::{
    Connection,
    Credentials,
    models::{VectorRequest, Vector, IndexStats, UpsertResponse, IndexDescription, Metric},
};

impl From<Metric> for String {
    fn from(value: Metric) -> Self {
        value.to_string()
    }
}

/// A datatype that represents a connection to a specific pinecone index: TODO: Add code snipet
pub struct Index {
    client: reqwest::Client,
    name: String,
    creds: Credentials,
    description: Option<IndexDescription>,
    stats: Option<IndexStats> 
}

impl Index {

    /// creates a new Index
    pub(crate) fn new<C>(con: &C, name: impl Into<String>) -> Index 
    where
        C: Connection
    {
        Index {
            client: reqwest::Client::new(),
            name: name.into(),
            creds: con.credentials().clone(), 
            description: None,
            stats: None
        }
    }

    /// This will get a brad new IndexDescription from pinecone. If successfull this will be
    /// cached.
    ///
    /// This operation is done for the other pinecone operations / requests as it returns us the url 
    /// to make requests too
    ///
    pub async fn describe(&mut self)  -> Result<&IndexDescription> {
        let name = self.name.clone();
        self.description = Some(try_pinecone_request_json::<Index, String, IndexDescription>(self, Method::GET, StatusCode::OK, None::<String>, format!("/databases/{}", name), None).await?);
        Ok(self.description().unwrap())
    }

    /// This will return the cached Index description if it exists.
    pub fn description(&self) -> Option<&IndexDescription> {
        self.description.as_ref()
    }

    /// Attempts to return the cached [`IndexDescription`], if unsuccessfull it will make a request
    /// and then return the cached description
    pub async fn cached_then_normal_describe(&mut self) -> Result<&IndexDescription>{
        if let Some(ref desc) = self.description {
            return Ok(desc);
        }
        self.describe().await
    }


    /// Returns the url for api requests if it's been cached, this is typically stored in
    /// [`IndexDescription`]
    pub fn url(&self) -> Option<String> {
        if let Some(ref desc) = self.description {
            if let Some(ref host) = desc.status.host {
                return Some(host.to_string());
            }
        }
        None
    }

    pub async fn try_url(&mut self) -> Result<String> {
        if let Some(url) = self.url() {
            return Ok(url);
        }
        let _ = self.describe().await;
        if let Some(url) = self.url() {
            return Ok(url);
        }
        Err(Error::URLNotAvailable)
    }


    pub async fn describe_stats(&mut self) -> Result<&IndexStats> {
        let url = self.try_url().await?;
        self.stats = Some(try_pinecone_request_json::<Index, String, IndexStats>(self, Method::GET, StatusCode::OK, Some(url), "/describe_index_stats", None).await?);
        Ok(self.stats().unwrap())
    }

    pub fn stats(&self) -> Option<&IndexStats> {
        self.stats.as_ref()
    }

    pub async fn upsert(&mut self, namespace: String, vectors: Vec<Vector>) -> Result<UpsertResponse> {
        let upsert = VectorRequest{
            namespace,
            vectors
        };
        let url = self.try_url().await?;
        try_pinecone_request_json::<Index, VectorRequest, UpsertResponse>(self, Method::POST, StatusCode::OK, Some(url), "/vectors/upsert", Some(&upsert)).await
    }

    pub async fn delete(self) -> Result<String> {
        try_pinecone_request_text::<Index, String>(&self, Method::DELETE, StatusCode::ACCEPTED, None::<String>, format!("/databases/{}", self.name), None).await
    }

    pub async fn configure(&self, replicas: usize, pod_type: String) -> Result<String> {
        let p = ConfigureIndexRequest{
            replicas,
            pod_type
        };
        try_pinecone_request_text::<Index, ConfigureIndexRequest>(self, Method::PATCH, StatusCode::ACCEPTED, None::<String>, format!("/databases/{}", self.name), Some(&p)).await
    }

}

impl Connection for Index {
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn credentials(&self) -> &Credentials {
        &self.creds
    }
}
#[cfg(test)]
mod index_tests {

    use super::*;
    use wasm_bindgen_test::*;
    use super::super::Client;

    fn create_client() -> Client {
        Client::new(
            env!("PINECONE_API_KEY"),
            env!("PINECONE_ENV")
        )
    }

    async fn create_index(con: &impl Connection) -> Index {
        Index::new(con, env!("PINECONE_INDEX_NAME"))
    }

    #[wasm_bindgen_test]
    async fn test_upsert() {
        let client = create_client();
        let mut index = create_index(&client).await;
        let vec = Vector{
            id: "A".to_string(),
            values: vec![0.5; 32],
            sprase_values: None,
            metadata: None
        };
        match index.upsert(String::from("halfbaked"), vec![vec]).await {
            Ok(_) => assert!(true),
            Err(err) => panic!("unable to upsert: {:?}", err)
        }
    }

    #[wasm_bindgen_test]
    async fn test_describe() {
        let client = create_client();
        let mut index = create_index(&client).await;
        match index.describe().await {
            Ok(_) => assert!(true),
            Err(err) => panic!("failed to get description: {:?}", err)

        }
    }

    #[wasm_bindgen_test]
    async fn test_describe_stats() {
        let client = create_client();
        let mut index = create_index(&client).await;
        match index.describe_stats().await {
            Ok(_) => assert!(true),
            Err(err) => panic!("failed to get index stats: {:?}", err)
        }
    }

    #[wasm_bindgen_test]
    async fn test_configure_index() {
        let client = create_client();
        let index = create_index(&client).await;
        match index.configure(1, "s1.x1".to_string()).await {
            Ok(_) => assert!(true),
            Err(error) => {
                match error {
                    Error::PineconeResponseError(code,typ,msg) => {
                        if code == StatusCode::BAD_REQUEST {
                            assert!(true);
                            return;
                        }
                        panic!("Unable to configure index: {:?}", Error::PineconeResponseError(code, typ, msg))
                    },
                    _ => {
                        panic!("Unable to configure index: {:?}", error)
                    }
                }
            }
        }
    }


/*
    #[wasm_bindgen_test]
    async fn test_delete_index() {
        let client = create_client();
        let index = create_index(&client).await;
        match index.delete().await {
            Ok(_) => assert!(true),
            Err(error) => {
                match error {
                    Error::PineconeResponseError(code,typ,msg) => {
                        if code == StatusCode::NOT_FOUND {
                            assert!(true);
                            return;
                        }
                        panic!("Unable to delete index: {:?}", Error::PineconeResponseError(code, typ, msg))
                    },
                    _ => {
                        panic!("Unable to delete index: {:?}", error)
                    }
                }
            }
        }
    }*/
}



