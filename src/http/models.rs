//! All the different models and data types sent and recieved to pinecone as well as their
//! alternative respesentations / rust specific representations are stored within this file.
//!
//! As the API updates these values will update. If changes do occur please create an issue if you
//! notice a breaking change.

use core::fmt;
use std::collections::{HashMap, BTreeMap};

use serde::{Serialize, Deserialize};

/// The distance metric used for similarity search.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Metric {
    /// Euclidian Distance.
    #[serde(rename="euclidean")]
    #[default]
    EUCLIDEAN,
    /// Cosine Similarity
    #[serde(rename="cosine")]
    COSINE,
    /// Dot Product Similarity
    #[serde(rename="dotproduct")]
    DOTPRODUCT
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Metric::EUCLIDEAN => write!(f, "euclidean"),
            Metric::COSINE => write!(f, "cosine"),
            Metric::DOTPRODUCT => write!(f, "dotproduct")
        }
    }
}

/// Returned data type under the /actions/whoami GET request detailing the Client's information
/// used for future requests as well as Client validation.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClientInfo {
    /// Name of the clients project.
    pub project_name: String,
    /// Name of clients user label
    pub user_label: String,
    /// Clients username
    pub user_name: String
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct CreateCollectionRequest {
    pub(super) name: String,
    pub(super) source: String
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct VectorRequest {
    pub(super) namespace: String,
    pub(super) vectors: Vec<Vector>
}

/// Details the generic pinecone error response sent during index operations.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PineconeErrorResponse {
    /// Error code.
    pub code: usize,
    /// Error message.
    pub message: String,
    /// Details for the error
    ///
    /// Actual value of this is unkown at this time and if anyone knows please create an Issue so I
    /// can properly implement this type!
    pub details: Vec<MappedValue>
    //TODO: implement the details field
}

/// Request sent to pinecone for the creation of an Index.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexCreateRequest {
    /// Index name.
    pub name: String,
    /// The dimension for the vectors stored within the index.
    pub dimension: usize,
    /// The metric for the Index, all the options for this are detailed in [`Metric`].
    pub metric: String
}

/// Details information about an individual namespace, found in [`IndexStats`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Namespace {
    /// The number of vectors int he namespace.
    #[serde(rename = "vectorCount")]
    pub vector_count: usize
}

/// Request sent to change the settings / details of an index.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigureIndexRequest {
    /// The desired number of replicas for the index.
    pub replicas: usize,
    /// The new pod type for the index. One of s1, p1, or p2 appended with . and one of x1, x2, x4, or
    /// x8.
    pub pod_type: String
}

/// The stats / information about an index, this is different information compared to
/// [`IndexDescription`].
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexStats {
    /// List of namespaces in the index.
    pub namespaces: HashMap<String, Namespace>,
    /// Dimension of vectors within the index.
    pub dimension: usize,
    /// Index Fullness.
    #[serde(rename= "indexFullness")]
    pub index_fullness: u32,
    /// Total number of vectors within the index.
    #[serde(rename = "totalVectorCount")]
    pub total_vector_count: u32
}

/// Vector sparse data. Represented as a list of indeices and a list of corresponded values, which
/// must be the same length.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SparseValues {
    /// Indecies.
    pub indeces: Vec<u32>,
    /// Values.
    pub values: Vec<f32>
}

/// Pinecones data sent during the succesfull Fetch Request.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FetchResponse{
    /// A map of Vector IDs to Vectors
    pub vectors: BTreeMap<String, Vector>,
    /// The namespace. This might be empty if no namespace was specified.
    pub namespace: String
}

/// A value representing a map from a string to a currently unknown value, as the value of these is
/// better understood their implementations might be transfered to a more type strict version.
pub type MappedValue = BTreeMap<String, serde_json::Value>;

/// Updates a vector in a namespace.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UpdateRequest {
    /// Vector id to update.
    pub id: String,
    /// Values to change it to
    pub values: Option<Vec<f32>>,
    /// New Sparse values to update
    #[serde(rename="sparseValues")]
    pub sparse_values: Option<SparseValues>,
    /// New metadata values.
    #[serde(rename="setMetadata")]
    pub metadata: Option<MappedValue>,
    /// Namespace to run this operation on, empty namespace can be used if you would like to run it
    /// on the whole index.
    pub namespace: Option<String>,
}

/// Represents a vector that can be sent and retrieved from pinecone.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vector {
    /// Unique Identifier for the vector.
    pub id: String,
    /// The values this vector holds, this should the same length as the dimension of the vector.
    pub values: Vec<f32>,
    /// The sparse values the vector should hold.
    pub sprase_values: Option<SparseValues>,
    /// Vector metadata that can be used during queries.
    pub metadata: Option<MappedValue>
}

/// Data type detailing the search of a namespace using a query vector.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct QueryRequest {
    /// The namespace to query.
    pub namespace: Option<String>,
    /// Number of values to search for. This number cannot be 0 or negative.
    #[serde(rename="topK")]
    pub top_k: usize, 
    /// The filter to apply. You can use vector metadata to limit your search. See [Metadat
    /// Filtering](https://www.pinecone.io/docs/metadata-filtering/)
    pub filter: Option<BTreeMap<String, serde_json::Value>>,
    /// Whether vector values should be included in the response
    #[serde(rename="includeValues")]
    pub include_values: bool,
    /// Whether metadata should be included in the response
    #[serde(rename="includeMetadata")]
    pub include_metadata: bool,
    /// Vector value if include_values was true
    pub vector: Option<Vec<f32>>,
    /// Vector Sparse Data.
    #[serde(rename="sparseVectors")]
    pub sparse_vector: Option<SparseValues>,
    /// The unique id of the vector to be used as a query vector.
    pub id: Option<String>
}

/// Response Returned during a query operation.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct QueryResponse {
    /// All the matched values
    pub matches: Vec<Match>,
    /// namespace this operation was done under
    pub namespace: String
}

/// Match is a specific match under a Query Request.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Match {
    /// Matched Vectors
    pub id: String,
    /// This is a measure of simialrty between this vector and the query vector. It uses the
    /// [`Metric`] value specified during the creation of the index.
    pub score: Option<f32>,
    /// The vector values.
    pub values: Option<Vec<f32>>,
    /// Vector Sparse Data.
    #[serde(rename="sparseValues")]
    pub sparse_values: Option<SparseValues>,
    /// The vector metadata.
    pub metadata: Option<MappedValue>
}

/// Detailing parameters for an operation that looks up and returns vector by ID.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FetchRequest {
    /// Ids to search for
    pub ids: Vec<String>,
    /// Namespace to search for these vectors
    pub namespace: Option<String>
}

impl FetchRequest {

    pub(crate) fn url(self, base: impl Into<String>) -> String {
        let mut url: String = base.into();
        url.push_str("/vectors/fetch?");
        for id in self.ids {
            url += format!("ids={}&", id).as_ref();
        }
        if let Some(namespace) = self.namespace {
            url += format!("namespace={}", namespace).as_ref();
        } else {
            url.truncate(url.len() - 1);
        }
        url
    }
}

/// Returns information about the index in great depth.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexDescription {
    /// Specific Index Information.
    pub database: IndexDatabaseDescription,
    /// Current status of the index.
    pub status: IndexStatusDescription 
}

/// Details the configuration of the index.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexDatabaseDescription {
    /// Index name.
    pub name: String,
    /// Vector dimension.
    pub dimension: usize,
    /// The similairty metric, values listed in [`Metric`].
    pub metric: Metric,
    /// Number of replicas.
    pub replicas: usize,
    /// Number of shards.
    pub shards: usize,
    /// Number of pods.
    pub pods: usize,
    /// Pod type for the index.
    pub pod_type: String,
}

/// Describes an Indexes Configuration
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DescribeIndexConfig {
    /// k bits.
    pub k_bits: usize,
    /// Hybrid.
    pub hybrid: bool
}

/// Describes the Status of an Index
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexStatusDescription {
    /// The index is waiting on some kind of operation.
    pub waiting: Vec<serde_json::Value>,
    /// Index has crashed.
    pub crashed: Vec<serde_json::Value>,
    /// The host url of the index.
    pub host: Option<String>,
    /// The port of the index.
    pub port: usize,
    /// Indexes state
    pub state: DescribeStatusState,
    /// Whether the Index is ready or not.
    pub ready: bool,
}

/// Current Status of an Index. Whenever you create a new index, for a minute or see you will
/// recieve an Initializing index status and connot run any index operations on it.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum DescribeStatusState {
    /// Initializing.
    #[default]
    Initializing,
    /// Scaling Up.
    ScalingUp,
    /// ScalingDown.
    ScalingDown,
    /// Terminating.
    Terminating,
    /// Ready.
    Ready,
    /// InitializationFailed.
    InitializationFailed
}

impl fmt::Display for DescribeStatusState {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DescribeStatusState::Initializing => write!(f, "Initializing"),
            DescribeStatusState::ScalingUp => write!(f, "ScalingUp"),
            DescribeStatusState::ScalingDown => write!(f, "ScalingDown"),
            DescribeStatusState::Terminating => write!(f, "Terminating"),
            DescribeStatusState::Ready => write!(f, "Ready"),
            DescribeStatusState::InitializationFailed => write!(f, "InitializationFailed")
        }
    }
}

/// Description / Information about a collection.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CollectionDescription {
    /// Collection name.
    pub name: String,
    /// Collection size.
    pub size: usize,
    /// Status of the collection.
    pub status: String
}

/// Response from an upsert request sending data to the Index.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UpsertResponse {
    /// Number of vectors upserted.
    #[serde(rename = "upsertedCount")]
    pub upserted_count: usize
}
