use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

pub trait Repository: Serialize + DeserializeOwned + Unpin + Debug {
    fn name() -> &'static str;
}
