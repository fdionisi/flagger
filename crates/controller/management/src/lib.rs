use flagger_core::{Flagger, FlaggerContext, FlaggerError};
use flagger_entities::feature::{Feature, FeatureInput};

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
        feature: &String,
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

        Ok(self.database().insert_one(input.clone()).await)
    }

    async fn list_features(&self, context: &FlaggerContext) -> Result<Vec<Feature>, FlaggerError> {
        context.authenticated()?;

        Ok(self.database().list().await)
    }

    async fn toggle_feature(
        &self,
        context: &FlaggerContext,
        feature: &String,
        enabled: bool,
    ) -> Result<Feature, FlaggerError> {
        context.authenticated()?;

        self.database()
            .toggle_enabled(feature.to_string(), enabled)
            .await;

        Ok(self
            .database()
            .read_by_name(feature.to_string())
            .await
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use flagger_entities::feature::{FeatureInput, FeatureKind};

    use flagger_core::{tests::test_flagger, FlaggerError};

    use super::ManagementController;

    #[tokio::test]
    async fn it_create_new_feature() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger().await?;
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
        let flagger = test_flagger().await?;
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
        let flagger = test_flagger().await?;
        let context = flagger.context();
        let input = FeatureInput {
            name: "enable feature".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };
        let feature = flagger.create_feature(&context, &input).await?;

        // when
        let updated_feature = flagger
            .toggle_feature(&context, &feature.name, true)
            .await?;

        // then
        assert_eq!(feature.name, updated_feature.name);
        assert_eq!(feature.enabled, false);
        assert_eq!(updated_feature.enabled, true);

        Ok(())
    }

    #[tokio::test]
    async fn it_disables_a_feature() -> Result<(), FlaggerError> {
        // given
        let flagger = test_flagger().await?;
        let context = flagger.context();
        let input = FeatureInput {
            name: "disable feature".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };
        let feature = flagger.create_feature(&context, &input).await?;
        flagger
            .toggle_feature(&context, &feature.name, true)
            .await?;

        // when
        let disabled_feature = flagger
            .toggle_feature(&context, &feature.name, false)
            .await?;

        // then
        assert_eq!(disabled_feature.enabled, false);

        Ok(())
    }
}
