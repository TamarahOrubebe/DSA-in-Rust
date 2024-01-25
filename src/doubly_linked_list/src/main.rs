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

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None
        }))
    }
}

#[derive(Debug, Clone)]
struct BetterTransactionLog {
    head: Link,
    tail: Link,
    length: u64
}

impl BetterTransactionLog {
    // creating a brand new list will come in handy
    fn new_empty_list() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new.clone())
        }
    
        self.length += 1;
        self.tail = Some(new);
    }

    fn pop(&mut self) -> Option<String> {

        self.head.take().map(|head| {
            if let Some(old) = head.borrow_mut().next.take() {
                self.head = Some(old.clone());
                old.borrow_mut().prev.take();

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

#[derive(Debug, Clone)]
struct ListIterator {
    current: Link
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
                result = Some(current.value.clone());
                current.next.clone()

            },

            None => None
        };

        result
    }
}

impl DoubleEndedIterator for ListIterator {

    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(inner_current) => {
                let current = inner_current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },

            None => None
        };

        result
    }
}


fn main() {
    let mut transaction_log = BetterTransactionLog::new_empty_list();
 
    transaction_log.append("hello ".to_owned());
    transaction_log.append("there ".to_owned());
    transaction_log.append("how ".to_owned());
    transaction_log.append("are ".to_owned());
    transaction_log.append("you?".to_owned());
    let new_log = transaction_log.clone();
    
    let mut transaction_log_iterator = ListIterator::new(transaction_log.head);
    let mut iter = ListIterator::new(new_log.tail);
    for v in transaction_log_iterator  {
        println!("{:?}", v);
    }

    for _ in 0..5 {
        println!("{:?}", iter.next_back())
    }

    // for _ in 0..5 {
    //     print!("{}", transaction_log.pop().unwrap());
    // }
}