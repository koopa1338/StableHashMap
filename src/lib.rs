use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::Hash,
};

pub struct StableHashMap<K, V, H = RandomState>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    hashmap: HashMap<usize, V, H>,
    key_vec: Vec<K>,
}

impl<K, V, H> StableHashMap<K, V, H>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    #[must_use]
    pub fn with_hasher(hash_builder: H) -> Self {
        Self {
            hashmap: HashMap::with_hasher(hash_builder),
            key_vec: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: H) -> Self {
        Self {
            hashmap: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            key_vec: Vec::new(),
        }
    }
}

impl<K, V> StableHashMap<K, V, RandomState>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
            key_vec: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_capacity(size: usize) -> Self {
        Self {
            hashmap: HashMap::with_capacity(size),
            key_vec: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.hashmap.insert(self.key_vec.len(), value);
        self.key_vec.push(key);
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        self.key_vec.pop().and_then(|key| {
            self.hashmap
                .remove(&self.key_vec.len())
                .map(|val| (key, val))
        })
    }

    #[must_use]
    pub fn get(&self, idx: usize) -> Option<(&K, &V)> {
        self.key_vec.get(idx).and_then(|key| {
            self.hashmap
                .get_key_value(&idx)
                .map(|(_, value)| (key, value))
        })
    }

    #[must_use]
    pub fn get_mut(&mut self, idx: usize) -> Option<(&mut K, &mut V)> {
        self.key_vec
            .get_mut(idx)
            .and_then(|key| self.hashmap.get_mut(&idx).map(|value| (key, value)))
    }

    #[must_use]
    pub fn get_by_key(&self, key: &K) -> Option<&V> {
        self.key_vec
            .iter()
            .position(|k| k == key)
            .and_then(|idx| self.hashmap.get(&idx))
    }

    #[must_use]
    pub fn get_mut_by_key(&mut self, key: &K) -> Option<&mut V> {
        self.key_vec
            .iter()
            .position(|k| k == key)
            .and_then(|idx| self.hashmap.get_mut(&idx))
    }
}

impl<K, V> Default for StableHashMap<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct StableHashMapIntoIterator<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    stable_map: StableHashMap<K, V>,
    index: usize,
}

impl<K, V> Iterator for StableHashMapIntoIterator<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.stable_map.key_vec.get(self.index).and_then(|key| {
            self.stable_map
                .hashmap
                .get(&self.index)
                .map(|value| (key.clone(), value.clone()))
        });
        self.index += 1;

        result
    }
}

impl<K, V> IntoIterator for StableHashMap<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    type Item = (K, V);
    type IntoIter = StableHashMapIntoIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            stable_map: self,
            index: 0,
        }
    }
}

impl<K, V> From<&[(K, V)]> for StableHashMap<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    fn from(tuples: &[(K, V)]) -> Self {
        let key_vec: Vec<K> = tuples.iter().map(|(k, _)| k.clone()).collect();
        let hashmap: HashMap<usize, V> = tuples
            .iter()
            .enumerate()
            .map(|(idx, (_, v))| (idx, v.clone()))
            .collect();
        Self { hashmap, key_vec }
    }
}

impl<K, V> From<Vec<(K, V)>> for StableHashMap<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    fn from(tuples: Vec<(K, V)>) -> Self {
        let key_vec: Vec<K> = tuples.clone().into_iter().map(|(k, _)| k).collect();
        let hashmap: HashMap<usize, V> = tuples
            .into_iter()
            .enumerate()
            .map(|(usize, (_, v))| (usize, v))
            .collect();
        Self { hashmap, key_vec }
    }
}
