use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
    time::{Duration, Instant},
};

struct RateLimiter {
    max_requests: usize,
    window: Duration,
    clients: Mutex<HashMap<u32, VecDeque<Instant>>>,
}

impl RateLimiter {
    fn new(max_requests: usize, window_ms: u64) -> Self {
        Self {
            max_requests,
            window: Duration::from_millis(window_ms),
            clients: Mutex::new(HashMap::new()),
        }
    }

    fn is_allowed(&self, client_id: u32) -> bool {
        let now = Instant::now();

        // Lock hashmap
        let mut clients = self.clients.lock().unwrap();

        // Get queue for this client
        let queue = clients
            .entry(client_id)
            .or_insert_with(VecDeque::new);

        // Remove expired requests
        while let Some(&front) = queue.front() {
            if now.duration_since(front) > self.window {
                queue.pop_front();
            } else {
                break;
            }
        }

        // Deny if limit exceeded
        if queue.len() >= self.max_requests {
            return false;
        }

        // Store current request timestamp
        queue.push_back(now);

        true
    }
}

fn main() {
    let limiter = RateLimiter::new(3, 5000);

    for i in 1..=5 {
        let allowed = limiter.is_allowed(101);

        println!("Client 101 Request {} -> {}", i, allowed);

        std::thread::sleep(Duration::from_millis(500));
    }

    println!("Another client:");

    for i in 1..=4 {
        let allowed = limiter.is_allowed(202);

        println!("Client 202 Request {} -> {}", i, allowed);
    }
}