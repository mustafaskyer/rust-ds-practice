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
