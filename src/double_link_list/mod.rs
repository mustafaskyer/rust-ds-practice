use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Double_LinkList {
    pub head: Pointer,
    pub tail: Pointer,
}

#[derive(Debug)]
pub struct Double_LinkList_Node {
    pub elemnt: i32,
    pub next: Pointer,
    pub prev: Pointer,
}

type Pointer = Option<Rc<RefCell<Double_LinkList_Node>>>;

impl Double_LinkList {
    pub fn new() -> Self {
        Double_LinkList {
            head: None,
            tail: None,
        }
    }

    pub fn add(&mut self, element: i32) {
        let new_head = Double_LinkList_Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(old_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head)
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn remove(&mut self) -> Option<i32> {
        if (self.head.is_none()) {
            println!("List is empty");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().elemnt;
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("List is empty");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    pub fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().elemnt);
            traversal = traversal.unwrap().borrow().next.clone()
        }
    }
}

impl Double_LinkList_Node {
    fn new(element: i32) -> Rc<RefCell<Double_LinkList_Node>> {
        Rc::new(RefCell::new(Double_LinkList_Node {
            elemnt: element,
            next: None,
            prev: None,
        }))
    }
}
