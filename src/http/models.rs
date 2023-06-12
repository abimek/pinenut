use std::collections::{HashMap, BTreeMap};

use serde::{Serialize, Deserialize};

/// This represents all the values 
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
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
    pub details: Vec<HashMap<String, Value>>
    //TODO: implement the details field
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexCreateRequest {
    pub(super) name: String,
    pub(super) dimension: usize,
    pub(super) metric: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
    #[serde(rename = "vectorCount")]
    vector_count: usize
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
    indeces: Vec<u32>,
    values: Vec<f32>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vector {
    pub id: String,
    pub values: Vec<f32>,
    pub sprase_values: Option<SparseValues>,
    pub metadata: Option<BTreeMap<String, serde_json::Value>>
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
    pub waiting: Vec<Value>,
    pub crashed: Vec<Value>,
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
