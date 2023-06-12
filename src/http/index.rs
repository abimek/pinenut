use reqwest::{Method, StatusCode};
use crate::{error::{Result, Error}, http::{try_pincone_get_request, try_pinecone_post_request}};
use super::{
    models::{VectorRequest, Vector, IndexStats, UpsertResponse, IndexDescription},
    Client
};

#[derive(Debug)]
pub enum Metric {
    EUCLIDEAN
}

impl ToString for Metric {
    fn to_string(&self) -> String {
        match &self {
            Self::EUCLIDEAN => "euclidean".to_string()
        }
    }
}

impl Into<String> for Metric {
    fn into(self) -> String {
        self.to_string()
    }
}

/// A datatype that represents a connection to a specific pinecone index: TODO: Add code snipet
pub struct Index<'a> {
    client: &'a Client,
    name: String,
    description: Option<IndexDescription>,
    stats: Option<IndexStats> 
}

impl<'a> Index<'a> {

    /// creates a new Index
    pub fn new(client: &'a Client, name: impl Into<String>) -> Index<'a> {
        Index{
            client,
            name: name.into(),
            description: None,
            stats: None
        }
    }

    pub(super) fn client(&self) -> &Client {
        self.client
    }

    pub async fn describe(&mut self)  -> Result<&IndexDescription> {
        self.description = Some(try_pincone_get_request::<IndexDescription>(self.client, format!("/databases/{}", self.name)).await?);
        Ok(self.description().unwrap())
    }

    pub fn description(&self) -> Option<&IndexDescription> {
        self.description.as_ref()
    }

    /// conveniant rapper to get the api url for post requests 
    pub fn url(&self) -> Option<&str> {
        if let Some(ref desc) = self.description {
            if let Some(ref host) = desc.status.host {
                return Some(host.as_ref());
            }
        }
        None
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
   /*     for vec in &vectors {
            if vec.values.len() as u32 != self.stats.dimension {
                return Err(Error::VectorDimensionError{found: vec.values.len() as u32, expected: self.stats.dimension, id: vec.id.to_string()});
            }
        }*/
        let upsert = VectorRequest{
            namespace,
            vectors
        };
        try_pinecone_post_request::<VectorRequest, UpsertResponse>(self, true, "/vectors/upsert", &upsert).await
    }
/*
    pub async fn upsert(&self, namespace: String, vectors: Vec<Vector>) -> Result<()> {
        for vec in &vectors {
            if vec.values.len() as u32 != self.stats.dimension {
                return Err(Error::VectorDimensionError{found: vec.values.len() as u32, expected: self.stats.dimension, id: vec.id.to_string()});
            }
        }
        let upsert = VectorRequest{
            namespace,
            vectors
        };
        let response = self.base_request(Method::POST, "/vectors/upsert")
            .json(&upsert)
            .send()
            .await;

        match response {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => Ok(()),
                    _ => {
                        match res.json::<UpsertErrorResponse>().await {
                            Ok(err) => {
                                Err(Error::UpsertError(Some(err), None))
                            },
                            Err(err) => {
                                Err(Error::UpsertError(None, Some(err)))
                            }
                        }
                    }
                }
            },
            Err(err) => Err(Error::HTTPFailureError(Method::POST, err))
        }
    }*/

}

#[cfg(test)]
mod index_tests {

    use super::*;
    use wasm_bindgen_test::*;
    use super::Client;

    fn create_client() -> Client {
        Client::new(
            env!("PINECONE_API_KEY"),
            env!("PINECONE_ENV")
        )
    }

    async fn create_index<'a>(client: &'a Client) -> Index<'a> {
        Index::new(client, env!("PINECONE_INDEX_NAME"))
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
}



