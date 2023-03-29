pub struct Node<T> {
    values: (T, Option<T>),
    node1: Box<Node<T>>,
    node2: Option<Box<Node<T>>>,
    node3: Option<Box<Node<T>>>
}

pub fn insert<T>() {

}