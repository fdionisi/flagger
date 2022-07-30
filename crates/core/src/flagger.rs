use flagger_database_adapter::DatabaseAdapter;

use crate::{context::FlaggerContext, error::FlaggerError};

/// Eport core flagger API methods.
/// The struct is the basic building block for a flagger as a service system.
/// It directly interracts with the underlying database system, which at the time of writing
/// is backed by MongoDB.
///
/// All public members are application controllers; these implements specific APIs depending on the
/// application scope.
pub struct Flagger {
    pub(crate) database: Box<dyn DatabaseAdapter>,
}

pub struct FlaggerBuilder<D>
where
    D: DatabaseAdapter,
{
    database: Option<Box<D>>,
}

impl Flagger {
    /// Initialize and return a builder to help with the configuration.
    pub fn builder<D>() -> FlaggerBuilder<D>
    where
        D: DatabaseAdapter,
    {
        FlaggerBuilder { database: None }
    }

    /// Initialize and return the feature flag context, used to interact with
    /// all API methods.
    pub fn context(&self) -> FlaggerContext {
        FlaggerContext
    }

    #[cfg(feature = "implement-controller")]
    pub fn database(&self) -> &Box<dyn DatabaseAdapter> {
        &self.database
    }
}

impl<D> FlaggerBuilder<D>
where
    D: DatabaseAdapter + 'static,
{
    pub async fn build(&mut self) -> Result<Flagger, FlaggerError> {
        let database = self.database.take().unwrap();

        Ok(Flagger { database })
    }

    pub fn with_database(&mut self, database: Box<D>) -> &mut Self {
        self.database.replace(database);

        self
    }
}

#[cfg(test)]
mod tests {
    use flagger_database_adapter::tests::InMemoryDatabaseAdapter;

    use super::*;

    #[tokio::test]
    async fn it_build_from_builder() -> Result<(), FlaggerError> {
        // given
        let _flagger = Flagger::builder()
            .with_database(Box::new(InMemoryDatabaseAdapter::default()))
            // when
            .build()
            .await?;

        // then
        Ok(())
    }
}
