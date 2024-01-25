/* The product team is back again. You did a great job improving the transaction log and they want to continue 
that progress and build an Internet of Things (IoT) device management platform so users can register a device 
with a numerical name  and later search for it.
For now, the product team settled on a numerical "name", to be available faster than the competition, and to
keep the requirements short:

Store IoT device objects (containing the IP address, numerical name, and type)
Retrieve IoT objects by numerical name
Iterate over IoT objects
A great use for a tree: the numerical name can be used to create a tree and search for it nice and quickly. */

type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree
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
