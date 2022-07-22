mod context;
mod server;
mod services;

pub use server::ApiServer;

#[cfg(any(test, feature = "test-suite"))]
pub mod tests {
    pub use actix_web::test::{call_and_read_body_json, TestRequest};

    #[macro_export]
    macro_rules! test_api_server {
        ($x:tt) => {{
            use std::sync::Arc;

            use actix_web::test;
            use flagger_core::tests::test_flagger;

            use crate::ApiServer;

            let flagger = test_flagger($x).await?;
            let app = ApiServer::create_app(Arc::new(flagger));
            test::init_service(app).await
        }};
    }
}
