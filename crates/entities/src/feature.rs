use flagger_types::repository::Repository;
use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde::{Deserialize, Serialize};

// serialize_object_id_as_hex_string

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FeatureKind {
    KillSwitch,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Feature {
    #[serde(
        rename(serialize = "_id", deserialize = "_id"),
        serialize_with = "serialize_object_id_as_hex_string"
    )]
    pub id: ObjectId,
    pub kind: FeatureKind,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureInput {
    pub kind: FeatureKind,
    pub name: String,
    pub description: Option<String>,
}

impl Repository for Feature {
    fn name() -> &'static str {
        "features"
    }
}
