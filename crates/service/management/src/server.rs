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

use crate::{context::ManagementContext, management_service, openapi};

pub struct ManagementServer {
    flagger: Arc<Flagger>,
}

pub struct ManagementServerBuilder {
    flagger: Option<Flagger>,
}

impl ManagementServer {
    pub fn builder() -> ManagementServerBuilder {
        ManagementServerBuilder { flagger: None }
    }

    pub async fn listen<A>(&self, addr: A) -> std::io::Result<()>
    where
        A: ToSocketAddrs,
    {
        let flagger = self.flagger.clone();
        HttpServer::new(move || ManagementServer::create_app(flagger.clone()))
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
            .app_data(web::Data::new(ManagementContext {
                flagger: flagger.clone(),
            }))
            .service(management_service::create_feature)
            .service(management_service::enable_feature)
            .service(management_service::disable_feature)
            .service(management_service::list_features)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi::ApiDocs::openapi()),
            )
    }
}

impl ManagementServerBuilder {
    pub fn build(&mut self) -> ManagementServer {
        ManagementServer {
            flagger: Arc::new(self.flagger.take().expect("can't build without flagger")),
        }
    }

    pub fn with_flagger(&mut self, flagger: Flagger) -> &mut ManagementServerBuilder {
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
        let _api_server = ManagementServer::builder()
            .with_flagger(test_flagger().await?)
            // when
            .build();

        // then
        Ok(())
    }
}
