use actix_web::{get, web, HttpResponse};
use flagger_core::controllers::ConsumerController;

use crate::context::ApiContext;

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
