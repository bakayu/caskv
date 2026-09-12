use crate::configuration::Config;

pub struct CaskVEngine {}
pub struct CaskVHandle {}

impl CaskVEngine {
    pub fn open(_opts: &Config) -> CaskVHandle {
        return CaskVHandle {};
    }
}
