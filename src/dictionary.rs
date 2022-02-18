use std::borrow::{Borrow, Cow};
use std::collections::hash_map::DefaultHasher;
use std::collections::linked_list::LinkedList;
use std::hash::{Hash, Hasher};
use std::{cmp::Eq, collections::HashMap, iter::FromIterator};

#[derive(Debug)]
struct DictionaryCell<T, U> {
    items: LinkedList<DictionaryNodeItem<T, U>>,
}

#[derive(Debug)]
struct DictionaryNodeItem<T, U> {
    key: T,
    value: U,
}

struct Dictionary<T, U> {
    bucket_count: u64,
    buckets: Vec<DictionaryCell<T, U>>,
}

impl<T, U> Dictionary<T, U> {
    fn new() -> Self {
        const DEFAULT_CAPACITY: u64 = 16;
        let mut v = Vec::with_capacity(DEFAULT_CAPACITY as usize);
        for _ in 0..DEFAULT_CAPACITY {
            v.push(DictionaryCell {
                items: LinkedList::from([]),
            })
        }
        Self {
            bucket_count: DEFAULT_CAPACITY,
            buckets: v,
        }
    }

    fn new_with_capacity(capacity: u64) -> Self {
        Self {
            bucket_count: capacity,
            buckets: Vec::with_capacity(capacity as usize),
        }
    }
}

impl<T: Hash + Eq + Clone, U: Copy> Dictionary<T, U> {
    fn put(&mut self, key: &T, value: U) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index = hasher.finish() % &self.bucket_count;

        self.buckets[bucket_index as usize]
            .items
            .push_back(DictionaryNodeItem {
                key: key.clone(),
                value,
            });
    }

    fn get(&mut self, key: &T) -> Option<U> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index = hasher.finish() % &self.bucket_count;
        for item in &self.buckets[bucket_index as usize].items {
            if item.key == *key {
                return Some(item.value);
            }
        }
        None
    }

    fn delete(&mut self, key: &T) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index = hasher.finish() % self.bucket_count;
        let mut i: Option<usize> = None;
        for (j, item) in &mut self.buckets[bucket_index as usize].items.iter().enumerate() {
            if item.key == *key {
                i = Some(j);
                break;
            }
        }
        if i.is_some() {
            panic!("Bucket does not contain correct value!");
        }
        let remaining = &mut self.buckets[bucket_index as usize]
            .items
            .split_off(i.unwrap());
        remaining.pop_front();
        self.buckets[bucket_index as usize].items.append(remaining);
    }
}

struct MyCoolType<K: Eq + Hash, V>(HashMap<K, Vec<V>>);

impl<K: Eq + Hash, V> FromIterator<(K, V)> for MyCoolType<K, V> {
    fn from_iter<I>(tuples: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut m = HashMap::new();
        for (k, v) in tuples {
            m.entry(k).or_insert_with(Vec::new).push(v)
        }
        Self(m)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Complex<'a> {
    n: i32,
    s: Cow<'a, str>,
}

impl<'a> Complex<'a> {
    fn new<S: Into<Cow<'a, str>>>(n: i32, s: S) -> Self {
        Complex { n, s: s.into() }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ConfigKey {
    Text(String),
    Binary(Vec<u8>),
}

impl ConfigKey {
    fn as_ref(&self) -> ConfigKeyRef {
        match self {
            ConfigKey::Text(t) => ConfigKeyRef::Text(t),
            ConfigKey::Binary(b) => ConfigKeyRef::Binary(b),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum ConfigKeyRef<'a> {
    Text(&'a str),
    Binary(&'a [u8]),
}

// ----------

trait Key {
    fn to_key(&self) -> ConfigKeyRef;
}

impl Hash for dyn Key + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_key().hash(state);
    }
}

impl PartialEq for dyn Key + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.to_key() == other.to_key()
    }
}

impl Eq for dyn Key + '_ {}

impl Key for ConfigKey {
    fn to_key(&self) -> ConfigKeyRef {
        self.as_ref()
    }
}

impl<'a> Key for &'a str {
    fn to_key(&self) -> ConfigKeyRef {
        ConfigKeyRef::Text(self)
    }
}

impl<'a> Borrow<dyn Key + 'a> for ConfigKey {
    fn borrow(&self) -> &(dyn Key + 'a) {
        self
    }
}

impl<'a> Borrow<dyn Key + 'a> for &'a str {
    fn borrow(&self) -> &(dyn Key + 'a) {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dictionary() {
        let mut map = Dictionary::<String, i64>::new();
        let s = "hello world".to_string();
        map.put(&s, 42);
        println!("{:?}", map.get(&s).unwrap());
        println!("{:?}", map.get(&s).unwrap());
    }

    #[test]
    fn my_cool_type() {
        let tuples = vec![("one", 1), ("two", 2), ("one", 3)];
        let MyCoolType(m) = tuples.into_iter().collect();
        println!("{:?}", m);
    }

    #[test]
    fn complex_key_type() {
        let mut m = std::collections::HashMap::<Complex<'_>, i32>::new();
        m.insert(Complex::new(42, "foo"), 123);

        assert_eq!(123, *m.get(&Complex::new(42, "foo")).unwrap());
    }

    #[test]
    fn config_key() {
        let mut m = HashMap::new();
        m.insert(ConfigKey::Text("foo".into()), 123);
        m.insert(ConfigKey::Binary(vec![]), 456);

        assert_eq!(m.get(&ConfigKey::Text("foo".into())), Some(&123));
        assert_eq!(m.get(&"foo" as &dyn Key), Some(&123));
        assert_eq!(m.get(&"bar" as &dyn Key), None);
    }
}
