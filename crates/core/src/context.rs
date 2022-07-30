pub struct FlaggerContext;

impl FlaggerContext {
    #[cfg(feature = "implement-controller")]
    pub fn authenticated(&self) -> Result<(), crate::FlaggerError> {
        Ok(())
    }

    #[cfg(feature = "implement-controller")]
    pub fn consumer(&self) -> Result<(), crate::FlaggerError> {
        Ok(())
    }
}
