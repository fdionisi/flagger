use flagger_entities::feature;
use utoipa::OpenApi;

use crate::services;

#[derive(OpenApi)]
#[openapi(
    paths(
        services::consumer_service::is_feature_enabled,
        services::management_service::create_feature,
        services::management_service::list_features,
        services::management_service::enable_feature,
        services::management_service::disable_feature,
    ),
    components(schemas(feature::Feature, feature::FeatureInput, feature::FeatureKind))
)]
pub struct ApiDocs;
