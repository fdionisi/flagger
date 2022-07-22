mod context;
pub mod controllers;
mod database;
mod error;
mod flagger;

pub use error::FlaggerError;
pub use flagger::{Flagger, FlaggerBuilder};

#[cfg(any(test, feature = "test-suite"))]
pub mod tests {
    use crate::{Flagger, FlaggerError};

    pub async fn test_flagger<S>(database_name: S) -> Result<Flagger, FlaggerError>
    where
        S: Into<String>,
    {
        Ok(Flagger::builder()
            .with_database_name(database_name)
            .with_database_url(
                std::env::var("FLAGGER_DATABASE_URL").expect("FLAGGER_DATABASE_URL to be set"),
            )
            .build()
            .await?)
    }
}
