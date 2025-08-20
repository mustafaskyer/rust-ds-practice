#[derive(Debug)]
pub struct LinkList<T> {
    pub head: Pointer<T>,
}
#[derive(Debug)]
pub struct Node<T> {
    pub element: T,
    pub next: Pointer<T>,
}
type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> LinkList<T> {
    pub fn new() -> LinkList<T> {
        LinkList { head: None }
    }

    pub fn add(&mut self, element: T) {
        let prev = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: prev,
        }));
        self.head = new_head;
    }

    pub fn remove(&mut self) -> Option<T> {
        match self.head.take() {
            Some(prev) => {
                self.head = prev.next;
                Some(prev.element)
            }
            None => None,
        }
    }

    pub fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }

    pub fn peak(&self) -> Option<T> {
        match &self.head {
            Some(prev) => Some(prev.element),
            None => None,
        }
    }
}

/*
* Demo client code
* let list = Node {
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
*/
