use std::rc::Rc;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new (value: T) -> Node<T> {
        Node {value, next: None}
    }
}


Rc<Cell>

struct LinkedLinst<T> {
    tail: Option<Rc<RefCell<Node<T>>>>
    head: Option<Rc<RefCell<Node<T>>>>

}

impl<T> LinkedLinst<T> {
    pub fn new() -> Self {
        Self{
            tail: None, 
            head: None
        }
    }
    pub fn add(&mut self, value: T) 
    {

        let new_node = Rc::new(ReffCell::new(Node::new(value)));
        
        match &self.tail {
            None => {
                self.head = Some(new_node.clone());
            },
            Some(last_node: &Rc<RefCell<Node<T>>>) => {
                let mut p_new_node = 
            }
        }
        
        
        // let node: Node<T> = Node::new(value);
        // let mut prev_heap: <Rc<Node<T>>> = Rc::new(node);
        // self.head = Some(prev_heap);

        // let mut prev_node: &Rc<Node<T>> = &self.head.unwrap();
        // prev_node.next =Some(node_heap);

    }
}

fn main() {
    println!("Hello, world!");
    let mut linked_list: LinkedLinst<i32> = LinkedLinst {head: None};
    let 

    linked_list.add(23)
}
