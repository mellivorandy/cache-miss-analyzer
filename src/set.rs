use std::collections::hash_map;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    tag: u32,
    valid: bool,
    prev: Weak<RefCell<Node>>,
    next: Option<Rc<RefCell<Node>>>,
}

struct Set {

}

impl Set {
    
}