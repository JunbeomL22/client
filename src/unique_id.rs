use std::sync::Mutex;
use once_cell::sync::Lazy;
//use rustc_hash::FxHashMap;
use ahash::AHashMap;
use serde::{Serialize, de};
use serde::de::Deserializer;

#[derive(Clone, Copy)]
pub struct UniqueId {
    id_ptr: *const str,
}

impl PartialEq for UniqueId {
    fn eq(&self, other: &Self) -> bool {
        //self.id_ptr == other.id_ptr
        std::ptr::eq(self.id_ptr, other.id_ptr)
    }
}

impl Eq for UniqueId {}

impl Serialize for UniqueId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match serde_json::from_str::<serde_json::Value>(self.as_str()) {
            Ok(value) => value.serialize(serializer),
            Err(_) => serializer.serialize_str(self.as_str())
        }
    }
}

impl<'de> de::Deserialize<'de> for UniqueId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let id_str = match value {
            serde_json::Value::String(s) => s,
            _ => serde_json::to_string(&value).map_err(de::Error::custom)?
        };
        Ok(UniqueId::from_str(&id_str))
    }
}
// Safety: id_ptr는 'static lifetime을 가진 문자열을 가리키므로 안전
unsafe impl Send for UniqueId {}
unsafe impl Sync for UniqueId {}

static UNIQUE_ID_CACHE: Lazy<Mutex<AHashMap<String, &'static str>>> = Lazy::new(|| Mutex::new(AHashMap::default()));
static DEFAULT_UNIQUE_ID: Lazy<UniqueId> = Lazy::new(|| UniqueId::from_str(""));

impl Default for UniqueId {
    fn default() -> Self {
        *DEFAULT_UNIQUE_ID
    }
}

impl UniqueId {
    pub fn from_str(id: &str) -> Self {
        let string = id.to_string();
        let mut cache = UNIQUE_ID_CACHE.lock().expect("Failed to lock id cache");
        let interned = *cache.entry(string).or_insert_with(|| Box::leak(Box::new(id.to_string())));
        UniqueId { id_ptr: interned as *const str }
    }

    pub fn as_str(&self) -> &str {
        unsafe { &*self.id_ptr }
    }

    pub fn count() -> usize {
        UNIQUE_ID_CACHE.lock().expect("Failed to lock id cache").len()
    }

    pub fn add<T: std::fmt::Display>(&self, others: &[T]) -> UniqueId {
        let mut new_id = self.as_str().to_string();
        for other in others {
            new_id.push_str(&other.to_string());
        }
        UniqueId::from_str(&new_id)
    }

    pub fn merged<T: std::fmt::Display>(elements: &[T]) -> UniqueId {
        let mut new_id = String::new();
        for element in elements {
            new_id.push_str(&element.to_string());
        }
        UniqueId::from_str(&new_id)
    }
}

impl std::ops::Deref for UniqueId {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl std::fmt::Display for UniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let escaped = self.as_str().replace("\\n", "\n")
                                 .replace("\\t", "\t")
                                 .replace("\\r", "\r");
        write!(f, "{}", escaped)
    }
}

impl std::fmt::Debug for UniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let escaped = self.as_str().replace("\\n", "\n")
                                 .replace("\\t", "\t")
                                 .replace("\\r", "\r");
        write!(f, "{}", escaped)
    }
}

impl std::hash::Hash for UniqueId {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id_ptr.hash(state)
    }
}

impl std::ops::Add<UniqueId> for UniqueId {
    type Output = UniqueId;

    fn add(self, other: UniqueId) -> UniqueId {
        let new_id = format!("{}{}", self.as_str(), other.as_str());
        UniqueId::from_str(&new_id)
    }
}

impl std::ops::Add<&str> for UniqueId {
    type Output = UniqueId;

    fn add(self, other: &str) -> UniqueId {
        let new_id = format!("{}{}", self.as_str(), other);
        UniqueId::from_str(&new_id)
    }
}

impl std::ops::Add<&UniqueId> for &UniqueId {
    type Output = UniqueId;

    fn add(self, other: &UniqueId) -> UniqueId {
        let new_id = format!("{}{}", self.as_str(), other.as_str());
        UniqueId::from_str(&new_id)
    }
}

impl std::ops::Add<&str> for &UniqueId {
    type Output = UniqueId;

    fn add(self, other: &str) -> UniqueId {
        let new_id = format!("{}{}", self.as_str(), other);
        UniqueId::from_str(&new_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_id_equality() {
        let id1 = UniqueId::from_str("test");
        let id2 = UniqueId::from_str("test");
        let id3 = UniqueId::from_str("test2");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
    }

    #[test]
    fn test_unique_id_serialization() {
        let id1 = UniqueId::from_str("test");
        let serialized = serde_json::to_string(&id1).unwrap();
        let deserialized: UniqueId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id1, deserialized);
    }

    #[test]
    fn test_unique_id_concatenation() {
        let id1 = UniqueId::from_str("test1");
        let id2 = UniqueId::from_str("test2");
        let id3 = UniqueId::from_str("test3");

        let res = id1 + id2 + id3;
        assert_eq!(res.to_string(), "test1test2test3");
    }

    #[test]
    fn test_unique_id_hash() {
        use std::collections::HashSet;
        
        let id1 = UniqueId::from_str("test");
        let id2 = UniqueId::from_str("test");
        let id3 = UniqueId::from_str("test2");

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_escaped_characters() {
        let id = UniqueId::from_str("test\\nline\\ttab\\rreturn");
        assert_eq!(id.to_string(), "test\nline\ttab\rreturn");
    }

    #[test]
    fn test_unique_id() {
        let id1 = UniqueId::from_str("test");
        let id2 = UniqueId::from_str("test");
        let id3 = UniqueId::from_str("test2");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
        
        println!("id1: {}", id1);
        let serialized = serde_json::to_string_pretty(&id1).unwrap();
        let deserialized: UniqueId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id1, deserialized);
    }

    #[test]
    fn test_unique_id_count() {
        let id1 = UniqueId::from_str("test");
        let id2 = UniqueId::from_str("test");
        let id3 = UniqueId::from_str("test2");

        let res = id1 + id2 + id3;
        assert_eq!(res.to_string(), "testtesttest2");
    }
}
