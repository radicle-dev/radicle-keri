use std::path::Path;

use git_ref_format::{RefStr, Qualified};
use git_storage::{Write, Commit};
use git_storage::refdb::Read as _;
use git_storage::odb::Read as _;
use git_storage::signature::UserInfo;

use git2::ObjectType;

use keri::event_message::signed_event_message::TimestampedSignedEventMessage;

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum Init {
        #[error(transparent)]
        Git(#[from] git_storage::write::error::Init),
    }

    #[derive(Debug, Error)]
    pub enum Identifier {
        #[error(transparent)]
        Git(#[from] git2::Error),
    }

    #[derive(Debug, Error)]
    pub enum FindEntry {
        #[error(transparent)]
        Git(#[from] git2::Error),
        #[error(transparent)]
        Parse(#[from] git_storage::refdb::error::ParseReference),
        #[error(transparent)]
        FindRef(#[from] git_storage::read::error::FindRef),
        // The below causes an error inside `thiserror` so we re-implement for now
        // #[error(transparent)]
        // Find(#[from] <git_storage::Write as git_storage::odb::Read>::FindCommit),
        #[error("Referenced commit not found")]
        FindCommit,
        #[error("No commit found in KEL")]
        Empty,
    }

    #[derive(Debug, Error)]
    pub enum KeriError {
        #[error(transparent)]
        Git(#[from] git2::Error),
        #[error("No head found for KEL")]
        EmptyLog,
        #[error(transparent)]
        FindRef(#[from] FindEntry),
        #[error(transparent)]
        MsgParseError(#[from] std::str::Utf8Error),
    }

}

pub struct KeriStore {
    storage: Write,
}

impl KeriStore {
    pub fn open<P: AsRef<Path>>(path: P, info: UserInfo) -> Result<Self, error::Init> {
        Ok(KeriStore {
            storage: Write::open(path, info)?,
        })
    }

    pub fn log_head(&self) -> Result<git_storage::Commit, error::FindEntry> {
        //let head_ref_str = RefString::try_from("rad/keri/id")?;

        // TODO hardcoded ref name: take out to a const
        let lit: String = String::from("rad/keri/id");
        let parsed: &RefStr = lit.as_str().try_into().unwrap();
        let keri_ref = Qualified::from_refstr(parsed).unwrap();
        let head_ref = match self.storage.read_only().find_reference(&keri_ref)? {
            Some(head) => head,
            None => return Err(error::FindEntry::Empty)
        };

        let head_oid = match head_ref.target {
            git_storage::Target::Direct { oid } => oid,
            git_storage::Target::Symbolic { name } => todo!(),
        };
        let head_commit = match self.storage.read_only().find_commit(head_oid) {
            Ok(commit) => commit,
            Err(_) => return Err(error::FindEntry::FindCommit)
        };

        match head_commit {
            Some(commit) => {
                Ok(commit)
            },
            None => Err(error::FindEntry::Empty),
        }
    }

    pub fn log_entry_sn(&self, sn: u64) 
        -> Result<Option<TimestampedSignedEventMessage>, error::KeriError> {
        let head = self.log_head()?;

        let mut msg: Option<TimestampedSignedEventMessage> = self.log_entry_sn_in_commit(sn, &head)?;

        while msg.is_none() {
            msg = head.parents().find_map(|c| self.log_entry_sn_in_commit(sn, &c).ok()?);
        }

        Ok(msg)
    }

    fn log_entry_sn_in_commit(&self, sn: u64, commit: &Commit) 
        -> Result<Option<TimestampedSignedEventMessage>, error::KeriError> {
            let tree = commit.tree()?;
            let msg: Option<TimestampedSignedEventMessage> = tree.into_iter().find_map(|t| {
                if t.kind() == Some(ObjectType::Blob) {
                    let blob_id = t.id();
                    let blob = match self.storage.find_blob(blob_id.into()) {
                        Ok(Some(blob)) => blob,
                        Ok(_) => return None, // TODO This is a silent error
                        Err(_) => return None, // TODO This is a silent error
                    };
                    if let Ok(msg_string) = std::str::from_utf8(blob.content()) {
                        if let Ok(msg) = 
                            serde_json::from_str::<TimestampedSignedEventMessage>(msg_string) {
                        if msg.signed_event_message.event_message.event.get_sn() == sn {
                            Some(msg)
                        } else {
                            None
                        }
                    } else {
                        None // String is not a KERI message TODO this is a silent error, because we should only find KERI messages in this chain
                    }
                    } else {
                        None // Blob is not a string
                    }
                } else {
                    None // This is something else, but log messages are stored as blobs
                }
            });

            Ok(msg)
    }
    
}