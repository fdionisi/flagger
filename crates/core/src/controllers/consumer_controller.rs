use flagger_entities::feature::Feature;
use mongodb::bson::doc;

use crate::{context::FlaggerContext, Flagger, FlaggerError};

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

        let feature = self
            .database
            .collection_with_type::<Feature>()
            .find_one(doc! { "name": feature_name }, None)
            .await?;

        Ok(feature.map(|f| f.enabled).unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::FlaggerError, tests::test_flagger};

    use super::ConsumerController;

    #[tokio::test]
    async fn it_doesnt_fail_for_missing_feature() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger("core_consumer_controller").await?;
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
