// NAIVE
// struct Node<T> {
//     value: T,
//     next: Option<Box<Node<T>>>
// }

// impl<T> Node<T> {
//     pub fn new(value: T) -> Self {
//         Self { value, next: None }
//     }
// }

// struct LinkedList<T> {
//     head: Option<Box<Node<T>>>
// }

// impl<T> LinkedList<T> {
//     pub fn add(&mut self, value: T) {
//         let node: Node<T> = Node::new(value);
//         let node_heap = Box::new(node);
//         self.head = Some(node_heap);

//         let mut prev_node = &self.head.unwrap();
//         prev_node.next = Some(node_heap);
//     }
// }

// fn main() {
//     let mut linked_list: LinkedList<i32> = LinkedList { head: None };
// }

// -----------------------------------

// use std::{cell::RefCell, rc::Rc};

// #[derive(Debug)]
// struct Node<T> {
//     value: T,
//     next: Option<Rc<RefCell<Node<T>>>>,
//     prev: Option<Rc<RefCell<Node<T>>>>,
// }

// // impl<T> Debug for Node<T> {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         todo!()
// //     }
// // }

// impl<T> Node<T> {
//     pub fn new(value: T) -> Self {
//         Self { value, next: None, prev: None }
//     }
// }

// #[derive(Debug)]
// struct LinkedList<T> {
//     tail: Option<Rc<RefCell<Node<T>>>>,
//     head: Option<Rc<RefCell<Node<T>>>>,
// }

// impl<T: Copy> LinkedList<T> {
//     pub fn new() -> Self {
//         Self {
//             tail: None,
//             head: None,
//         }
//     }

//     pub fn add(&mut self, value: T) {
//         let new_node = Rc::new(RefCell::new(Node::new(value)));

//         match &self.tail {
//             None => {
//                 self.head = Some(new_node.clone());
//             },
//             Some(last_node) => {
//                 let mut p_new_node = new_node.borrow_mut();
//                 p_new_node.prev = Some(last_node.clone());

//                 let mut p_last_node = last_node.borrow_mut();
//                 p_last_node.next = Some(new_node.clone());
//             }
//         }

//         self.tail = Some(new_node);
//     }

//     pub fn last(&self) -> Option<T> {
//         let node = self.tail.clone()?;
//         Some(node.borrow().value)
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         let tail = self.tail.take()?;
//         let prev = tail.borrow().prev.clone();

//         if let Some(prev_rc) = &prev {
//             prev_rc.borrow_mut().next = None;
//         } else {
//             self.head = None;
//         };

//         Some(tail.borrow().value)
//     }
// }

// -----------------------


use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

// impl<T> Debug for Node<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None, prev: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    tail: Option<Rc<RefCell<Node<T>>>>,
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            tail: None,
            head: None,
        }
    }

    pub fn add(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match &self.tail {
            None => {
                self.head = Some(new_node.clone());
            },
            Some(last_node) => {
                let mut p_new_node = new_node.borrow_mut();
                p_new_node.prev = Some(Rc::downgrade(&last_node));

                let mut p_last_node = last_node.borrow_mut();
                p_last_node.next = Some(new_node.clone());
            }
        }

        self.tail = Some(new_node);
    }

    pub fn last(&self) -> Option<T> {
        let node = self.tail.clone()?;
        Some(node.borrow().value)
    }

    pub fn pop(&mut self) -> Option<T> {
        let tail = self.tail.take()?;
        let prev = tail.borrow().prev.clone();

        if let Some(prev_weak) = &prev {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().next = None;
            }
        } else {
            self.head = None;
        };

        Some(tail.borrow().value)
    }
}


fn main() {
    let mut linked_list: LinkedList<i64> =  LinkedList::new();

    linked_list.add(1);
    linked_list.add(2);

    println!("{:?}", linked_list);
}