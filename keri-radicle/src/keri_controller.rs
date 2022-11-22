use keri_git::gitdb::GitStorageDatabase;

use git_storage::{Write, Commit};
use radicle_git_types::reference::Reference;
use keri::keys::{PrivateKey, PublicKey};

use crate::keri_identity::KeriIdentity;

pub struct KeriController<'k> {
    identity: Option<KeriIdentity>,
    storage: GitStorageDatabase<'k>,
}

impl<'k> KeriController<'k> {
    pub fn new(storage: &'k Write, identity: Option<KeriIdentity>) -> Self {
        Self {
            storage: GitStorageDatabase::open(storage).unwrap(),
            identity: identity,
        }
    }

    /// Start a new identity and assign it to the Controller
    /// 
    pub fn Inception(&mut self, pu_key: PublicKey, pr_key: PrivateKey) {
        todo!()
    }

    /// Validate a signed ref based on the KERI identity
    /// 
    /// The Controller will use the stored Identity in the signed ref (*not* the controller identity)
    /// to validate its associated KEL hence validating the signature key
    pub fn Validate(signed_ref: Reference<N,R,C>) {
        todo!()
    }

    /// Create a SignedRef for a given commit (usually the latest HEAD)
    /// 
    /// The Controller will use the identity provided 
    pub fn Sign(commit: Commit) -> Reference<N, R, C> {
        todo!()
    }
}