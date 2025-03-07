use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub enum NodeData {
    Dummy,
    Real { tag: u32, valid: bool },
}

pub struct Node {
    data: NodeData,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn get_tag(&self) -> Option<u32> {
        match &self.data {
            NodeData::Real { tag, .. } => Some(*tag),
            NodeData::Dummy => None,
        }
    }

    fn is_valid(&self) -> bool {
        match &self.data {
            NodeData::Real { valid, .. } => *valid,
            NodeData::Dummy => false,
        }
    }

    fn set_valid(&mut self, valid_flag: bool) {
        if let NodeData::Real { valid, .. } = &mut self.data {
            *valid = valid_flag;
        }
    }

    fn update_tag(&mut self, new_tag: u32) -> Option<u32> {
        if let NodeData::Real { tag, .. } = &mut self.data {
            let old_tag = *tag;
            *tag = new_tag;
            return Some(old_tag);
        }
        None
    }
}

pub struct Set {
    capacity: usize,
    size: usize,
    map: HashMap<u32, Rc<RefCell<Node>>>,

    // dummy head, tail
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl Set {
    pub fn new(capacity: usize) -> Self {
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

        // link head and tail
        head.borrow_mut().next = Some(Rc::clone(&tail));

        Set {
            capacity,
            size: 0,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    // if hit, move node to the head 
    // else return false
    pub fn get(&mut self, tag: u32) -> bool {
        if let Some(node_rc) = self.map.get(&tag) {
            if node_rc.borrow().is_valid() {
                self.update_node(Rc::clone(node_rc));
                return true;
            }
        }
        false
    }

    pub fn put(&mut self, tag: u32) {
        if let Some(node_rc) = self.map.get(&tag) {
            {
                // case 1: tag already exists in the cache
                let mut node = node_rc.borrow_mut();
                node.set_valid(true);
            }
            self.update_node(Rc::clone(node_rc));
            return;
        }
    
        // case 2: tag does not exist — check for reusable invalid node
        if let Some((old_tag, node_rc)) = self.find_invalid_node() {
            self.map.remove(&old_tag);
    
            {
                let mut node = node_rc.borrow_mut();
                node.update_tag(tag);
                node.set_valid(true);
            }
    
            // insert new tag into map and move to head
            self.map.insert(tag, Rc::clone(&node_rc));
            self.update_node(node_rc);
            return;
        }
    
        // case 3: no reusable invalid node found — need to create a new node
        let new_node = Rc::new(RefCell::new(Node {
            data: NodeData::Real { tag, valid: true },
            prev: Some(Weak::new()),
            next: None,
        }));
    
        // add the new node to the map and move to head
        self.map.insert(tag, Rc::clone(&new_node));
        self.insert_at_front(Rc::clone(&new_node));
    
        self.size += 1;
    
        // case 4: cache size exceeds capacity => evict the LRU node
        if self.size > self.capacity {
            if let Some(removed_node) = self.evict() {
                if let Some(old_tag) = removed_node.borrow().get_tag() {
                    self.map.remove(&old_tag);
                    self.size -= 1;
                }
            }
        }
    }
    
    fn find_invalid_node(&self) -> Option<(u32, Rc<RefCell<Node>>)> {
        for (&tag, node_rc) in &self.map {
            if !node_rc.borrow().is_valid() {
                return Some((tag, Rc::clone(node_rc)));
            }
        }
        None
    }

    pub fn insert_at_front(&self, node: Rc<RefCell<Node>>) {
        let head_next = self.head.borrow().next.clone();

        {
            let mut node_mut = node.borrow_mut();
            node_mut.prev = Some(Rc::downgrade(&self.head));
            node_mut.next = head_next.clone();
        }

        if let Some(ref old) = head_next {
            old.borrow_mut().prev = Some(Rc::downgrade(&node));
        }

        self.head.borrow_mut().next = Some(node);
    }

    // unlink the node from list
    pub fn unlink(&self, node: &Rc<RefCell<Node>>) {
        let prev_rc_opt = node.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
        let next_rc_opt = node.borrow().next.clone();

        if let Some(prev_rc) = prev_rc_opt.clone() {
            if let Some(next_rc) = next_rc_opt.clone() {
                prev_rc.borrow_mut().next = Some(next_rc.clone());
            } else {
                prev_rc.borrow_mut().next = None;
            }
        }

        if let Some(next_rc) = next_rc_opt {
            match prev_rc_opt {
                Some(prev_rc) => {
                    next_rc.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
                }
                None => {
                    next_rc.borrow_mut().prev = Some(Weak::new());
                }
            }
        }
    }

    pub fn update_node(&self, node: Rc<RefCell<Node>>) {
        self.unlink(&node);
        self.insert_at_front(node);
    }

    pub fn evict(&self) -> Option<Rc<RefCell<Node>>> {
        let last_rc = self.tail.borrow().prev.as_ref().and_then(|weak| weak.upgrade())?;
        let prev_rc_opt = last_rc.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
    
        if let Some(prev_rc) = prev_rc_opt {
            self.tail.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
            prev_rc.borrow_mut().next = Some(Rc::clone(&self.tail));
        }

        Some(last_rc)
    }
}
