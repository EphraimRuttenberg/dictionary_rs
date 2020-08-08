#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::clone::Clone;
use std::fmt;
use std::fmt::Write;

#[derive(Copy, Clone)]
enum Bucket<K: Clone, V: Clone> {
    Entry((K, V, usize, usize)),
    Empty,
    Tombstone
}

/* capacity is the number of objects the dict can hold, resizes when 
 *      it is at 2/3 capacity
 *      
 * size is the number of items in the dict, will never be more than
 *      2/3 capacity
 *
 * table is where the data is stored. it is in the format of a vec
 *      full of Bucket enums, which either encode an empty spot, a
 *      spot where an item was deleted, or an item
 *
 * This is meant to be a hashmap for keys that can be hashed 
 */
pub struct Dictionary<K: Clone + Hash, V: Clone> {
    capacity: usize,
    size: usize,
    table: Vec<Bucket<K, V>>
}


impl<K: Clone + Hash + PartialEq, V: Clone> Dictionary<K, V>{
    pub fn new() -> Dictionary<K, V> {
        Dictionary {
            capacity: 8,
            size: 0,
            table: vec![Bucket::Empty; 8]
        }
    }

    pub fn new_sized(size: usize) -> Dictionary<K, V> {
        if size == 0 {
            panic!("Cannot create a zero-sized dict");
        }

        Dictionary {
            capacity: size,
            size: 0,
            table: vec![Bucket::Empty; size]
        }
    }
    

    //Returns a Result::Err if the vectors are different sizes
    pub fn from_vecs(mut key_vec: Vec<K>, mut value_vec: Vec<V>) -> Dictionary<K, V> {
        if key_vec.len() != value_vec.len() {
            panic!("Differently sized vecs");
        } else if key_vec.is_empty() {
            panic!("Cannot create a zero-sized dict");
        } else {
            let _size = (key_vec.len()/2)*3 + 1;
            let mut dict: Dictionary<K, V> = Dictionary { 
                capacity: _size,
                size: 0,
                table: vec![Bucket::Empty; _size]
            };
            for _ in 0..key_vec.len() {
                let key = key_vec.pop().unwrap();
                let value = value_vec.pop().unwrap();
                dict.insert(key, value);
            }

            dict
        }
    }

    pub fn from_tuples(tuples: Vec<(K, V)>) -> Dictionary<K, V> {
        if tuples.is_empty() {
            panic!("Cannot create a zero-sized vec");
        }
        let mut dict: Dictionary<K, V> = Dictionary::new_sized(tuples.len());

        for (key, value) in tuples {
            dict.insert(key, value);
        }

        dict
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    //Checks if a resize is needed before inserting the new item, resizes if needed
    pub fn insert(&mut self, key: K, value: V) {
        self.size += 1;
        if 2 * (self.capacity/3) < self.size {
            self.resize();
        }
        let hash = self.get_hash(&key);
        self.force_insert(key, value, hash);
    }

    fn resize(&mut self) {
        self.capacity *= 2;
        let _table = self.table.clone();
        self.table = vec![Bucket::Empty; self.capacity];
        for entry in _table.iter() {    
            if let Bucket::Entry(d) = entry.clone() {
                self.force_insert(d.0, d.1, d.2);
            }
        }
    }

    // Inserts new items without regard for size of the dict, it is separated from 
    // the insert() function to prevent recursion on resizing. 
    fn force_insert(&mut self, key: K, value: V, key_hash: usize) {
        let mut index = (key_hash % self.capacity) as usize;
        const PERTURB_SHIFT: u8 = 5;
        let mut perturb: usize = key_hash;

        loop {
            let current: Bucket<K, V> = self.table.get(index).unwrap().clone();

            match current {
                Bucket::Entry(d) => {
                    if d.0 == key {
                        self.table[index] = Bucket::Entry((d.0, value, d.2, index));
                        break;
                    } else {
                        perturb >>= PERTURB_SHIFT;
                        index = ((5*index) + 1 + perturb) % self.capacity as usize;
                        continue
                    }
                },

                _ => {
                    self.table[index] = Bucket::Entry((key, value, key_hash, index));
                    break;
                }
            };
        }
    }

    /* Performs a lookup using almost the exact same algorithm as insertion
     * Returns an Some(value) if the key exists, and None otherwise
     */
    fn lookup(&self, key: &K) -> Option<(K, V, usize)> { 
        let key_hash: usize = self.get_hash(&key);

        let mut index = (key_hash % self.capacity) as usize;
        const PERTURB_SHIFT: u8 = 5;
        let mut perturb: usize = key_hash;

        loop {
            let current: Bucket<K, V> = self.table.get(index).unwrap().clone();

            match current {
                Bucket::Entry(d) => {
                    if d.0 == *key {
                        break Some((d.0, d.1, index));
                    } else {
                        perturb >>= PERTURB_SHIFT;
                        index = ((5*index) + 1 + perturb) % self.capacity as usize;
                        continue;
                    }
                },

                Bucket::Tombstone => {
                    perturb >>= PERTURB_SHIFT;
                    index = ((5*index) + 1 + perturb) % self.capacity as usize;
                    continue;
                }, 

                Bucket::Empty => {
                    break None;
                }
            };
        }
    }

    pub fn get(&self, key: &K) -> Result<V, String> {
       match self.lookup(key) {
           Some(v) => Ok(v.1),
           None => Err(format!("Key does not exist"))
       }
    }

    pub fn remove (&mut self, key: &K) {
        let value = self.lookup(key);
        match value {
            Some(v) => {
                self.table[v.2] = Bucket::Tombstone;
            },
            None => panic!("Key is not in the dictionary")
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.lookup(key).is_some()
    }

    fn get_hash(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() as usize
    }
    pub fn clear(&mut self) {
        *self = Dictionary::new();
    }

    fn get_non_empty(&self) -> impl Iterator<Item = &Bucket<K, V>> + '_ {
        self.table.iter()
            .filter(|v| match v { Bucket::Entry(_n) => true, _ => false })
    }

    // Returns a vector of keys contained in the dict
    pub fn keys(&self) -> Vec<&K> {
        let mut key_vec: Vec<&K> = Vec::new();
        for item in self.table.iter() {
            if let Bucket::Entry(n) = item {
                key_vec.push(&n.0);
            }
        }
        key_vec
    }

    // Returns a vector of values contained in the dict
    pub fn values(&self) -> Vec<&V> {
        let mut value_vec: Vec<&V> = Vec::new();
        for item in self.table.iter() {
            if let Bucket::Entry(n) = item {
                value_vec.push(&n.1);
            }
        }
        value_vec
    }
    
    // Returns a vector of (key, value) tuples containing every
    // key value pairing in the dict
    pub fn items(&self) -> Vec<(&K, &V)> {
        let mut item_vec: Vec<(&K, &V)> = Vec::new();
        for item in self.table.iter() {
            if let Bucket::Entry(n) = item {
                item_vec.push((&n.0, &n.1));
            }
        }
        item_vec
    }
}

impl<K, V> fmt::Display for Dictionary<K, V>
    where K: Clone + Hash + fmt::Display + PartialEq + Eq,
          V: Clone + fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str = String::new();
        output_str.push_str("{");

        for k in self.get_non_empty() {
            if let Bucket::Entry(d) = k {
                write!(output_str, "{}: {}, ", d.0, d.1)?;
            }
        }

        let len = output_str.len();
        if len > 1 {
            output_str = String::from(&output_str[..len - 2]);
        }
        output_str.push_str("}");

        write!(f, "{}", output_str)
    }
}
