use std::cell::RefCell;
use std::rc::Rc;

// The goal here is to use a linkedlist to implement a transaction log
// say of SQL statements to a database to enable recovery of the DB state
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
        self.head.take().map(|head| {
            if let Some(old) = head.borrow_mut().next.take(){
                    self.head = Some(old);
                    self.length -= 1;
                } else {
                    self.tail.take();
            }
             Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }
}


fn main() {
    let mut transaction_log = TransactionLog::new_empty();

    transaction_log.append("hello ".to_owned());
    transaction_log.append("there ".to_owned());
    transaction_log.append("how ".to_owned());
    transaction_log.append("are ".to_owned());
    transaction_log.append("you?".to_owned());

    for _ in 0..5 {
        print!("{}", transaction_log.pop().unwrap());
    }
}