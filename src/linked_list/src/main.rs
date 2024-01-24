use std::cell::RefCell;
use std::rc::Rc;

// The goal here is to use a linkedlist to implement a transaction log
// say of SQL statements to a database to enable recovery of the DB state



// Create a base Node data structure
// to help the compiler know the size of the recursive Node type during static analysis, wrap it in a smart
// pointer and since we would want to mutate it's value the RefCell comes in handy due to interior mutability

type SingleLink = Option<Rc<RefCell<Node>>>;
#[derive(Clone)]
struct Node {
    value: String,
    next:  SingleLink
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None
        }))
    }
}

// Create the Transaction log data structure which would be the list 
// You would want to have a pointer to the head, the tail, and a length field 
// to facilitate constant time insertion and deletion at both ends.

struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64
}

impl TransactionLog {
    // creating a brand new list will come in handy
    fn new_empty_list() -> TransactionLog {
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

        self.head.take().map(|head| {
            if let Some(old) = head.borrow_mut().next.take() {
                self.head = Some(old);
            } else {
                self.tail.take();
            }

            self.length -= 1;

            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }
}


fn main() {
    let mut transaction_log = TransactionLog::new_empty_list();

    transaction_log.append("hello ".to_owned());
    transaction_log.append("there ".to_owned());
    transaction_log.append("how ".to_owned());
    transaction_log.append("are ".to_owned());
    transaction_log.append("you?".to_owned());

    for _ in 0..5 {
        print!("{}", transaction_log.pop().unwrap());
    }
}