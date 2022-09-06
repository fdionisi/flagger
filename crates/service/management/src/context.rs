use std::sync::Arc;

use flagger_core::Flagger;

#[derive(Clone)]
pub struct ManagementContext {
    pub flagger: Arc<Flagger>,
}
