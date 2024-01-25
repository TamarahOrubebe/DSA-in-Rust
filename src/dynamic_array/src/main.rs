// Again, the product team has another feature request. Users liked the going-back-and-forth feature a lot,
// so they want to save a few noteworthy timestamps in a separate list.
// To clean up the product team's demands, here is a list of the required features:

// Save a transaction's timestamp in a list
// Access the elements quickly by index, in any order
// Iterate the items in the order they were saved
use std::cmp;

type Node = Option<u64>;

pub struct TimeStampSaver {
    cap: usize,
    pub length: usize,
    buff: Box<[Node]>
}


impl TimeStampSaver {
    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buff.len();
        let mut new_cap = old_cap + (old_cap >> 1);
        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());

        let current = self.buff.clone();
        self.cap = new_cap;
        self.buff = vec![None; new_cap].into_boxed_slice();
        self.buff[..current.len()].clone_from_slice(&current);
    }

    fn at(&mut self, index: usize) -> Option<u64> {
        if index < self.length {
            self.buff[index]
        } else {
            None
        }
    }
}

struct TimeStampList {
    current: usize,
    data: Box<[Node]>
}


impl Iterator for TimeStampList {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            self.current += 1;
            item
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for TimeStampList {
     fn next_back(&mut self) -> Option<u64> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            if self.current == 0 {
                self.current = self.data.len() - 1;
            } else {
                 self.current -= 1;
            }

            item
        } else {
            None
        }
    }
}


fn main() {

}