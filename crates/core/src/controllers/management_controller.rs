use flagger_entities::feature::{Feature, FeatureInput};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_document};

use crate::{context::FlaggerContext, Flagger, FlaggerError};

#[async_trait::async_trait]
pub trait ManagementController {
    async fn create_feature(
        &self,
        context: &FlaggerContext,
        input: &FeatureInput,
    ) -> Result<Feature, FlaggerError>;

    async fn list_features(&self, context: &FlaggerContext) -> Result<Vec<Feature>, FlaggerError>;

    async fn toggle_feature(
        &self,
        context: &FlaggerContext,
        feature: &ObjectId,
        enabled: bool,
    ) -> Result<Feature, FlaggerError>;
}

#[async_trait::async_trait]
impl ManagementController for Flagger {
    async fn create_feature(
        &self,
        context: &FlaggerContext,
        input: &FeatureInput,
    ) -> Result<Feature, FlaggerError> {
        context.authenticated()?;

        let result = self
            .database
            .collection::<Feature>()
            .insert_one(to_document(&input)?, None)
            .await?;

        let inserted_id = result
            .inserted_id
            .as_object_id()
            .ok_or(FlaggerError::NotFound)?;

        Ok(self
            .database
            .collection_with_type::<Feature>()
            .find_one(doc! { "_id": inserted_id }, None)
            .await?
            .ok_or(FlaggerError::NotFound)?)
    }

    async fn list_features(&self, context: &FlaggerContext) -> Result<Vec<Feature>, FlaggerError> {
        context.authenticated()?;

        Ok(self
            .database
            .collection_with_type::<Feature>()
            .find(doc! {}, None)
            .await?
            .try_collect()
            .await?)
    }

    async fn toggle_feature(
        &self,
        context: &FlaggerContext,
        feature: &ObjectId,
        enabled: bool,
    ) -> Result<Feature, FlaggerError> {
        context.authenticated()?;

        let result = self
            .database
            .collection::<Feature>()
            .update_one(
                doc! { "_id": &feature },
                doc! { "$set": { "enabled": enabled } },
                None,
            )
            .await?;

        if result.matched_count == 0 {
            return Err(FlaggerError::NotFound);
        }

        Ok(self
            .database
            .collection_with_type::<Feature>()
            .find_one(doc! { "_id": &feature }, None)
            .await?
            .ok_or(FlaggerError::NotFound)?)
    }
}

#[cfg(test)]
mod tests {
    use flagger_entities::feature::{FeatureInput, FeatureKind};

    use crate::{error::FlaggerError, tests::test_flagger};

    use super::ManagementController;

    #[tokio::test]
    async fn it_create_new_feature() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger("core_management_controller").await?;
        let context = flagger.context();
        let input = FeatureInput {
            name: "create feature".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };

        // when
        let feature = flagger.create_feature(&context, &input).await?;

        // then
        assert_eq!(feature.name, input.name);
        assert_eq!(feature.description, input.description);
        assert_eq!(feature.kind, input.kind);
        assert_eq!(feature.enabled, false);

        Ok(())
    }

    #[tokio::test]
    async fn it_list_inserted_features() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger("core_management_controller_list").await?;
        let context = flagger.context();
        let input = FeatureInput {
            name: "listed feature".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };

        // when
        let empty_features = flagger.list_features(&context).await?;
        flagger.create_feature(&context, &input).await?;
        let one_feature_list = flagger.list_features(&context).await?;

        // then
        assert!(empty_features.is_empty());
        assert_eq!(one_feature_list.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn it_enables_a_feature() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger("core_management_controller").await?;
        let context = flagger.context();
        let input = FeatureInput {
            name: "enable feature".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };
        let feature = flagger.create_feature(&context, &input).await?;

        // when
        let updated_feature = flagger.toggle_feature(&context, &feature.id, true).await?;

        // then
        assert_eq!(feature.name, updated_feature.name);
        assert_eq!(feature.enabled, false);
        assert_eq!(updated_feature.enabled, true);

        Ok(())
    }

    #[tokio::test]
    async fn it_disables_a_feature() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger("core_management_controller").await?;
        let context = flagger.context();
        let input = FeatureInput {
            name: "disable feature".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };
        let feature = flagger.create_feature(&context, &input).await?;
        flagger.toggle_feature(&context, &feature.id, true).await?;

        // when
        let disabled_feature = flagger.toggle_feature(&context, &feature.id, false).await?;

        // then
        assert_eq!(disabled_feature.enabled, false);

        Ok(())
    }
}
