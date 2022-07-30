mod context;
mod error;
mod flagger;

pub use context::FlaggerContext;
pub use error::FlaggerError;
pub use flagger::{Flagger, FlaggerBuilder};

#[cfg(any(test, feature = "test-suite"))]
pub mod tests {
    use flagger_database_adapter::tests::InMemoryDatabaseAdapter;

    use crate::{Flagger, FlaggerError};

    pub async fn test_flagger() -> Result<Flagger, FlaggerError> {
        Ok(Flagger::builder()
            .with_database(Box::new(InMemoryDatabaseAdapter::default()))
            .build()
            .await?)
    }
}
