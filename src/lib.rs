//! This crate allows you to connect and interact with Pinecone, the vector database. After
//! authenticating you're api key and you're environment key, this crate facilitates the connection
//! to a pinecone index. Once a connection has been validated this API allows you to upsert,
//! query, and update data within Pinecone as well more unmentioned commands. More details about
//! the different api methods can be found [here](https://docs.pinecone.io/reference/describe_index_stats_post)
//!
//! This crate currently only supports the http / rest pinecone api and does not support GRCP. GRCP
//! will be implemented into the future as opt in. Http is currently the default
//!
//! To connect, initalize a [`Client`] using the [`Client::new`] method. This is an asynchronous
//! operation that will also validate you're credentials and will error if invalid credentials are
//! given. You can then run operations on you're client / account using the methods on [`Client`]
//! or create a new [`Index`] via the [`Client::index`] method. This index will not be validated
//! intially, it can be validated by calling the [`Index::describe`] method which will attempt to
//! get information about the index and subsequently validate the credentials if it goes through
//! successfully.
//!
//! Below is a basic client and index example.
//!```no_run
//!use pinenut::{Client, models::Vector};
//!
//!async fn index_upsert() {
//!
//!    // We create an instance of client first and firstmost. Panics if it couldn't authenticate.
//!    let client = Client::new(env!("PINECONE_API_KEY"), env!("PINECONE_ENV")).await.unwrap();
//!    // creates an index, will not authenticate.
//!    let mut index = client.index(env!("PINECONE_INDEX_NAME"));
//!
//!    // We use describe as a form of authenticate, panicing if we couldn't authenticate.
//!    let _ = index.describe().await.unwrap();
//!    let vec = Vector{
//!        id: "B".to_string(),
//!        values: vec![0.5; 32],
//!        sparse_values: None,
//!        metadata: None
//!    };
//!
//!    match index.upsert(String::from("odle"), vec![vec]).await {
//!        Ok(_) => assert!(true),
//!        Err(err) => panic!("unable to upsert: {:?}", err)
//!    }
//!}
//!```
//!
//!
//!

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

macro_rules! if_rest {
    ($($item:item)*) => {$(
        #[cfg(feature="rest")]
        $item
    )*}
}

//TODO: create macro for GRCP when that becomes implemented

if_rest! {
    mod rest;
    pub use self::rest::{models, Client, Index};
}

pub mod error;
pub use crate::error::{Error, Result};
