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

    // dummy
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

        let mut set = Set {
            capacity,
            size: 0,
            map: HashMap::new(),
            head,
            tail,
        };

        // initialize all cache blocks with valid = false
        for i in 0..capacity {
            let tag = i as u32;
            let new_node = Rc::new(RefCell::new(Node {
                data: NodeData::Real { tag, valid: false },
                prev: Some(Weak::new()),
                next: None,
            }));
            
            set.map.insert(tag, Rc::clone(&new_node));
            
            set.insert_at_front(Rc::clone(&new_node));
            set.size += 1;
        }

        set
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
    
        // case 2: tag does not exist => check for invalid node
        if let Some(invalid_tag) = self.find_invalid_node_tag() {
            let node_rc = self.map.remove(&invalid_tag).unwrap();
            {
                let mut node = node_rc.borrow_mut();
                node.update_tag(tag);
                node.set_valid(true);
            }

            // insert new tag into map and move to head
            self.map.insert(tag, Rc::clone(&node_rc));
            self.update_node(Rc::clone(&node_rc));
            return;
        }
        
        // case 3: no invalid node found => create a new node
        if let Some(removed_node) = self.evict() {
            let old_tag = removed_node.borrow().get_tag();
            
            if let Some(old_tag) = old_tag {
                self.map.remove(&old_tag);
                
                {
                    let mut node = removed_node.borrow_mut();
                    node.update_tag(tag);
                    node.set_valid(true);
                }
                
                self.map.insert(tag, Rc::clone(&removed_node));
                self.insert_at_front(Rc::clone(&removed_node));
            }
        }

        if self.size < self.capacity {
            let new_node = Rc::new(RefCell::new(Node {
                data: NodeData::Real { tag, valid: true },
                prev: None,
                next: None,
            }));

            self.map.insert(tag, Rc::clone(&new_node));
            self.insert_at_front(Rc::clone(&new_node));
            self.size += 1;
            return;
        }
    }
    
    fn find_invalid_node_tag(&self) -> Option<u32> {
        self.map.iter()
            .find(|(_, node_rc)| !node_rc.borrow().is_valid())
            .map(|(&tag, _)| tag)
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
            match next_rc_opt.clone() {
                Some(next_rc) => prev_rc.borrow_mut().next = Some(next_rc.clone()),
                None => prev_rc.borrow_mut().next = None,
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

    #[cfg(test)]
    pub fn debug_list(&self) -> Vec<u32> {
        let mut result = Vec::new();
        let mut current = self.head.borrow().next.clone();

        while let Some(node_rc) = current {
            if Rc::ptr_eq(&node_rc, &self.tail) {
                break;
            }
            if let Some(tag) = node_rc.borrow().get_tag() {
                if node_rc.borrow().is_valid() {
                    result.push(tag);
                }
            }
            current = node_rc.borrow().next.clone();
        }
        result
    }
}
