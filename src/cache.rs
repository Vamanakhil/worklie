use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Old in-memory cache implementation
/// Superseded by SQLite CacheManager in cache_manager.rs
/// Kept for reference / potential fallback cache layer
#[allow(dead_code)]
pub struct Cache<K, V> {
    data: Arc<Mutex<HashMap<K, CacheEntry<V>>>>,
    ttl_seconds: u64,
}

struct CacheEntry<V> {
    #[allow(dead_code)]
    value: V,
    #[allow(dead_code)]
    timestamp: u64,
}

impl<K: std::hash::Hash + Eq, V: Clone> Cache<K, V> {
    /// Create a new cache with TTL (in seconds)
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            ttl_seconds,
        }
    }

    /// Get a value from cache if it exists and hasn't expired
    pub fn get(&self, key: &K) -> Option<V> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let data = self.data.lock().unwrap();
        if let Some(entry) = data.get(key) {
            if now - entry.timestamp < self.ttl_seconds {
                return Some(entry.value.clone());
            }
        }
        None
    }

    /// Put a value in cache
    pub fn put(&self, key: K, value: V) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut data = self.data.lock().unwrap();
        data.insert(
            key,
            CacheEntry {
                value,
                timestamp: now,
            },
        );
    }

    /// Clear expired entries
    pub fn cleanup_expired(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut data = self.data.lock().unwrap();
        data.retain(|_, entry| now - entry.timestamp < self.ttl_seconds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_cache_basic() {
        let cache: Cache<&str, String> = Cache::new(60);
        cache.put("key1", "value1".to_string());
        assert_eq!(cache.get(&"key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_cache_expiry() {
        let cache: Cache<&str, String> = Cache::new(1);
        cache.put("key1", "value1".to_string());
        assert_eq!(cache.get(&"key1"), Some("value1".to_string()));
        thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get(&"key1"), None);
    }
}
