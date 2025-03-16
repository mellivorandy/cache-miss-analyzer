use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub enum NodeData {
    Dummy,
    Real { tag: u32 },
}

pub struct Node {
    data: NodeData,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

pub struct Set {
    pub capacity: usize,
    size: usize,
    map: HashMap<u32, Rc<RefCell<Node>>>,

    // Dummy nodes for doubly linked list
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl Set {
    pub fn new() -> Self {
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

        // Link head and tail
        head.borrow_mut().next = Some(Rc::clone(&tail));

        Set {
            capacity: 0,
            size: 0,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    // If hit, move node to the head and return true
    // Otherwise return false
    pub fn get(&mut self, tag: u32) -> bool {
        if let Some(node_rc) = self.map.get(&tag) {
            self.update_node(Rc::clone(node_rc));
            return true;
        }
        false
    }

    pub fn put(&mut self, tag: u32) {
        match self.map.get(&tag) {
            Some(node_rc) => {
                // Hit
                self.update_node(node_rc.clone());
            }
            None => {
                // Miss
                if self.size >= self.capacity {
                    if let Some(removed) = self.evict() {
                        if let NodeData::Real { tag: old_tag, .. } = removed.borrow().data {
                            self.map.remove(&old_tag);
                            self.size -= 1;
                        }
                    }
                }

                let new_node = Rc::new(RefCell::new(Node {
                    data: NodeData::Real { tag },
                    next: None,
                    prev: Some(Weak::new()),
                }));
    
                self.map.insert(tag, Rc::clone(&new_node));
                
                self.insert_at_front(Rc::clone(&new_node));
                self.size += 1;
            }
        }
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

    pub fn unlink(&self, node: &Rc<RefCell<Node>>) {
        let prev_rc_opt = node.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
        let next_rc_opt = node.borrow().next.clone();

        if let Some(prev_rc) = prev_rc_opt.clone() {
            match next_rc_opt.clone() {
                Some(next_rc) => {
                    prev_rc.borrow_mut().next = Some(next_rc.clone());
                }
                None => {
                    prev_rc.borrow_mut().next = None;
                }
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
            if let NodeData::Real { tag } = node_rc.borrow().data {
                result.push(tag);
            }
            current = node_rc.borrow().next.clone();
        }
        result
    }
}
