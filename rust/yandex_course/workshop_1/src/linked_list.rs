

// struct Node<T> {
//     value: T,
//     next: Option<Box<Node<T>>>
// }

// impl<T> Node<T> {
//     pub fn new (value: T) -> Node<T> {
//         Node {value, next: None}
//     }
// }

// struct LinkedLinst<T> {
//     head: Option<Box<Node<T>>>

// }

// impl<T> LinkedLinst<T> {
//     pub fn add(&mut self, value: T) 
//     {
//         let node: Node<T> = Node::new(value);
//         let mut prev_heap: <Box<Node<T>>> = Box::new(node);
//         self.head = Some(prev_heap);

//         let mut prev_node: &Box<Node<T>> = &self.head.unwrap();
//         prev_node.next =Some(node_heap);

//     }
// }