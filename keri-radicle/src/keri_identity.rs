use keri::{database::EventDatabase, prefix::{AttachedSignaturePrefix, BasicPrefix, IdentifierPrefix, Prefix, SelfAddressingPrefix, SelfSigningPrefix}};

pub struct KeriIdentity {
    prefix: SelfAddressingPrefix,
}

impl KeriIdentity {
    pub fn prefix(&self) -> &SelfAddressingPrefix {
        &self.prefix
    }
}