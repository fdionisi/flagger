use std::str::FromStr;

use actix_web::{get, post, web, HttpResponse};
use flagger_core::controllers::ManagementController;
use flagger_entities::{feature::FeatureInput, ObjectId};

use crate::context::ApiContext;

#[post("/management/feature")]
pub async fn create_feature(
    api_context: web::Data<ApiContext>,
    input: web::Json<FeatureInput>,
) -> HttpResponse {
    let flagger_context = api_context.flagger.context();

    match api_context
        .flagger
        .create_feature(&flagger_context, &input.0)
        .await
    {
        Ok(feature) => HttpResponse::Ok().json(feature),
        _ => panic!("expr"),
    }
}

#[get("/management/feature")]
pub async fn list_features(api_context: web::Data<ApiContext>) -> HttpResponse {
    let flagger_context = api_context.flagger.context();

    match api_context.flagger.list_features(&flagger_context).await {
        Ok(features) => HttpResponse::Ok().json(features),
        _ => panic!("expr"),
    }
}

#[post("/management/feature/{id}/enable")]
pub async fn enable_feature(
    api_context: web::Data<ApiContext>,
    feature: web::Path<String>,
) -> HttpResponse {
    let flagger_context = api_context.flagger.context();
    let feature = ObjectId::from_str(&feature).expect("to be an hex ObjectId");

    match api_context
        .flagger
        .toggle_feature(&flagger_context, &feature, true)
        .await
    {
        Ok(feature) => HttpResponse::Ok().json(feature),
        _ => panic!("expr"),
    }
}

#[post("/management/feature/{id}/disable")]
pub async fn disable_feature(
    api_context: web::Data<ApiContext>,
    feature: web::Path<String>,
) -> HttpResponse {
    let flagger_context = api_context.flagger.context();
    let feature = ObjectId::from_str(&feature).expect("to be an hex ObjectId");

    match api_context
        .flagger
        .toggle_feature(&flagger_context, &feature, false)
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
        test_api_server,
        tests::{call_and_read_body_json, TestRequest},
    };

    #[tokio::test]
    async fn it_creates_a_feature() -> Result<(), FlaggerError> {
        // given
        let api_server = test_api_server!("management_feature");
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
        let api_server = test_api_server!("api_management_feature_feature");
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
        let api_server = test_api_server!("management_feature");
        let feature: Feature = {
            let input = FeatureInput {
                name: "enable feature from API".into(),
                description: None,
                kind: FeatureKind::KillSwitch,
            };

            let request = TestRequest::post()
                .uri("/management/feature")
                .set_json(&input)
                .to_request();

            call_and_read_body_json(&api_server, request).await
        };
        let request = TestRequest::post()
            .uri(&format!("/management/feature/{}/enable", feature.id))
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
        let api_server = test_api_server!("management_feature");
        let feature: Feature = {
            let input = FeatureInput {
                name: "enable feature from API".into(),
                description: None,
                kind: FeatureKind::KillSwitch,
            };

            let request = TestRequest::post()
                .uri("/management/feature")
                .set_json(&input)
                .to_request();

            let feature: Feature = call_and_read_body_json(&api_server, request).await;

            let request = TestRequest::post()
                .uri(&format!("/management/feature/{}/enable", &feature.id))
                .set_json(&input)
                .to_request();

            call_and_read_body_json(&api_server, request).await
        };
        let request = TestRequest::post()
            .uri(&format!("/management/feature/{}/disable", feature.id))
            .to_request();

        // when
        let response: Feature = call_and_read_body_json(&api_server, request).await;

        // then
        assert_eq!(response.enabled, false);

        Ok(())
    }
}
