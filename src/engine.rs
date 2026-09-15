use crate::CaskVError;
use crate::configuration::Config;
use crate::memory::{Key, KeyDir, KeyDirEntry};

pub struct CaskVEngine {
    key_dir: KeyDir,
}
pub struct CaskVHandle {}

impl CaskVEngine {
    pub fn open(_opts: &Config) -> Result<CaskVHandle, CaskVError> {
        Ok(CaskVHandle {})
    }
}
