mod error;

use std::path::{Path, PathBuf};

use error::RocksDBAdapterError;
use flagger_database_adapter::DatabaseAdapter;
use flagger_entities::feature::{Feature, FeatureInput};
use rocksdb::DB as RocksDB;
pub use rocksdb::{DBCompressionType as RocksDBCompressionType, Options as RocksDBOptions};

pub struct RocksDBAdapter(RocksDB);

#[derive(Default)]
pub struct RocksDBAdapterBuilder {
    database_path: Option<PathBuf>,
    options: Option<RocksDBOptions>,
}

impl RocksDBAdapter {
    pub fn builder() -> RocksDBAdapterBuilder {
        RocksDBAdapterBuilder::default()
    }
}

#[async_trait::async_trait]
impl DatabaseAdapter for RocksDBAdapter {
    async fn insert_one(&self, input: FeatureInput) -> Feature {
        let feature = Feature {
            name: input.name.clone(),
            kind: input.kind,
            description: input.description,
            enabled: false,
        };

        self.0.put(input.name, feature.ser()).unwrap();

        feature
    }

    async fn read_by_name(&self, name: String) -> Option<Feature> {
        let feature = self.0.get(name).unwrap();
        feature.map(Feature::de)
    }

    async fn list(&self) -> Vec<Feature> {
        self.0
            .iterator(rocksdb::IteratorMode::Start)
            .map(|(_, v)| Feature::de(v))
            .collect()
    }

    async fn toggle_enabled(&self, name: String, is_enabled: bool) -> bool {
        let f = self.read_by_name(name.clone()).await;
        match f {
            None => false,
            Some(mut f) => {
                f.enabled = is_enabled;
                self.0.put(name.clone(), f.ser()).unwrap();
                true
            }
        }
    }
}

impl RocksDBAdapterBuilder {
    pub fn build(&mut self) -> Result<RocksDBAdapter, RocksDBAdapterError> {
        let options = self.options.take().unwrap_or(RocksDBOptions::default());
        let database_path = self
            .database_path
            .take()
            .ok_or(RocksDBAdapterError::Builder)?;

        Ok(RocksDBAdapter(RocksDB::open(&options, database_path)?))
    }

    pub fn with_options(&mut self, options: RocksDBOptions) -> &mut Self {
        self.options.replace(options);

        self
    }

    pub fn with_database_path<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.database_path.replace(path.as_ref().into());

        self
    }
}
