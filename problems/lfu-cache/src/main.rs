use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NodePtr = Rc<RefCell<Node>>;

struct Node {
    key: i32,
    value: i32,
    freq: usize,
    prev: Option<NodePtr>,
    next: Option<NodePtr>,
}

struct DoublyLinkedList {
    head: Option<NodePtr>,
    tail: Option<NodePtr>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
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

    fn remove_tail(&mut self) -> Option<NodePtr> {
        if let Some(tail) = self.tail.clone() {
            self.remove(tail.clone());
            Some(tail)
        } else {
            None
        }
    }

    fn state(&self) -> String {
        let mut current = self.head.clone();
        let mut parts = Vec::new();

        while let Some(node) = current {
            let n = node.borrow();

            parts.push(format!(
                "[{}:{}:f={}]",
                n.key,
                n.value,
                n.freq
            ));

            current = n.next.clone();
        }

        parts.join(" ")
    }
}

struct LFUCache {
    capacity: usize,
    min_freq: usize,

    // key -> node
    map: HashMap<i32, NodePtr>,

    // freq -> DLL
    freq_map: HashMap<usize, DoublyLinkedList>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            min_freq: 0,
            map: HashMap::new(),
            freq_map: HashMap::new(),
        }
    }

    fn update_freq(&mut self, node: NodePtr) {
        let old_freq = node.borrow().freq;

        // remove from old freq DLL
        if let Some(list) = self.freq_map.get_mut(&old_freq) {
            list.remove(node.clone());

            // if old freq list becomes empty
            if list.is_empty() && self.min_freq == old_freq {
                self.min_freq += 1;
            }
        }

        // increase freq
        node.borrow_mut().freq += 1;

        let new_freq = node.borrow().freq;

        // insert into new freq DLL
        self.freq_map
            .entry(new_freq)
            .or_insert_with(DoublyLinkedList::new)
            .insert_at_head(node);
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            let value = node.borrow().value;

            self.update_freq(node);

            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // key already exists
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;

            self.update_freq(node);

            return;
        }

        // eviction
        if self.map.len() == self.capacity {
            if let Some(list) = self.freq_map.get_mut(&self.min_freq) {
                if let Some(node) = list.remove_tail() {
                    let old_key = node.borrow().key;
                    self.map.remove(&old_key);
                }
            }
        }

        // create new node
        let new_node = Rc::new(RefCell::new(Node {
            key,
            value,
            freq: 1,
            prev: None,
            next: None,
        }));

        // insert into freq=1 DLL
        self.freq_map
            .entry(1)
            .or_insert_with(DoublyLinkedList::new)
            .insert_at_head(new_node.clone());

        self.map.insert(key, new_node);

        self.min_freq = 1;
    }

    fn cache_state(&self) {
        println!("-------------------");

        for (freq, dll) in &self.freq_map {
            println!("freq {} => {}", freq, dll.state());
        }

        println!("min_freq => {}", self.min_freq);
        println!("-------------------");
    }
}

fn main() {
    let mut cache = LFUCache::new(2);

    cache.put(1, 1);
    cache.cache_state();

    cache.put(2, 2);
    cache.cache_state();

    println!("get(1) => {}", cache.get(1));
    cache.cache_state();

    cache.put(3, 3); // evicts key 2
    cache.cache_state();

    println!("get(2) => {}", cache.get(2));
    println!("get(3) => {}", cache.get(3));

    cache.cache_state();

    cache.put(4, 4); // evicts key 1
    cache.cache_state();

    println!("get(1) => {}", cache.get(1));
    println!("get(3) => {}", cache.get(3));
    println!("get(4) => {}", cache.get(4));

    cache.cache_state();
}