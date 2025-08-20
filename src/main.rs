mod link_list;

use link_list::{LinkList, Node};
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
}
