use mongodb::{bson::ser::Error as BsonError, error::Error as MongoDBError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlaggerError {
    #[error("unexpected {0} from MongoDB")]
    Mongo(#[from] MongoDBError),
    #[error("unexpected {0} serializing BSON")]
    Bson(#[from] BsonError),
    #[error("the {0} field is required")]
    Builder(String),
    #[error("Entity not found")]
    NotFound,
}
