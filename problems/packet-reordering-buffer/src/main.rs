/*
Ordered Payload Processor

Design a function that receives a sequence number and its payload.
Payloads may arrive out of order, but they must be released in
increasing sequence order starting from 1.

- Buffer out-of-order payloads.
- Release all contiguous payloads whenever possible.
- Ignore duplicate payloads that have already been released or buffered.
- Return the payloads released for each incoming request.

Example:
(2, "B") -> []
(1, "A") -> ["A", "B"]
(4, "D") -> []
(3, "C") -> ["C", "D"]
(2, "B") -> []   // Duplicate
(5, "E") -> ["E"]
*/

use std::collections::HashMap;

struct OrderedSequence {
    next_expected: i32,
    buffer: HashMap<i32, String>,
}

impl OrderedSequence {
    fn new() -> Self {
        Self {
            next_expected: 1,
            buffer: HashMap::new(),
        }
    }

    fn receive(&mut self, sequence_number: i32, payload: &str) -> Vec<String> {
        // Already released, ignore
        if sequence_number < self.next_expected {
            return vec![];
        }

        // Duplicate in buffer, ignore
        if self.buffer.contains_key(&sequence_number) {
            return vec![];
        }

        self.buffer
            .insert(sequence_number, payload.to_string());

        let mut result = Vec::new();

        while let Some(payload) = self.buffer.remove(&self.next_expected) {
            result.push(payload);
            self.next_expected += 1;
        }

        result
    }
}

fn main() {
    let mut seq = OrderedSequence::new();

    println!("{:?}", seq.receive(2, "B")); // []
    println!("{:?}", seq.receive(1, "A")); // ["A", "B"]
    println!("{:?}", seq.receive(4, "D")); // []
    println!("{:?}", seq.receive(3, "C")); // ["C", "D"]
    println!("{:?}", seq.receive(2, "B")); // []
    println!("{:?}", seq.receive(5, "E")); // ["E"]
}