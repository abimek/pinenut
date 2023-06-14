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
//!
//! TODO: Index and Client example
//!
//!
//!
//!

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

macro_rules! if_http {
    ($($item:item)*) => {$(
        #[cfg(feature="http")]
        $item
    )*}
}

//TODO: create macro for GRCP when that becomes implemented

if_http! {
    mod http;
    pub use self::http::{models, Client, Index};
}

pub mod error;
pub use crate::error::{Error, Result};
