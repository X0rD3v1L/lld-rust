struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let min = if let Some(&(_, current_min)) = self.stack.last() {
            current_min.min(val)
        } else {
            val
        };

        self.stack.push((val, min));
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop().map(|(val, _)| val)
    }

    fn top(&self) -> Option<i32> {
        self.stack.last().map(|&(val, _)| val)
    }

    fn get_min(&self) -> Option<i32> {
        self.stack.last().map(|&(_, min)| min)
    }
}

fn main() {
    let mut obj = MinStack::new();

    obj.push(2);

    println!("Popped: {:?}", obj.pop());
    println!("Top: {:?}", obj.top());
    println!("Min: {:?}", obj.get_min());
}