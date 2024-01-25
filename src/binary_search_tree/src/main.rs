/* The product team is back again. You did a great job improving the transaction log and they want to continue 
that progress and build an Internet of Things (IoT) device management platform so users can register a device 
with a numerical name  and later search for it.
For now, the product team settled on a numerical "name", to be available faster than the competition, and to
keep the requirements short:

Store IoT device objects (containing the IP address, numerical name, and type)
Retrieve IoT objects by numerical name
Iterate over IoT objects
A great use for a tree: the numerical name can be used to create a tree and search for it nice and quickly. */
use std::mem;
type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree
}

impl Node {
    fn new(dev: IoTDevice) -> Box<Node> {
        Box::new(Node {
            dev,
            left: None,
            right: None
        })
    }
}

struct BinarySearchTree {
    root: Tree,
    pub length: u64,
}

#[derive(Debug, Clone)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String
}


impl BinarySearchTree {
    pub fn add(&mut self, device: IoTDevice) {
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
        self.length += 1;
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            },

            _ => Some(Node::new(device))
        }
    }

    pub fn find(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, numerical_id)
    }

    fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < numerical_id {
                    self.find_r(&n.left, numerical_id)
                } else {
                    self.find_r(&n.right, numerical_id)
                }
            },

            _ => None
        }
    }

    pub fn walk(&self) -> Vec<IoTDevice> {
        self.walk_in_order(&self.root)
    }

    fn walk_in_order(&self, node: &Tree) -> Vec<IoTDevice> {
        let mut sorted_iot_devices: Vec<IoTDevice> = Vec::new();

        if let Some(n) = node {
             self.walk_in_order(&n.left);
             sorted_iot_devices.push(n.dev.clone());
             self.walk_in_order(&n.right);
        } 
       sorted_iot_devices
    }
}

fn main() {

}


