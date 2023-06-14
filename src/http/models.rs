use core::fmt;
use std::collections::{HashMap, BTreeMap};

use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum Metric {
    EUCLIDEAN
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Metric::EUCLIDEAN => write!(f, "euclidean")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateCollectionRequest {
    pub(super) name: String,
    pub(super) source: String
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VectorRequest {
    pub(super) namespace: String,
    pub(super) vectors: Vec<Vector>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PineconeErrorResponse {
    pub code: usize,
    pub message: String,
    pub details: Vec<MappedValue>
    //TODO: implement the details field
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexCreateRequest {
    pub name: String,
    pub dimension: usize,
    pub metric: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Namespace {
    #[serde(rename = "vectorCount")]
    pub vector_count: usize
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigureIndexRequest {
    pub replicas: usize,
    pub pod_type: String
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexStats {
    pub namespaces: HashMap<String, Namespace>,
    pub dimension: usize,
    #[serde(rename= "indexFullness")]
    pub index_fullness: u32,
    #[serde(rename = "totalVectorCount")]
    pub total_vector_count: u32
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SparseValues {
    pub indeces: Vec<u32>,
    pub values: Vec<f32>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FetchResponse{
    pub vectors: BTreeMap<String, Vector>,
    pub namespace: String
}

pub type MappedValue = BTreeMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UpdateRequest {
    pub id: String,
    pub values: Option<Vec<f32>>,
    #[serde(rename="sparseValues")]
    pub sparse_values: Option<SparseValues>,
    #[serde(rename="setMetadata")]
    pub metadata: Option<MappedValue>,
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vector {
    pub id: String,
    pub values: Vec<f32>,
    pub sprase_values: Option<SparseValues>,
    pub metadata: Option<MappedValue>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct QueryRequest {
    pub namespace: Option<String>,
    #[serde(rename="topK")]
    pub top_k: usize, 
    pub filter: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(rename="includeValues")]
    pub include_values: bool,
    #[serde(rename="includeMetadata")]
    pub include_metadata: bool,
    pub vector: Option<Vec<f32>>,
    #[serde(rename="sparseVectors")]
    pub sparse_vector: Option<SparseValues>,
    pub id: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct QueryResponse {
    pub matches: Vec<Match>,
    pub namespace: String
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Match {
    pub id: String,
    pub score: Option<f32>,
    pub values: Option<Vec<f32>>,
    #[serde(rename="sparseValues")]
    pub sparse_values: Option<SparseValues>,
    pub metadata: Option<MappedValue>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FetchRequest {
    pub ids: Vec<String>,
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexDescription {
    pub database: IndexDatabaseDescription,
    pub status: IndexStatusDescription 
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexDatabaseDescription {
    pub name: String,
    pub dimension: usize,
    pub metric: String,
    //TODO: convert to Metric type
    pub replicas: usize,
    pub shards: usize,
    pub pods: usize,
    pub pod_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DescribeIndexConfig {
    pub k_bits: usize,
    pub hybrid: bool
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexStatusDescription {
    pub waiting: Vec<serde_json::Value>,
    pub crashed: Vec<serde_json::Value>,
    pub host: Option<String>,
    pub port: usize,
    pub state: DescribeStatusState,
    pub ready: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum DescribeStatusState {
    #[default]
    Initializing,
    ScalingUp,
    ScalingDown,
    Terminating,
    Ready,
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CollectionDescription {
    pub name: String,
    pub size: usize,
    pub status: String
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UpsertResponse {
    #[serde(rename = "upsertedCount")]
    pub upserted_count: usize
}
