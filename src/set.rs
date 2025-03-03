use std::collections::{hash_map, HashMap};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

enum NodeData {
    Dummy,
    Real { tag: u32, valid: bool },
}

struct Node {
    data: NodeData,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

pub struct Set {
    capacity: u32,
    size: u32,
    map: HashMap<u32, Rc<RefCell<Node>>>,

    // dummy head, tail
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl Set {
    fn new(capacity: u32) -> Self {
        let head = Rc::new(RefCell::new(Node {
            data: NodeData::Dummy,
            next: None,
            prev: None,
        }));
        let tail = Rc::new(RefCell::new(Node {
            data: NodeData::Dummy,
            next: None,
            prev: Some(Rc::downgrade(&head)),
        }));

        head.borrow_mut().next = Some(Rc::clone(&tail));

        Set {
            capacity,
            size: 0,
            map: HashMap::new(),
            head,
            tail,
        }
    }
}
