use flagger_types::identity_provider::{Identity, IdentityProvider};

pub struct AwsIdentityProvider;

impl IdentityProvider for AwsIdentityProvider {
    fn verify(_token: String) -> Identity {
        todo!()
    }
}
