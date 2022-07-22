use std::sync::Arc;

use flagger_core::Flagger;

#[derive(Clone)]
pub struct ApiContext {
    pub flagger: Arc<Flagger>,
}
