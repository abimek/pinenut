
use crate::{Result, Error};

use super::{
    try_pinecone_get_request,
    try_pinecone_post_request,
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
    pub async fn describe(&mut self)  -> Result<&IndexDescription> {
        self.description = Some(try_pinecone_get_request::<Self, IndexDescription>(self, format!("/databases/{}", self.name), None::<String>).await?);
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
        self.describe().await;
        if let Some(url) = self.url() {
            return Ok(url);
        }
        return Err(Error::URLNotAvailable)
    }


    pub async fn describe_stats(&mut self) -> Result<&IndexStats> {
        let url = Some(self.try_url().await?);
        self.stats = Some(try_pinecone_get_request::<Self, IndexStats>(self, "/describe_index_stats", url).await?);
        Ok(self.stats().unwrap())
    }

    pub fn stats(&self) -> Option<&IndexStats> {
        self.stats.as_ref()
    }



/*    /// gets the latest information / statistics about the index, this is also used to validate
    /// that we have a proper connection
    pub async fn describe_stats(&mut self) -> Result<()> {
      /*  let response = self.base_request(Method::GET, format!("/databases/{}", self.name))
            .send()
            .await;

        match response {
            Ok(resp) => {
                match resp.json::<IndexStats>().await {
                    Ok(index_info) => {
                        self.stats = index_info;
                        Ok(())
                    },
                    Err(err) => Err(Error::DescribeIndexError(err))
                }
            },
            Err(err) => {
                Err(Error::HTTPReqwestError("failed to get index stats".to_string(), err))
            }
        }*/
    }*/


    pub async fn upsert(&mut self, namespace: String, vectors: Vec<Vector>) -> Result<UpsertResponse> {
        let upsert = VectorRequest{
            namespace,
            vectors
        };
        try_pinecone_post_request::<VectorRequest, UpsertResponse>(self, true, "/vectors/upsert", &upsert).await
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
}



