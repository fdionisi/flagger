use std::collections::BTreeMap;

use jwt::{Header, Token};

pub type Identity = Token<Header, BTreeMap<String, String>, String>;

/// Interface with the external identity pool.
pub trait IdentityProvider {
    /// Validates an input token and retuns its content.
    fn verify(token: String) -> Identity;
}
