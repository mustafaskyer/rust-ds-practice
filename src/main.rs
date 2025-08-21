mod double_link_list;
mod link_list;
use double_link_list::{Double_LinkList, Double_LinkList_Node};
use link_list::{LinkList, Node};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct CustomNode {
    next: Option<Weak<RefCell<CustomNode>>>,
}

impl Drop for CustomNode {
    fn drop(&mut self) {
        println!("Dropping {:?}", self)
    }
}
fn main() {
    let list = Node {
        element: 1,
        next: None,
    };

    println!("{:?}", list.element);

    let link_list = LinkList {
        head: Some(Box::new(Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 2,
                next: Some(Box::new(Node {
                    element: 1,
                    next: None,
                })),
            })),
        })),
    };

    println!("{:?}", &link_list.head);
    println!("{:?}", &link_list.head.unwrap().next.unwrap().element);

    // with impl
    let mut new_link_list = LinkList::new();
    new_link_list.add(3);
    new_link_list.add(4);
    println!("{:?}", new_link_list.head);

    println!("Removing ...");
    println!("{:?}", new_link_list.remove().unwrap()); // 4
    new_link_list.add(4);
    new_link_list.add(5);
    new_link_list.add(6);
    println!("Prinintg ...");
    println!("{:?}", new_link_list.print()); // 4
    println!("Peaking ...");
    println!("{:?}", new_link_list.peak()); // 4

    /* Double link list */
    let mut d_list = Double_LinkList::new();
    d_list.add(1);
    d_list.add(2);
    d_list.add(3);
    d_list.add(4);
    d_list.add(5);

    d_list.remove();

    println!("Printing {:?}", d_list.print());

    /* Reporduce ref cycles */
    let a = Rc::new(RefCell::new(CustomNode { next: None }));
    println!(
        "A Count {:?} . Weak count: {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );

    let b = Rc::new(RefCell::new(CustomNode {
        next: Some(Rc::downgrade(&a)),
    }));
    println!("A Count After Created B: {:?}", Rc::strong_count(&a));
    println!("B Count {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(CustomNode {
        next: Some(Rc::downgrade(&b)),
    }));
    println!("A Count After Created C {:?}", Rc::strong_count(&a));
    println!("B Count After Created C {:?}", Rc::strong_count(&a));
    println!("C Count {:?}", Rc::strong_count(&b));

    // let a point to c
    (*a).borrow_mut().next = Some(Rc::downgrade(&c)); // creating a cycle

    println!("Result after creating the cycle");
    println!("A: {:?}", Rc::strong_count(&a));
    println!("B: {:?}", Rc::strong_count(&b));
    println!("C: {:?}", Rc::strong_count(&c));

    // Don't execute this, will cause stack overflow
    println!("A {:?}", a); // thread 'main' has overflowed its stack . thread 'main' has overflowed its stack
}
