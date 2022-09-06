use std::sync::Arc;

use flagger_consumer_controller::ConsumerController;
use flagger_core::Flagger;
use tonic::{Request, Response, Status};

use crate::consumer::{
    consumer_server::Consumer,
    {CheckFeatureRequest, CheckFeatureResponse},
};

#[derive(Clone)]
pub struct ConsumerService {
    flagger: Arc<Flagger>,
}

impl ConsumerService {
    pub fn new(flagger: Arc<Flagger>) -> Self {
        Self { flagger }
    }
}

#[tonic::async_trait]
impl Consumer for ConsumerService {
    async fn check_feature(
        &self,
        request: Request<CheckFeatureRequest>,
    ) -> Result<Response<CheckFeatureResponse>, Status> {
        let context = self.flagger.context();

        Ok(Response::new(CheckFeatureResponse {
            status: self
                .flagger
                .is_enabled(&context, request.into_inner().feature_name)
                .await
                .unwrap(),
        }))
    }
}
