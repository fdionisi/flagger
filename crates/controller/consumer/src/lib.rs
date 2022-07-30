use flagger_core::{Flagger, FlaggerContext, FlaggerError};

#[async_trait::async_trait]
pub trait ConsumerController {
    async fn is_enabled(
        &self,
        context: &FlaggerContext,
        feature_name: String,
    ) -> Result<bool, FlaggerError>;
}

#[async_trait::async_trait]
impl ConsumerController for Flagger {
    async fn is_enabled(
        &self,
        context: &FlaggerContext,
        feature_name: String,
    ) -> Result<bool, FlaggerError> {
        context.consumer()?;

        let feature = self.database().read_by_name(feature_name).await;

        Ok(feature.map(|f| f.enabled).unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use flagger_core::{tests::test_flagger, FlaggerError};

    use super::ConsumerController;

    #[tokio::test]
    async fn it_doesnt_fail_for_missing_feature() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger().await?;
        let context = flagger.context();

        // when
        let is_enabled = flagger
            .is_enabled(&context, "missing feature".into())
            .await?;

        // then
        assert_eq!(is_enabled, false);

        Ok(())
    }
}
