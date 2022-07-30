use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::Mutex,
};

use flagger_database_adapter::DatabaseAdapter;
use flagger_entities::feature::{Feature, FeatureInput, FeatureKind};
use rusqlite::Connection;

pub const FEATURES_TABLE_NAME: &'static str = "features";

pub struct SQLiteAdapter {
    connection: Mutex<Connection>,
}

#[derive(Default)]
pub struct SQLiteAdapterBuilder {
    path: Option<PathBuf>,
}

impl SQLiteAdapter {
    pub fn builder() -> SQLiteAdapterBuilder {
        SQLiteAdapterBuilder::default()
    }

    async fn new(path: PathBuf) -> Self {
        let connection = Connection::open(path).unwrap();
        connection
            .execute(
                &format!(
                    r#"
                    CREATE TABLE IF NOT EXISTS {}(
                        kind TEXT NOT NULL,
                        name TEXT NOT NULL UNIQUE,
                        description TEXT,
                        enabled BOOLEAN DEFAULT false
                    )"#,
                    FEATURES_TABLE_NAME
                ),
                (),
            )
            .unwrap();

        Self {
            connection: Mutex::new(connection),
        }
    }

    fn read_by_attribute<D: Display>(&self, attribute: String, value: D) -> Option<Feature> {
        let connection = self.connection.lock().unwrap();

        let mut stmt = connection
            .prepare(&format!(
                "SELECT rowid, kind, name, description, enabled FROM {} WHERE {} = '{}'",
                FEATURES_TABLE_NAME, attribute, value
            ))
            .unwrap();

        let s = stmt
            .query_map([], |row| {
                let kind: String = row.get(1).unwrap();
                Ok(Feature {
                    kind: FeatureKind::from(kind),
                    name: row.get(2).unwrap(),
                    description: row.get(3).unwrap_or(None),
                    enabled: row.get(4).unwrap_or(false),
                })
            })
            .unwrap()
            .take(1)
            .map(|v| v.unwrap())
            .collect::<Vec<Feature>>()
            .pop();

        s
    }
}

#[async_trait::async_trait]
impl DatabaseAdapter for SQLiteAdapter {
    async fn insert_one(&self, input: FeatureInput) -> Feature {
        {
            let connection = self.connection.lock().unwrap();
            let result = connection.execute(
                &format!(
                    "INSERT INTO {} (name, kind, description) VALUES (?1, ?2, ?3)",
                    FEATURES_TABLE_NAME
                ),
                (&input.name, input.kind.to_string(), &input.description),
            );

            match result {
                _ => (),
            }
        };

        self.read_by_name(input.name).await.unwrap()
    }

    async fn read_by_name(&self, name: String) -> Option<Feature> {
        self.read_by_attribute("name".into(), name)
    }

    async fn list(&self) -> Vec<Feature> {
        let connection = self.connection.lock().unwrap();

        let mut stmt = connection
            .prepare(&format!(
                "SELECT rowid, kind, name, description, enabled FROM {} ",
                FEATURES_TABLE_NAME
            ))
            .unwrap();

        stmt.query_map([], |row| {
            let kind: String = row.get(1).unwrap();
            Ok(Feature {
                kind: FeatureKind::from(kind),
                name: row.get(2).unwrap(),
                description: row.get(3).unwrap(),
                enabled: row.get(4).unwrap(),
            })
        })
        .unwrap()
        .map(|v| v.unwrap())
        .collect()
    }

    async fn toggle_enabled(&self, name: String, is_enabled: bool) -> bool {
        let connection = self.connection.lock().unwrap();
        let result = connection.execute(
            &format!(
                "UPDATE {} SET enabled = (?1) WHERE name = (?2)",
                FEATURES_TABLE_NAME
            ),
            (&is_enabled, name),
        );

        match result {
            Ok(_) => true,
            _ => false,
        }
    }
}

impl SQLiteAdapterBuilder {
    pub fn with_database_path<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.path.replace(path.as_ref().into());

        self
    }

    pub async fn build(&mut self) -> SQLiteAdapter {
        SQLiteAdapter::new(self.path.take().unwrap()).await
    }
}
