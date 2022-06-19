use std::fmt::Display;

pub struct Hashmap<K, V> {
    items: Vec<(K, V)>,
}

impl<K, V> Hashmap<K, V> {
    pub fn push(&mut self, key: K, value: V) {
        self.items[]
    }

    pub fn get(&self, key: &K) -> &V {
        &self.value
    }
}

impl<K: Display, V: Display> Display for Hashmap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("");
        for i in 
    }
}