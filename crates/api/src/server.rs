use std::{net::ToSocketAddrs, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    http::KeepAlive,
    middleware::Logger,
    web, App, HttpServer,
};
use flagger_core::Flagger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{context::ApiContext, openapi, services};

pub struct ApiServer {
    flagger: Arc<Flagger>,
}

pub struct ApiServerBuilder {
    flagger: Option<Flagger>,
}

impl ApiServer {
    pub fn builder() -> ApiServerBuilder {
        ApiServerBuilder { flagger: None }
    }

    pub async fn listen<A>(&self, addr: A) -> std::io::Result<()>
    where
        A: ToSocketAddrs,
    {
        let flagger = self.flagger.clone();
        HttpServer::new(move || ApiServer::create_app(flagger.clone()))
            .keep_alive(KeepAlive::Os)
            .bind(addr)?
            .run()
            .await
    }

    pub(crate) fn create_app(
        flagger: Arc<Flagger>,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<impl MessageBody>,
            Config = (),
            InitError = (),
            Error = actix_web::Error,
        >,
    > {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(web::Data::new(ApiContext {
                flagger: flagger.clone(),
            }))
            .service(services::consumer_service::is_feature_enabled)
            .service(services::management_service::create_feature)
            .service(services::management_service::enable_feature)
            .service(services::management_service::disable_feature)
            .service(services::management_service::list_features)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi::ApiDocs::openapi()),
            )
    }
}

impl ApiServerBuilder {
    pub fn build(&mut self) -> ApiServer {
        ApiServer {
            flagger: Arc::new(self.flagger.take().expect("can't build without flagger")),
        }
    }

    pub fn with_flagger(&mut self, flagger: Flagger) -> &mut ApiServerBuilder {
        self.flagger = Some(flagger);

        self
    }
}

#[cfg(test)]
mod tests {
    use flagger_core::{tests::test_flagger, FlaggerError};

    use super::*;

    #[tokio::test]
    async fn it_build_from_builder() -> Result<(), FlaggerError> {
        // given
        let _api_server = ApiServer::builder()
            .with_flagger(test_flagger().await?)
            // when
            .build();

        // then
        Ok(())
    }
}
