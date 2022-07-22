use crate::{
    context::FlaggerContext,
    database::{Database, DatabaseBuilder},
    error::FlaggerError,
};

/// Eport core flagger API methods.
/// The struct is the basic building block for a flagger as a service system.
/// It directly interracts with the underlying database system, which at the time of writing
/// is backed by MongoDB.
///
/// All public members are application controllers; these implements specific APIs depending on the
/// application scope.
pub struct Flagger {
    pub(crate) database: Database,
}

pub struct FlaggerBuilder {
    database_builder: DatabaseBuilder,
}

impl Flagger {
    /// Initialize and return a builder to help with the configuration.
    pub fn builder() -> FlaggerBuilder {
        FlaggerBuilder {
            database_builder: Database::builder(),
        }
    }

    /// Initialize and return the feature flag context, used to interact with
    /// all API methods.
    pub fn context(&self) -> FlaggerContext {
        FlaggerContext
    }
}

impl FlaggerBuilder {
    pub async fn build(&mut self) -> Result<Flagger, FlaggerError> {
        let database = self.database_builder.build().await?;

        Ok(Flagger { database })
    }

    pub fn with_database_url<S>(&mut self, url: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.database_builder.with_database_url(url);

        self
    }

    pub fn with_database_name<S>(&mut self, name: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.database_builder.with_database_name(name);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_build_from_builder() -> Result<(), FlaggerError> {
        // given
        let _flagger = Flagger::builder()
            .with_database_name("flagger_test")
            .with_database_url(
                std::env::var("FLAGGER_DATABASE_URL").expect("FLAGGER_DATABASE_URL to be set"),
            )
            // when
            .build()
            .await?;

        // then
        Ok(())
    }
}
