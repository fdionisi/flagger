mod context;
mod management_service;
mod openapi;
mod server;

pub use server::ManagementServer;

#[cfg(any(test, feature = "test-suite"))]
pub mod tests {
    pub use actix_web::test::{call_and_read_body_json, TestRequest};

    #[macro_export]
    macro_rules! test_management_service {
        () => {{
            use std::sync::Arc;

            use actix_web::test;
            use flagger_core::tests::test_flagger;

            use crate::ManagementServer;

            let flagger = test_flagger().await?;
            let app = ManagementServer::create_app(Arc::new(flagger));
            test::init_service(app).await
        }};
    }
}
