use flagger_types::repository::Repository;
use mongodb::{bson::Document, options::ClientOptions, Client, Collection};

use crate::error::FlaggerError;

#[derive(Default)]
pub struct DatabaseBuilder {
    database_name: Option<String>,
    database_url: Option<String>,
}

pub struct Database {
    database_name: String,
    client: Client,
}

impl DatabaseBuilder {
    pub fn with_database_url<S>(&mut self, url: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.database_url = Some(url.into());

        self
    }

    pub fn with_database_name<S>(&mut self, name: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.database_name = Some(name.into());

        self
    }

    pub async fn build(&mut self) -> Result<Database, FlaggerError> {
        let database_url = self
            .database_url
            .as_ref()
            .ok_or(FlaggerError::Builder("url".into()))?;

        let database_name = self
            .database_name
            .as_ref()
            .ok_or(FlaggerError::Builder("name".into()))?;

        Ok(Database::new(database_url, database_name).await?)
    }
}

impl Database {
    pub fn builder() -> DatabaseBuilder {
        DatabaseBuilder::default()
    }

    pub fn collection<R>(&self) -> Collection<Document>
    where
        R: Repository,
    {
        self.client
            .database(&self.database_name)
            .collection(R::name())
    }

    pub fn collection_with_type<R>(&self) -> Collection<R>
    where
        R: Repository,
    {
        self.client
            .database(&self.database_name)
            .collection::<R>(R::name())
    }
}

impl Database {
    async fn new<S>(database_url: S, database_name: S) -> Result<Self, FlaggerError>
    where
        S: AsRef<str>,
    {
        let mut client_options = ClientOptions::parse(database_url.as_ref()).await?;
        client_options.app_name = Some("booky".to_string());

        Ok(Self {
            database_name: database_name.as_ref().to_string(),
            client: Client::with_options(client_options)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::FlaggerError;

    use super::*;

    #[tokio::test]
    async fn build_from_str() -> Result<(), FlaggerError> {
        let _ = Database::builder()
            .with_database_url("mongodb://127.0.0.1:27017")
            .with_database_name("test")
            .build()
            .await?;

        Ok(())
    }
}
