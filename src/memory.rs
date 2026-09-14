use std::borrow::Borrow;

use ahash::AHashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(Box<[u8]>);

impl Borrow<[u8]> for Key {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl From<&[u8]> for Key {
    fn from(value: &[u8]) -> Self {
        Key(value.into())
    }
}

impl From<Vec<u8>> for Key {
    fn from(value: Vec<u8>) -> Self {
        Key(value.into_boxed_slice())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyDirEntry {
    file_id: u32,
    value_size: u32,
    value_pos: u64,
    timestamp: u64,
}

impl KeyDirEntry {
    pub fn new(file_id: u32, value_size: u32, value_pos: u64, timestamp: u64) -> Self {
        Self {
            file_id,
            value_size,
            value_pos,
            timestamp,
        }
    }

    pub fn file_id(&self) -> u32 {
        self.file_id
    }

    pub fn value_size(&self) -> u32 {
        self.value_size
    }

    pub fn value_pos(&self) -> u64 {
        self.value_pos
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

pub struct KeyDir(AHashMap<Key, KeyDirEntry>);

impl KeyDir {
    pub fn new() -> Self {
        Self(AHashMap::default())
    }

    pub fn insert(&mut self, key: Key, entry: KeyDirEntry) -> Option<KeyDirEntry> {
        self.0.insert(key, entry)
    }

    pub fn get(&self, key: &[u8]) -> Option<&KeyDirEntry> {
        // NOTE: we are not using `&Key` as the signature for `key` argument here,
        // because at call site, most of the times key is already going to be a `&[u8]`,
        // forcing caller to pass a `&Key` would result in unnecessary allocations.
        self.0.get(key)
    }

    pub fn remove(&mut self, key: &[u8]) -> Option<KeyDirEntry> {
        self.0.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &KeyDirEntry)> {
        self.0.iter()
    }
}
