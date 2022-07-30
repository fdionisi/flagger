use flagger_database_adapter::DatabaseAdapter;
use flagger_entities::feature::{Feature, FeatureInput};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Database,
};

pub const FEATURE_COLLECTION_NAME: &'static str = "features";

pub struct MongoDBAdapter {
    database_name: String,
    client: Client,
}

#[derive(Default)]
pub struct MongoDBAdapterBuilder {
    database_name: Option<String>,
    database_url: Option<String>,
}

impl MongoDBAdapter {
    pub fn builder() -> MongoDBAdapterBuilder {
        MongoDBAdapterBuilder::default()
    }

    fn database(&self) -> Database {
        self.client.database(&self.database_name)
    }

    async fn read_by_attribute(&self, attribute: String, query: String) -> Option<Feature> {
        self.database()
            .collection::<Feature>(FEATURE_COLLECTION_NAME)
            .find_one(doc! { attribute: query }, None)
            .await
            .unwrap()
    }

    async fn new<S>(database_url: S, database_name: S) -> Self
    where
        S: AsRef<str>,
    {
        let mut client_options = ClientOptions::parse(database_url.as_ref()).await.unwrap();
        client_options.app_name = Some("booky".to_string());

        Self {
            database_name: database_name.as_ref().to_string(),
            client: Client::with_options(client_options).unwrap(),
        }
    }
}

#[async_trait::async_trait]
impl DatabaseAdapter for MongoDBAdapter {
    async fn insert_one(&self, input: FeatureInput) -> Feature {
        let database = self.database();

        database
            .collection(FEATURE_COLLECTION_NAME)
            .insert_one(
                doc! {
                    "kind": input.kind.to_string(),
                    "name": input.name.clone(),
                    "description": input.description,
                },
                None,
            )
            .await
            .unwrap();

        self.read_by_name(input.name).await.unwrap()
    }

    async fn read_by_name(&self, feature_name: String) -> Option<Feature> {
        self.read_by_attribute("name".into(), feature_name).await
    }

    async fn list(&self) -> Vec<Feature> {
        self.database()
            .collection::<Feature>(FEATURE_COLLECTION_NAME)
            .find(doc! {}, None)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
    }

    async fn toggle_enabled(&self, name: String, is_enabled: bool) -> bool {
        let _id = ObjectId::parse_str(name).unwrap();
        let result = self
            .database()
            .collection::<Feature>(FEATURE_COLLECTION_NAME)
            .update_one(
                doc! { "_id": &_id },
                doc! { "$set": { "enabled": is_enabled } },
                None,
            )
            .await
            .unwrap();

        return result.matched_count == 0;
    }
}

impl MongoDBAdapterBuilder {
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

    pub async fn build(&mut self) -> MongoDBAdapter {
        let database_url = self.database_url.as_ref().unwrap();

        let database_name = self.database_name.as_ref().unwrap();

        MongoDBAdapter::new(database_url, database_name).await
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::error::DatabaseError;

//     use super::*;

//     #[tokio::test]
//     async fn build_from_str() -> Result<(), DatabaseError> {
//         let _ = Database::builder()
//             .with_database_url("mongodb://127.0.0.1:27017")
//             .with_database_name("test")
//             .build()
//             .await?;

//         Ok(())
//     }
// }
