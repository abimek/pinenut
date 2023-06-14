//! pinecone-wasm Errors

use crate::http::models::PineconeErrorResponse;
use reqwest::{Method, Response, StatusCode};
use std::result;
use thiserror::Error as ThisError;

/// result allias where the Err term is pine-client::Error
pub type Result<T> = result::Result<T, Error>;

/// A set of errors that can occur in the Pinecone Client.
#[derive(Debug, ThisError)]
#[non_exhaustive]
pub enum Error {
    /// An error for when an invalid argument is used / given.
    #[error("Invalid Argument for {name} found {found} expected {expected}")]
    ArgumentError {
        /// Name of the argument.
        name: String,
        /// Found value or description of found value.
        found: String,
        /// Expected value or description of expected value.
        expected: String,
    },

    /// This is the error used when request fails to make a request.
    #[error("Reqwest Error")]
    ReqwestError(reqwest::Error),

    /// An error returned when reqwest fails to do an action while the overall request is.
    /// successfull, it is used currently for when a request goes through but a json decode fails.
    #[error("Reqwest Response error")]
    ReqwestResponseError(StatusCode, reqwest::Error),

    /// When pinecone sends their [`PineconeErrorResponse`] to the client.
    #[error("Finetune Failed with Response with Status Code {0:?} {1:?} {2:?}")]
    PineconeResponseError(StatusCode, Option<PineconeErrorResponse>, Option<String>),

    /// This error is used when Pinecone fails to return a [`PineconeErrorResponse`]
    #[error("This request has failed")]
    PineconeError(Response),

    /// An error that describes an incorrectly sized vector.
    #[error("Vector of id {id} had dimension {found} expected dimension size of {expected}")]
    VectorDimensionError {
        /// The dimension found.
        found: u32,
        /// The expected dimension.
        expected: u32,
        /// The Vector id.
        id: String,
    },

    /// An error used for when the url value within an IndexDescription can't be found.
    #[error("URL is not available within [`pine_client::http::models::DescribeStatus`]")]
    URLNotAvailable,

    /// This is an internal error used for internal checks. This **should** never actually happen.
    #[error("Unsupported method: {}", method.as_str())]
    UnsupportedMethod {
        /// The unsupported method
        method: Method 
    },
}
