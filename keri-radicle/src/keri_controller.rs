use keri::keri::Keri;
use keri::signer::KeyManager;

use git_storage::Write;

use git_ref_format::RefString;

// TODO Since this code will probably live in `heartwood`, this type should
// be the heartwood SignedRef type. This is just a placeholder.
pub type SignedRef = RefString;

pub struct KeriController<K>
where
    K: KeyManager + 'static,
{
    identity: Keri<K>,
}

impl<K> KeriController<K>
where
    K: KeyManager + 'static,
{
    pub fn new(storage: &Write, key_manager: impl KeyManager) -> Self {
        todo!()
    }

    /// Start a new identity and assign it to the Controller
    pub fn inception(&mut self) {
        todo!()
    }

    /// Validate a signed ref based on the KERI identity
    ///
    /// The Controller will use the stored Identity in the signed ref
    /// (*not* the controller identity)
    /// to validate its associated KEL hence validating the signature key
    pub fn validate(&self, signed_ref: SignedRef) {
        todo!()
    }
}
