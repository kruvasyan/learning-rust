use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_BUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V>
    where K: Eq + Hash {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }

    fn get_bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.get_bucket(&key);
        let bucket = &mut self.buckets[bucket];
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        self.items += 1; // diff with stream, its in the beginning
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.get_bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ref ekey, _)| ekey == key)
            .map(|&(_, ref v)| v)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let bucket = self.get_bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ref ekey, _)| ekey == key)
            .is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket = self.get_bucket(key);
        let bucket = &mut self.buckets[bucket];
        let i = bucket
            .iter()
            .position(|&(ref ekey, _)| ekey == key)?;
        self.items -= 1;
        Some(bucket.swap_remove(i).1)
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        return self.items == 0
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS,
            n => 2 * n,
        };

        let mut new_buckets: Vec<Vec<_>> = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        for (key, value) in self
            .buckets
            .iter_mut()
            .flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }

        self.buckets = new_buckets; // diff with stream, here mem::replace
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        map.insert("foo", 42);
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert_eq!(map.get(&"foo"), None);
    }
}

