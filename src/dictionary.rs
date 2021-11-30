use std::collections::hash_map::DefaultHasher;
use std::collections::linked_list::LinkedList;
use std::hash::{Hash, Hasher};

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
}
