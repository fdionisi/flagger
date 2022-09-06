use std::{net::SocketAddr, sync::Arc};

use flagger_core::Flagger;
use tonic::transport::Server as TonicServer;

use crate::{
    consumer::consumer_server::ConsumerServer as TonicConsumerServer,
    consumer_service::ConsumerService,
};

pub struct ConsumerServer {
    flagger: Arc<Flagger>,
}

pub struct ConsumerServerBuilder {
    flagger: Option<Flagger>,
}

impl ConsumerServer {
    pub fn builder() -> ConsumerServerBuilder {
        ConsumerServerBuilder { flagger: None }
    }

    pub async fn listen(&self, addr: SocketAddr) -> std::io::Result<()> {
        let flagger = self.flagger.clone();
        TonicServer::builder()
            .add_service(TonicConsumerServer::new(ConsumerService::new(flagger)))
            .serve(addr)
            .await
            .unwrap();

        Ok(())
    }
}

impl ConsumerServerBuilder {
    pub fn build(&mut self) -> ConsumerServer {
        ConsumerServer {
            flagger: Arc::new(self.flagger.take().expect("can't build without flagger")),
        }
    }

    pub fn with_flagger(&mut self, flagger: Flagger) -> &mut Self {
        self.flagger = Some(flagger);

        self
    }
}
