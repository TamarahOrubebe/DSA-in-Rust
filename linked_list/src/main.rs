use std::cell::RefCell;
use std::rc::Rc;

// The goal here is to use a linkedlist to implement a transaction log 
type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None
        }))
    }
}


struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64
}

impl TransactionLog {
    fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }

        self.length += 1;
        self.tail = Some(new);
    }

    fn pop(&mut self) -> Option<String> {
        match self.head.take() {
            Some(old) => {
                    old.borrow_mut().next = self.tail;
                    self.length -= 1;
                }
            
            None => ()
        }
    }
}


fn main() {
    
}