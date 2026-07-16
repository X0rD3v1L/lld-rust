use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Duration, Instant};
use std::thread::sleep;

type NodePtr = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    expires_at: Instant,
    prev: Option<NodePtr>,
    next: Option<NodePtr>,
}

struct LRUCache {
    capacity: usize,
    ttl: Duration,
    map: HashMap<i32, NodePtr>,
    head: Option<NodePtr>,
    tail: Option<NodePtr>,
}

impl LRUCache {
    fn new(capacity: usize, ttl_secs: u64) -> Self {
        Self {
            capacity,
            ttl: Duration::from_secs(ttl_secs),
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn is_expired(node: &NodePtr) -> bool {
        Instant::now() > node.borrow().expires_at
    }

    fn remove(&mut self, node: NodePtr) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        match prev {
            Some(ref p) => p.borrow_mut().next = next.clone(),
            None => self.head = next.clone(),
        }

        match next {
            Some(ref n) => n.borrow_mut().prev = prev.clone(),
            None => self.tail = prev.clone(),
        }

        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
    }

    fn insert_at_head(&mut self, node: NodePtr) {
        node.borrow_mut().next = self.head.clone();
        node.borrow_mut().prev = None;

        if let Some(ref head) = self.head {
            head.borrow_mut().prev = Some(node.clone());
        }

        self.head = Some(node.clone());

        if self.tail.is_none() {
            self.tail = Some(node);
        }
    }

    fn delete_node(&mut self, node: NodePtr) {
        let key = node.borrow().key;

        self.remove(node);
        self.map.remove(&key);
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {

            // TTL check
            if Self::is_expired(&node) {
                self.delete_node(node);
                return -1;
            }

            let value = node.borrow().value;

            self.remove(node.clone());
            self.insert_at_head(node);

            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {

            // refresh value + ttl
            {
                let mut n = node.borrow_mut();
                n.value = value;
                n.expires_at = Instant::now() + self.ttl;
            }

            self.remove(node.clone());
            self.insert_at_head(node);

            return;
        }

        // remove expired tail nodes first
        while let Some(tail) = self.tail.clone() {
            if Self::is_expired(&tail) {
                self.delete_node(tail);
            } else {
                break;
            }
        }

        // normal LRU eviction
        if self.map.len() == self.capacity {
            if let Some(tail) = self.tail.clone() {
                self.delete_node(tail);
            }
        }

        let new_node = Rc::new(RefCell::new(Node {
            key,
            value,
            expires_at: Instant::now() + self.ttl,
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
            let n = node.borrow();

            let expired = Instant::now() > n.expires_at;

            parts.push(format!(
                "[{}:{}:{}]",
                n.key,
                n.value,
                if expired { "expired" } else { "active" }
            ));

            current = n.next.clone();
        }

        parts.join(" ")
    }
}

fn main() {
    let mut cache = LRUCache::new(2, 3);

    cache.put(1, 10);
    cache.put(2, 20);

    println!("Initial: {}", cache.cache_state());

    sleep(Duration::from_secs(2));

    println!("get(1): {}", cache.get(1));

    sleep(Duration::from_secs(2));

    println!("get(1): {}", cache.get(1)); // expired
    println!("cache: {}", cache.cache_state());

    cache.put(3, 30);

    println!("Final: {}", cache.cache_state());
}