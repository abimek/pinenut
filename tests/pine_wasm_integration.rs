extern crate pinenut;

use pinenut::{models::Vector, Client};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
async fn index_upsert() {
    // We create an instance of client first and firstmost. Panics if it couldn't authenticate.
    let client = Client::new(env!("PINECONE_API_KEY"), env!("PINECONE_ENV"))
        .await
        .unwrap();

    // creates an index, will not authenticate.
    let mut index = client.index(env!("PINECONE_INDEX_NAME"));

    // We use describe as a form of authenticate, panicing if we couldn't authenticate.
    let _ = index.describe().await.unwrap();
    let vec = Vector {
        id: "B".to_string(),
        values: vec![0.5; 32],
        sprase_values: None,
        metadata: None,
    };

    match index.upsert(String::from("odle"), vec![vec]).await {
        Ok(_) => assert!(true),
        Err(err) => panic!("unable to upsert: {:?}", err),
    }
}
