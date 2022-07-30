use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlaggerError {
    #[error("the {0} field is required")]
    Builder(String),
    #[error("Entity not found")]
    NotFound,
}
