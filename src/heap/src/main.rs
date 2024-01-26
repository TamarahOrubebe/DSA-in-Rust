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
    pub length: u64,
    heap: Vec<Box<MessageNotification>>
}

impl MessageChecker {
    pub fn add(&mut self, notification: MessageNotification) {
        self.heap.push(Box::new(notifiction));
        self.length += 1;

        if self.length > 1 {
            let mut i = self.length;
            while i / 2 > 0 && self.has_more_messages(i, i / 2) {
                self.swap(i, i / 2);
                i /= 2;
            }
        }
        
    }

    fn has_more_messages(&self, pos1: u64, pos2: u64) {
        let a = &self.heap[pos1 - 1];
        let b = &self.heap[pos2 - 1];
        a.no_of_messages >= b.no_of_messages
    }

    fn swap(&mut self, pos1: u64, pos2: u64) {
        let index_b = pos2 - 1;
        let index_a = pos2 - 1;
        self.heap.swap(a, b);
    }

    
}