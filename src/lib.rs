use std::{collections::HashMap, hash::Hash};

pub struct StableHashMap<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    hashmap: HashMap<K, V>,
    key_vec: Vec<K>,
}

impl<K, V> StableHashMap<K, V>
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
        self.key_vec.push(key.clone());
        self.hashmap.insert(key, value);
    }

    pub fn pop(&mut self) -> Option<(&K, &V)> {
        self.key_vec
            .pop()
            .and_then(|key| self.hashmap.get_key_value(&key))
    }

    pub fn get(&mut self, idx: usize) -> Option<(&K, &V)> {
        self.key_vec
            .get(idx)
            .and_then(|key| self.hashmap.get_key_value(key))
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<(&mut K, &mut V)> {
        self.key_vec
            .get_mut(idx)
            .and_then(|key| self.hashmap.get_mut(key).map(|value| (key, value)))
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
                .get(key)
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
        let key_vec = (*tuples).iter().map(|(k, _)| k.clone()).collect::<Vec<_>>();
        let hashmap = tuples
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<_, _>>();
        Self { hashmap, key_vec }
    }
}

impl<K, V> From<Vec<(K, V)>> for StableHashMap<K, V>
where
    K: Clone + Eq + PartialEq + Hash,
    V: Clone,
{
    fn from(tuples: Vec<(K, V)>) -> Self {
        let key_vec = tuples
            .clone()
            .into_iter()
            .map(|(k, _)| k)
            .collect::<Vec<_>>();
        let hashmap = tuples.into_iter().collect::<HashMap<_, _>>();
        Self { hashmap, key_vec }
    }
}
