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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        // same reason to use &[u8] as `KeyDir::get`
        self.0.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &KeyDirEntry)> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_dir_entry_accessors() {
        let entry = KeyDirEntry::new(1, 24, 40, 0);

        assert_eq!(entry.file_id(), 1);
        assert_eq!(entry.value_size(), 24);
        assert_eq!(entry.value_pos(), 40);
        assert_eq!(entry.timestamp(), 0);
    }

    #[test]
    fn new_key_dir_is_empty() {
        let key_dir = KeyDir::new();

        assert!(key_dir.get(b"does_not_exist").is_none());
    }

    #[test]
    fn insert_returns_none_initially_and_previous_value_on_replacement() {
        let mut key_dir = KeyDir::new();
        let entry = KeyDirEntry::new(1, 24, 40, 0);

        assert!(
            key_dir
                .insert(Key::from(b"test_key".as_slice()), entry)
                .is_none()
        );

        assert_eq!(
            key_dir
                .insert(
                    Key::from(b"test_key".as_slice()),
                    KeyDirEntry::new(2, 26, 10, 10)
                )
                .unwrap(),
            entry
        );
    }

    #[test]
    fn key_accessible_after_insert() {
        let mut key_dir = KeyDir::new();
        let entry = KeyDirEntry::new(1, 24, 40, 0);
        key_dir.insert(Key::from(b"test_key".as_slice()), entry);

        assert_eq!(key_dir.get(b"test_key").unwrap(), &entry);
    }

    #[test]
    fn remove_returns_entry_and_none_on_removed_keys() {
        let mut key_dir = KeyDir::new();
        let entry = KeyDirEntry::new(1, 24, 40, 0);
        key_dir.insert(Key::from(b"test_key".as_slice()), entry);
        let key = b"test_key";

        let returned_entry = key_dir.remove(key).unwrap();
        assert_eq!(entry, returned_entry);
        assert!(key_dir.remove(key).is_none());
    }

    #[test]
    fn get_and_remove_return_none_on_missing_keys() {
        let mut key_dir = KeyDir::new();
        let entry = KeyDirEntry::new(1, 24, 40, 0);
        let key1 = b"test_key_1";
        key_dir.insert(Key::from(key1.as_slice()), entry);
        let key2 = b"test_key_2";

        assert!(key_dir.remove(key2).is_none());
        assert!(key_dir.get(key2).is_none());
    }

    #[test]
    fn supports_empty_key() {
        let mut key_dir = KeyDir::new();
        let entry = KeyDirEntry::new(1, 10, 20, 30);

        assert_eq!(key_dir.insert(Key::from(b"".as_slice()), entry), None);

        assert!(key_dir.get(b"".as_slice()).is_some());
        assert_eq!(key_dir.remove(b"".as_slice()).unwrap().file_id(), 1);
    }

    #[test]
    fn supports_binary_key() {
        let mut key_dir = KeyDir::new();
        let key = [0x00, 0xff, 0x01, 0xfe];
        let entry = KeyDirEntry::new(2, 20, 30, 40);

        assert_eq!(key_dir.insert(Key::from(key.as_slice()), entry), None);
        assert_eq!(key_dir.get(&key[..]).map(KeyDirEntry::file_id), Some(2));

        let other_key = [0x00, 0xff, 0x01, 0xfd];
        assert!(key_dir.get(&other_key).is_none());
    }

    #[test]
    fn iter_returns_all_entries() {
        let mut key_dir = KeyDir::new();

        key_dir.insert(
            Key::from(b"1".as_slice()),
            KeyDirEntry::new(1, 10, 100, 1000),
        );
        key_dir.insert(
            Key::from(b"2".as_slice()),
            KeyDirEntry::new(2, 20, 200, 2000),
        );

        let entries: Vec<_> = key_dir
            .iter()
            .map(|(key, entry)| (key.0.as_ref(), entry.file_id()))
            .collect();

        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&(b"1".as_slice(), 1)));
        assert!(entries.contains(&(b"2".as_slice(), 2)));
    }
}
