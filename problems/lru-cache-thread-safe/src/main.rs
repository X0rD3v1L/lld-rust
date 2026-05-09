use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type NodePtr = Arc<Mutex<Node>>;

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<NodePtr>,
    next: Option<NodePtr>,
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NodePtr>,
    head: Option<NodePtr>,
    tail: Option<NodePtr>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn remove(&mut self, node: NodePtr) {
        let (prev, next) = {
            let n = node.lock().unwrap();
            (n.prev.clone(), n.next.clone())
        };

        match prev {
            Some(ref p) => p.lock().unwrap().next = next.clone(),
            None => self.head = next.clone(),
        }

        match next {
            Some(ref n) => n.lock().unwrap().prev = prev.clone(),
            None => self.tail = prev.clone(),
        }

        let mut n = node.lock().unwrap();
        n.prev = None;
        n.next = None;
    }

    fn insert_at_head(&mut self, node: NodePtr) {
        {
            let mut n = node.lock().unwrap();
            n.next = self.head.clone();
            n.prev = None;
        }

        if let Some(ref head) = self.head {
            head.lock().unwrap().prev = Some(node.clone());
        }

        self.head = Some(node.clone());

        if self.tail.is_none() {
            self.tail = Some(node);
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            let value = node.lock().unwrap().value;

            self.remove(node.clone());
            self.insert_at_head(node);

            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.lock().unwrap().value = value;

            self.remove(node.clone());
            self.insert_at_head(node);
            return;
        }

        if self.map.len() == self.capacity {
            if let Some(tail) = self.tail.clone() {
                let old_key = tail.lock().unwrap().key;
                self.remove(tail);
                self.map.remove(&old_key);
            }
        }

        let new_node = Arc::new(Mutex::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }));

        self.insert_at_head(new_node.clone());
        self.map.insert(key, new_node);
    }

    fn cache_state(&self) -> String {
        let mut current = self.head.clone();
        let mut parts = Vec::new();

        while let Some(node) = current {
            let n = node.lock().unwrap();
            parts.push(format!("[{}:{}]", n.key, n.value));
            current = n.next.clone();
        }

        parts.join(" ")
    }
}

fn main() {
    use std::thread;

    // --- Case 1: cache miss ---
    println!("=== Case 1: cache miss ===");
    {
        let mut c = LRUCache::new(2);
        println!("get(99) = {} (expect -1)", c.get(99));
    }

    // --- Case 2: basic eviction (LRU is evicted) ---
    println!("\n=== Case 2: basic eviction ===");
    {
        let mut c = LRUCache::new(2);
        c.put(1, 1);
        c.put(2, 2);
        c.put(3, 3); // evicts key 1 (least recently used)
        println!("get(1) = {} (expect -1, evicted)", c.get(1));
        println!("get(2) = {} (expect 2)", c.get(2));
        println!("get(3) = {} (expect 3)", c.get(3));
    }

    // --- Case 3: get refreshes recency, changes eviction order ---
    println!("\n=== Case 3: get refreshes recency ===");
    {
        let mut c = LRUCache::new(2);
        c.put(1, 1);
        c.put(2, 2);
        c.get(1);    // key 1 becomes MRU, key 2 is now LRU
        c.put(3, 3); // evicts key 2
        println!("get(1) = {} (expect 1)", c.get(1));
        println!("get(2) = {} (expect -1, evicted)", c.get(2));
        println!("get(3) = {} (expect 3)", c.get(3));
    }

    // --- Case 4: update existing key (no eviction, value changes) ---
    println!("\n=== Case 4: update existing key ===");
    {
        let mut c = LRUCache::new(2);
        c.put(1, 10);
        c.put(1, 99); // update key 1
        println!("get(1) = {} (expect 99)", c.get(1));
        println!("state: {} (expect [1:99])", c.cache_state());
    }

    // --- Case 5: capacity = 1 ---
    println!("\n=== Case 5: capacity 1 ===");
    {
        let mut c = LRUCache::new(1);
        c.put(1, 1);
        c.put(2, 2); // evicts key 1
        println!("get(1) = {} (expect -1)", c.get(1));
        println!("get(2) = {} (expect 2)", c.get(2));
    }

    // --- Case 6: concurrent puts and gets across 4 threads ---
    println!("\n=== Case 6: concurrent threads ===");
    {
        let cache = Arc::new(Mutex::new(LRUCache::new(3)));

        let handles: Vec<_> = (1..=4).map(|i| {
            let c = cache.clone();
            thread::spawn(move || {
                let mut cache = c.lock().unwrap();
                cache.put(i, i * 10);
                println!("T{i}: put({i}, {}) → {}", i * 10, cache.cache_state());
            })
        }).collect();

        for h in handles {
            h.join().unwrap();
        }

        let mut cache = cache.lock().unwrap();
        println!("After 4 puts (cap 3), one key evicted");
        println!("Final: {}", cache.cache_state());
        // whichever key was inserted first is evicted; rest are present
        let present: Vec<i32> = (1..=4).filter(|&k| cache.get(k) != -1).collect();
        println!("Keys still present: {:?} (expect 3 of [1,2,3,4])", present);
    }
}
