use std::collections::hash_map;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    tag: u32,
    valid: bool,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

pub struct Set {

}

impl Set {

}
