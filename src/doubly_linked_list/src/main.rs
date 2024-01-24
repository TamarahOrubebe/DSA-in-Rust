use std::cell::RefCell;
use std::rc::Rc;

// The goal here is to use a doublylinkedlist to implement a transaction log
// say of SQL statements to a database that can be iterated forward/backward to enable recovery of the DB state



// Create a base Node data structure
// to help the compiler know the size of the recursive Node type during static analysis, wrap it in a smart
// pointer and since we would want to mutate it's value the RefCell comes in handy due to interior mutability

type Link = Option<Rc<RefCell<Node>>>;
#[derive(Debug, Clone)]
struct Node {
    value: String,
    next:  Link,
    prev: Link
}

#[derive(Debug, Clone)]
struct BetterTransactionLog {
    head: Link,
    tail: Link,
    length: u64
}


struct ListIterator {
    current: List
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator {
            current: start_at
        }
    }
}


impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        
        let current = &self.current;

        let mut result = None;

        self.current = match current {
            Some(inner_current) => {
                let current = inner_current.borrow();
                result = current.value.clone();
                current.next.clone()

            },

            None => None
        };

        result
    }
}