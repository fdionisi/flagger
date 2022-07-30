use actix_web::{get, web, HttpResponse};
use flagger_consumer_controller::ConsumerController;

use crate::context::ApiContext;

/// Evaluate if a feature is enabled.
///
/// The± enpoint always return either `true` or `false`. If a target feature does not exists, the
/// enxpoint will return `false`.
#[utoipa::path(
    get,
    path = "/consumer/{feature_name}/is-enabled",
    responses(
        (status = 200, description = "The enable status of the feature.", body = bool),
    )
)]
#[get("/consumer/{feature_name}/is-enabled")]
pub async fn is_feature_enabled(
    api_context: web::Data<ApiContext>,
    feature_name: web::Path<String>,
) -> HttpResponse {
    let flagger_context = api_context.flagger.context();

    match api_context
        .flagger
        .is_enabled(&flagger_context, feature_name.into_inner())
        .await
    {
        Ok(feature) => HttpResponse::Ok().json(feature),
        _ => panic!("expr"),
    }
}
