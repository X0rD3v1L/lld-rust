#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_back(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });

        // match self.head.as_mut() {
        //     None => {
        //         self.head = Some(new_node);
        //     }
        //     Some(mut current) => {
        //         while current.next.is_some() {
        //             current = current.next.as_mut().unwrap();
        //         }

        //         current.next = Some(new_node);
        //     }
        // }

        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node)
    }

    fn reverse(&mut self) {
        let mut prev: Option<Box<Node>> = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            // Save the next node
            let next = node.next.take();

            // Reverse the pointer
            node.next = prev;

            // Move prev and current forward
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }

    fn print(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }

        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Before reverse:");
    list.print();

    list.reverse();

    println!("After reverse:");
    list.print();
}