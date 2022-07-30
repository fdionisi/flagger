use flagger_entities::feature::{Feature, FeatureInput};

#[async_trait::async_trait]
pub trait DatabaseAdapter: Sync + Send {
    async fn insert_one(&self, input: FeatureInput) -> Feature;

    async fn read_by_name(&self, name: String) -> Option<Feature>;

    async fn list(&self) -> Vec<Feature>;

    async fn toggle_enabled(&self, name: String, is_enabled: bool) -> bool;
}

#[cfg(any(test, feature = "test-suite"))]
pub mod tests {
    use std::{collections::BTreeMap, sync::Mutex};

    use flagger_entities::feature::Feature;

    use super::*;

    #[derive(Default)]
    pub struct InMemoryDatabaseAdapter(Mutex<BTreeMap<String, Feature>>);

    #[async_trait::async_trait]
    impl DatabaseAdapter for InMemoryDatabaseAdapter {
        async fn insert_one(&self, input: FeatureInput) -> Feature {
            let mut inner = self.0.lock().unwrap();
            let feature = Feature {
                name: input.name.clone(),
                kind: input.kind,
                description: input.description,
                enabled: false,
            };

            inner.insert(input.name, feature.clone());

            feature
        }

        async fn read_by_name(&self, name: String) -> Option<Feature> {
            let inner = self.0.lock().unwrap();
            inner
                .clone()
                .into_iter()
                .find(|(n, _)| n == &name)
                .map(|(_, f)| f)
                .clone()
        }

        async fn list(&self) -> Vec<Feature> {
            let inner = self.0.lock().unwrap();
            inner.clone().into_iter().map(|(_, f)| f).collect()
        }

        async fn toggle_enabled(&self, name: String, is_enabled: bool) -> bool {
            let mut inner = self.0.lock().unwrap();

            if !inner.contains_key(&name) {
                return false;
            }

            inner.entry(name).and_modify(|f| f.enabled = is_enabled);

            true
        }
    }
}
