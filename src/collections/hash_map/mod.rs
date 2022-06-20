use std::fmt::Display;

pub struct Hashmap<K, V> {
    items: Vec<(K, V)>,
}

impl<K: Display, V: Display> Display for Hashmap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("");
        
        for item in &self.items {
            out.push_str(format!("{}: {}\n", item.0, item.1).as_str());
        }

        write!(f, "{}", out)
    }
}