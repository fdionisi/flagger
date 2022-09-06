use actix_http::StatusCode;
use actix_web::{get, post, web, HttpResponse};
use flagger_entities::feature::FeatureInput;
use flagger_management_controller::ManagementController;

use crate::context::ManagementContext;

/// Create a new feature.
///
/// There are multiple lines here. I just want to understand how it's interpreted by utoipa.
#[utoipa::path(
    post,
    request_body = FeatureInput,
    path = "/management/feature",
    responses(
        (status = 201, description = "Feature created successfully.", body = Feature),
    )
)]
#[post("/management/feature")]
pub async fn create_feature(
    api_context: web::Data<ManagementContext>,
    input: web::Json<FeatureInput>,
) -> HttpResponse {
    let flagger_context = api_context.flagger.context();

    match api_context
        .flagger
        .create_feature(&flagger_context, &input.0)
        .await
    {
        Ok(feature) => HttpResponse::Ok().status(StatusCode::CREATED).json(feature),
        _ => panic!("expr"),
    }
}

#[utoipa::path(
    get,
    path = "/management/feature",
    responses(
        (status = 200, description = "List all features.", body = [Feature]),
    )
)]
#[get("/management/feature")]
pub async fn list_features(api_context: web::Data<ManagementContext>) -> HttpResponse {
    let flagger_context = api_context.flagger.context();

    match api_context.flagger.list_features(&flagger_context).await {
        Ok(features) => HttpResponse::Ok().json(features),
        _ => panic!("expr"),
    }
}

#[utoipa::path(
    post,
    path = "/management/feature/{id}/enable",
    params(
        ("id", description = "Unique id for a feature")
    ),
    responses(
        (status = 200, description = "Feature successfully updated.", body = Feature),
    )
)]
#[post("/management/feature/{id}/enable")]
pub async fn enable_feature(
    api_context: web::Data<ManagementContext>,
    feature: web::Path<String>,
) -> HttpResponse {
    let flagger_context = api_context.flagger.context();

    match api_context
        .flagger
        .toggle_feature(&flagger_context, &feature.to_string(), true)
        .await
    {
        Ok(feature) => HttpResponse::Ok().json(feature),
        _ => panic!("expr"),
    }
}

#[utoipa::path(
    post,
    path = "/management/feature/{id}/disable",
    params(
        ("id", description = "Unique id for a feature")
    ),
    responses(
        (status = 200, description = "Feature successfully updated.", body = Feature),
    )
)]
#[post("/management/feature/{id}/disable")]
pub async fn disable_feature(
    api_context: web::Data<ManagementContext>,
    feature: web::Path<String>,
) -> HttpResponse {
    let flagger_context = api_context.flagger.context();

    match api_context
        .flagger
        .toggle_feature(&flagger_context, &feature.to_string(), false)
        .await
    {
        Ok(feature) => HttpResponse::Ok().json(feature),
        _ => panic!("expr"),
    }
}

#[cfg(test)]
mod tests {
    use flagger_core::FlaggerError;
    use flagger_entities::feature::{Feature, FeatureInput, FeatureKind};

    use crate::{
        test_management_service,
        tests::{call_and_read_body_json, TestRequest},
    };

    #[tokio::test]
    async fn it_creates_a_feature() -> Result<(), FlaggerError> {
        // given
        let api_server = test_management_service!();
        let input = FeatureInput {
            name: "feature from api 1".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };
        let request = TestRequest::post()
            .uri("/management/feature")
            .set_json(&input)
            .to_request();

        // when
        let response: Feature = call_and_read_body_json(&api_server, request).await;

        // then
        assert_eq!(response.name, input.name);
        assert_eq!(response.description, input.description);
        assert_eq!(response.kind, FeatureKind::KillSwitch);
        assert_eq!(response.enabled, false);

        Ok(())
    }

    #[tokio::test]
    async fn it_list_inserted_features() -> Result<(), FlaggerError> {
        // given
        let api_server = test_management_service!();
        let input = FeatureInput {
            name: "feature from api 1".into(),
            description: None,
            kind: FeatureKind::KillSwitch,
        };

        // when
        let empty_features: Vec<Feature> = call_and_read_body_json(
            &api_server,
            TestRequest::get().uri("/management/feature").to_request(),
        )
        .await;
        let _: Feature = call_and_read_body_json(
            &api_server,
            TestRequest::post()
                .uri("/management/feature")
                .set_json(&input)
                .to_request(),
        )
        .await;
        let one_feature_list: Vec<Feature> = call_and_read_body_json(
            &api_server,
            TestRequest::get().uri("/management/feature").to_request(),
        )
        .await;

        // then
        assert!(empty_features.is_empty());
        assert_eq!(one_feature_list.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn it_enables_a_feature() -> Result<(), FlaggerError> {
        // given
        let api_server = test_management_service!();
        let feature: Feature = {
            let input = FeatureInput {
                name: "enable-feature-from-API".into(),
                description: None,
                kind: FeatureKind::KillSwitch,
            };

            let request = TestRequest::post()
                .uri("/management/feature")
                .set_json(&input)
                .to_request();

            call_and_read_body_json(&api_server, request).await
        };
        println!("/management/feature/{}/enable", feature.name);
        let request = TestRequest::post()
            .uri(&format!("/management/feature/{}/enable", feature.name))
            .to_request();

        // when
        let response: Feature = call_and_read_body_json(&api_server, request).await;

        // then
        assert_eq!(response.enabled, true);

        Ok(())
    }

    #[tokio::test]
    async fn it_disables_a_feature() -> Result<(), FlaggerError> {
        // given
        let api_server = test_management_service!();
        let feature: Feature = {
            let input = FeatureInput {
                name: "disable-feature_from-API".into(),
                description: None,
                kind: FeatureKind::KillSwitch,
            };

            let request = TestRequest::post()
                .uri("/management/feature")
                .set_json(&input)
                .to_request();

            let feature: Feature = call_and_read_body_json(&api_server, request).await;

            let request = TestRequest::post()
                .uri(&format!("/management/feature/{}/enable", &feature.name))
                .set_json(&input)
                .to_request();

            call_and_read_body_json(&api_server, request).await
        };
        let request = TestRequest::post()
            .uri(&format!("/management/feature/{}/disable", feature.name))
            .to_request();

        // when
        let response: Feature = call_and_read_body_json(&api_server, request).await;

        // then
        assert_eq!(response.enabled, false);

        Ok(())
    }
}
