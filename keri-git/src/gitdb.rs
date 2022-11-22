use git_storage::signature::UserInfo;

use std::path::{Path, PathBuf};

use git_storage::Write;

use keri::{database::EventDatabase, prefix::{AttachedSignaturePrefix, BasicPrefix, IdentifierPrefix, SelfAddressingPrefix, SelfSigningPrefix}};

use crate::keri_store::KeriStore;

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum IO {
        #[error("KERI refs directory does not exist")]
        NoKeriDir,
        #[error(transparent)]
        IOError(#[from] std::io::Error),
        #[error(transparent)]
        InitError(#[from] crate::keri_store::error::Init),
    }

    #[derive(Debug, Error)]
    pub enum KeriError {
        #[error(transparent)]
        KeriError(#[from] crate::keri_store::error::KeriError),
    }
}


pub struct GitStorageDatabase<'k> {
    storage: KeriStore<'k>,
}

impl<'k> GitStorageDatabase<'k> {
    pub fn open(storage: &'k Write) -> Result<Self, error::IO> {
        Ok(Self {
            storage: KeriStore::open(storage)?,
        })
    }
    
    // Use UserInfo to derive applicable namespace ?
    pub fn init<P: AsRef<Path>>(path: P, info: UserInfo) -> Result<(), error::IO> {
        let keri_path = Self::keri_dir(&path);

        std::fs::create_dir_all(keri_path)?;

        Ok(())
    }

    fn keri_dir<P: AsRef<Path>>(path: &P) -> PathBuf {
        let mut keri_path = PathBuf::new();
        keri_path.push(&path);
        // TODO: Take this path out to a constant
        keri_path.push("refs/rad/");
        
        keri_path
    }

    pub fn check_keri_dir<P: AsRef<Path>>(path: &P) -> bool {
        let keri_path = Self::keri_dir(&path);
        keri_path.is_dir()
    }
}

impl<'k> EventDatabase for GitStorageDatabase<'k> {
    type Error = crate::keri_store::error::KeriError;

    fn last_event_at_sn(
        &self,
        pref: &IdentifierPrefix, // TODO: map identifier to namespace
        sn: u64,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        let event = self.storage.log_entry_sn(sn).unwrap();

        match event {
            Some(e) => {
                let mut event_bytes = Vec::new();
                ciborium::ser::into_writer(&e, &mut event_bytes).unwrap();
                Ok(Some(event_bytes))
            },
            None => Ok(None)
        }
    }

    fn get_kerl(&self, id: &IdentifierPrefix) -> Result<Option<Vec<u8>>, Self::Error> {
        todo!()
    }

    fn log_event(
        &self,
        prefix: &IdentifierPrefix,
        dig: &SelfAddressingPrefix,
        raw: &[u8],
        sigs: &[AttachedSignaturePrefix],
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn finalise_event(
        &self,
        prefix: &IdentifierPrefix,
        sn: u64,
        dig:&SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_partially_signed_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig:&SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_out_of_order_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig:&SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn likely_duplicitous_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig:&SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn duplicitous_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig:&SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_nt_receipt_for_event(
        &self,
        pref: &IdentifierPrefix,
        dig:&SelfAddressingPrefix,
        signer:&BasicPrefix,
        sig:&SelfSigningPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_t_receipt_for_event(
        &self,
        pref: &IdentifierPrefix,
        dig:&SelfAddressingPrefix,
        signer: &IdentifierPrefix,
        sig:&AttachedSignaturePrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_nt_receipt(
        &self,
        pref: &IdentifierPrefix,
        dig: &SelfAddressingPrefix,
        signer: &BasicPrefix,
        sig: &SelfSigningPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_t_receipt(
        &self,
        pref: &IdentifierPrefix,
        dig:&SelfAddressingPrefix,
        signer: &IdentifierPrefix,
        sig:&AttachedSignaturePrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn has_receipt(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        validator: &IdentifierPrefix,
    ) -> Result<bool, Self::Error> {
        todo!()
    }
}