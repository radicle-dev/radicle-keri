use git_storage::signature::UserInfo;

use std::path::{Path, PathBuf};

use git_storage::Write;
use git_storage::write::error;

use keri::{database::EventDatabase, prefix};

pub enum KeriError {
    GitInitError,
    IOError,
    NoKeriDir,
    KeriError, //TODO Expand on these errors as we write the methods
}

impl From<error::Init> for KeriError {
    fn from(_: error::Init) -> Self {
        KeriError::GitInitError
    }
}

impl From<std::io::Error> for KeriError {
    fn from(_: std::io::Error) -> Self {
        KeriError::IOError
    }
}

pub struct GitStorageDatabase {
    storage: Write,
}

impl GitStorageDatabase {
    pub fn open<P: AsRef<Path>>(path: P, info: UserInfo) -> Result<Self, KeriError> {
        if !Self::check_keri_dir(&path) {
            return Err(KeriError::NoKeriDir)
        }

        Ok(Self {
            storage: Write::open(path, info)?
        })
    }
    
    pub fn init<P: AsRef<Path>>(path: P, info: UserInfo) -> Result<Self, KeriError> {
        let keri_path = Self::keri_dir(&path);

        std::fs::create_dir_all(keri_path)?;

        Self::open(path, info)
    }

    fn keri_dir<P: AsRef<Path>>(path: &P) -> PathBuf {
        let mut keri_path = PathBuf::new();
        keri_path.push(&path);
        // TODO: Take this path out to a constant
        keri_path.push("refs/rad/");
        
        keri_path
    }

    fn check_keri_dir<P: AsRef<Path>>(path: &P) -> bool {
        let keri_path = Self::keri_dir(&path);
        keri_path.is_dir()
    }
}

impl EventDatabase for GitStorageDatabase {
    type Error = KeriError;

    fn last_event_at_sn(
        &self,
        pref: &prefix::IdentifierPrefix,
        sn: u64,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        todo!()
    }

    fn get_kerl(&self, id: &prefix::IdentifierPrefix) -> Result<Option<Vec<u8>>, Self::Error> {
        todo!()
    }

    fn log_event(
        &self,
        prefix: &prefix::IdentifierPrefix,
        dig: &prefix::SelfAddressingPrefix,
        raw: &[u8],
        sigs: &[prefix::AttachedSignaturePrefix],
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn finalise_event(
        &self,
        prefix:&prefix::IdentifierPrefix,
        sn: u64,
        dig:&prefix::SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_partially_signed_event(
        &self,
        pref:&prefix::IdentifierPrefix,
        sn: u64,
        dig:&prefix::SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_out_of_order_event(
        &self,
        pref:&prefix::IdentifierPrefix,
        sn: u64,
        dig:&prefix::SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn likely_duplicitous_event(
        &self,
        pref:&prefix::IdentifierPrefix,
        sn: u64,
        dig:&prefix::SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn duplicitous_event(
        &self,
        pref:&prefix::IdentifierPrefix,
        sn: u64,
        dig:&prefix::SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_nt_receipt_for_event(
        &self,
        pref:&prefix::IdentifierPrefix,
        dig:&prefix::SelfAddressingPrefix,
        signer:&prefix::BasicPrefix,
        sig:&prefix::SelfSigningPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_t_receipt_for_event(
        &self,
        pref:&prefix::IdentifierPrefix,
        dig:&prefix::SelfAddressingPrefix,
        signer:&prefix::IdentifierPrefix,
        sig:&prefix::AttachedSignaturePrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_nt_receipt(
        &self,
        pref:&prefix::IdentifierPrefix,
        dig:&prefix::SelfAddressingPrefix,
        signer:&prefix::BasicPrefix,
        sig:&prefix::SelfSigningPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_t_receipt(
        &self,
        pref:&prefix::IdentifierPrefix,
        dig:&prefix::SelfAddressingPrefix,
        signer:&prefix::IdentifierPrefix,
        sig:&prefix::AttachedSignaturePrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn has_receipt(
        &self,
        pref:&prefix::IdentifierPrefix,
        sn: u64,
        validator:&prefix::IdentifierPrefix,
    ) -> Result<bool, Self::Error> {
        todo!()
    }
}