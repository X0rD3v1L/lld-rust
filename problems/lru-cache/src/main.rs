use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

type NodePtr = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<NodePtr>,
    next: Option<NodePtr>
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NodePtr>,
    head: Option<NodePtr>,
    tail: Option<NodePtr>
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None
        }
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

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
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
            node.borrow_mut().value = value;

            self.remove(node.clone());
            self.insert_at_head(node);
            return;
        }

        if self.map.len() == self.capacity {
            if let Some(tail) = self.tail.clone() {
                let old_key = tail.borrow().key;
                self.remove(tail);
                self.map.remove(&old_key);
            }
        }

        let new_node = Rc::new(RefCell::new(Node {
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
            let n = node.borrow();
            parts.push(format!("[{}:{}]", n.key, n.value));
            current = n.next.clone();
        }

        parts.join(" ")
    }
}

fn main() {
    let mut cache = LRUCache::new(2);

    println!("Capacity :: {}", cache.capacity);
    cache.put(1, 1);
    println!("put(1, 1): {}", cache.cache_state());
    cache.put(2, 2);
    println!("put(2, 2): {}", cache.cache_state());

    println!("get(1): {}", cache.get(1));
    println!("cache: {}", cache.cache_state());

    cache.put(3, 3);
    println!("put(3, 3): {}", cache.cache_state()); // evicts key 2

    println!("get(2): {}", cache.get(2));
    println!("cache: {}", cache.cache_state());

    cache.put(4, 4);
    println!("put(4, 4): {}", cache.cache_state()); // evicts key 1

    println!("get(1): {}", cache.get(1));
    println!("get(3): {}", cache.get(3));
    println!("get(4): {}", cache.get(4));
    println!("cache: {}", cache.cache_state());
}
