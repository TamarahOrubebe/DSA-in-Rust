/* After the great success of the IoT device platform, an add-on has been planned. The product team is asking 
for a way to efficiently process messages that come from the devices, so that customers only have to deal with
the actual handling of the message and skip the "plumbing" code. Since processing can be executed at (short) 
intervals, they require a way to order them quickly—ideally so that the device with the most messages can come
first.

This sounds like the heap data structure, doesn't it? In fact, it can be a max-heap. 
By using the number of messages to determine the priority of a message notification, the heap can do the heavy
lifting of this feature

The idea is to use the number of messages as an indicator of which device to poll first, which is why the device
is required. Using this type, the heap does not require any specific node or link types to work:*/

#[derive(Debug, Clone)]
struct IoTDevice {
    pub numerical_id: u64,
    pub address: String
}

#[derive(Debug, Clone)]
struct MessageNotification {
    no_of_messages: u64,
    dev: IoTDevice
}

#[derive(Debug, Clone)]
struct MessageChecker {
    pub length: usize,
    heap: Vec<Box<MessageNotification>>
}

impl MessageChecker {
    pub fn add(&mut self, notification: MessageNotification) {
        self.heap.push(Box::new(notification));
        self.length += 1;

        if self.length > 1 {
            let mut i = self.length;
            while i / 2 > 0 && self.has_more_messages(i, i / 2) {
                self.swap(i, i / 2);
                i /= 2;
            }
        }
        
    }

    fn has_more_messages(&self, pos1: usize, pos2: usize) -> bool {
        let a = &self.heap[pos1 - 1];
        let b = &self.heap[pos2 - 1];
        a.no_of_messages >= b.no_of_messages
    }

    fn swap(&mut self, pos1: usize, pos2: usize) {
        let index_a = pos1 - 1;
        let index_b = pos2 - 1;
        self.heap.swap(index_a, index_b);
    }

    pub fn pop(&mut self) -> Option<MessageNotification> {
        if self.length > 0 {
            let elem = self.heap.swap_remove(0);
            self.length = self.heap.len();
            let mut i = 1;

            while i * 2 < self.length {
                let children = (i * 2, i * 2 + 1);

                i = if self.has_more_messages(children.0, children.1) {
                    if self.has_more_messages(children.0, i) {
                        self.swap(i, children.0);
                        children.0
                    } else {
                        break;
                    }
                } else {
                    if self.has_more_messages(children.1, i) {
                        self.swap(i, children.1);
                        children.1
                    } else {
                        break;
                    }
                };
            }
            Some(*elem)
        } else {
            None
        }
    }
}


fn main() {
    
}