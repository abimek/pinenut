//! This crate allows you to connect and interact with Pinecone, the vector database. After
//! authenticating you're api key and you're environment key, this crate faciliates the connection
//! to a pinecone index.c
//!
//!
//!
//!
//!
//!
//!

//#![deny(missing_docs)]
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
