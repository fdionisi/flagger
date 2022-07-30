use rocksdb::Error as RocksDBError;

#[derive(Debug)]
pub enum RocksDBAdapterError {
    Builder,
    RocksDB(RocksDBError),
}

impl From<RocksDBError> for RocksDBAdapterError {
    fn from(error: RocksDBError) -> Self {
        RocksDBAdapterError::RocksDB(error)
    }
}
