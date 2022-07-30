use flagger_entities::feature;
use utoipa::OpenApi;

use crate::services;

#[derive(OpenApi)]
#[openapi(
    handlers(
        services::consumer_service::is_feature_enabled,
        services::management_service::create_feature,
        services::management_service::list_features,
        services::management_service::enable_feature,
        services::management_service::disable_feature,
    ),
    components(feature::Feature, feature::FeatureInput, feature::FeatureKind)
)]
pub struct ApiDocs;
