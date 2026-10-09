use std::collections::LinkedList;
use std::collections::linked_list;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;

struct Slot<K, V> {
    key: K,
    value: V,
}

pub struct HashMap<K: Hash + Eq, V> {
    buckets: Vec<LinkedList<Slot<K, V>>>,
    len: usize,
    collisions_qtt: usize,
}

pub struct Iter<'a, K: Hash + Eq, V> {
    hashmap: &'a HashMap<K, V>,
    bucket_idx: usize,
    current: linked_list::Iter<'a, Slot<K, V>>,
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: (0..16).map(|_| LinkedList::new()).collect(),
            len: 0,
            collisions_qtt: 0,
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let result = self.only_put(key, value);

        if result.is_none() {
            self.len += 1;
            if self.should_resize() {
                self.rehash();
            }
        }

        result
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_helper(key);
        let list = &self.buckets[index];

        list.iter().find(|&s| s.key == *key).map(|s| &s.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash_helper(key);
        let list = &mut self.buckets[index];
        let has_collision = list.len() > 1;
        let removed = list.extract_if(|s| s.key == *key).next();

        removed.map(|s| {
            self.len -= 1;
            if has_collision {
                self.collisions_qtt -= 1;
            }
            s.value
        })
    }

    fn rehash(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let old_buckets = mem::replace(
            &mut self.buckets,
            (0..new_capacity).map(|_| LinkedList::new()).collect(),
        );
        self.collisions_qtt = 0;

        for list in old_buckets {
            for slot in list {
                self.only_put(slot.key, slot.value);
            }
        }
    }

    fn only_put(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash_helper(&key);

        let list = &mut self.buckets[index];

        match list.iter_mut().find(|s| s.key == key) {
            Some(s) => Some(mem::replace(&mut s.value, value)),
            None => {
                if !list.is_empty() {
                    self.collisions_qtt += 1;
                }
                list.push_back(Slot { key, value });
                None
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn collisions_qtt(&self) -> usize {
        self.collisions_qtt
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            hashmap: self,
            bucket_idx: 0,
            current: self.buckets[0].iter(),
        }
    }

    fn hash_helper(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    #[inline]
    pub fn load_factor(&self) -> f64 {
        (self.len as f64) / (self.buckets.len() as f64)
    }

    #[inline]
    fn should_resize(&self) -> bool {
        self.load_factor() >= 0.8
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K: Hash + Eq, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(slot) = self.current.next() {
                return Some((&slot.key, &slot.value));
            }

            self.bucket_idx += 1;
            if self.bucket_idx >= self.hashmap.buckets.len() {
                return None;
            }

            self.current = self.hashmap.buckets[self.bucket_idx].iter();
        }
    }
}
