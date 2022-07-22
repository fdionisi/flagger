use crate::FlaggerError;

pub struct FlaggerContext;

impl FlaggerContext {
    pub(crate) fn authenticated(&self) -> Result<(), FlaggerError> {
        Ok(())
    }

    pub(crate) fn consumer(&self) -> Result<(), FlaggerError> {
        Ok(())
    }
}
