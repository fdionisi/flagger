use flagger_entities::feature;
use utoipa::OpenApi;

use crate::management_service;

#[derive(OpenApi)]
#[openapi(
    handlers(
        management_service::create_feature,
        management_service::list_features,
        management_service::enable_feature,
        management_service::disable_feature,
    ),
    components(feature::Feature, feature::FeatureInput, feature::FeatureKind)
)]
pub struct ApiDocs;
