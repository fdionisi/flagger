use std::string::ToString;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The feature's kind.
#[derive(Clone, ToSchema, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FeatureKind {
    KillSwitch = 0,
}

/// A feature is the smallest and most central entity of the platform.
#[derive(Clone, ToSchema, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Feature {
    /// The feature's kind.
    pub kind: FeatureKind,
    /// A human and machine readable name for the feature.
    #[schema(example = "experiments/website-landingpage-alternative")]
    pub name: String,
    /// Human centric details regarding the feature. It should try to answer the question "what is the purpose of this feature?"
    pub description: Option<String>,
    /// Wether or not the feature is enabled.
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Clone, ToSchema, Debug, Deserialize, Serialize)]
pub struct FeatureInput {
    /// The new feature's kind.
    pub kind: FeatureKind,
    /// A human and machine readable name for the new feature.
    pub name: String,
    /// Human centric details regarding the feature. It should try to answer the question "what is the purpose of this feature?"
    pub description: Option<String>,
}

impl Feature {
    pub fn ser(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn de<B: AsRef<[u8]>>(bytes: B) -> Self {
        bincode::deserialize(&bytes.as_ref()).unwrap()
    }
}

impl<S> From<S> for FeatureKind
where
    S: AsRef<str>,
{
    fn from(data: S) -> Self {
        match data.as_ref() {
            "KillSwitch" => Self::KillSwitch,
            _ => panic!("invalid value"),
        }
    }
}

impl ToString for FeatureKind {
    fn to_string(&self) -> String {
        match self {
            Self::KillSwitch => String::from("KillSwitch"),
        }
    }
}
