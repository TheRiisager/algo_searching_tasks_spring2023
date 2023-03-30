pub struct Node<T: PartialOrd> {
    value1: T,
    value2: Option<T>,
    node1: Option<Box<Node<T>>>,
    node2: Option<Box<Node<T>>>,
    node3: Option<Box<Node<T>>>
}

impl<T: PartialOrd> Node<T> {
    fn isLeaf(&self) -> bool{
        match &self.node1 {
            Some(_) => return false,
            None => return true,
        }
    }

    fn add(mut self, key: T) {
        if let None = self.value2{
            if self.value1 < key {
                self.value2 = Some(key);
            } else {
                self.value2 = Some(self.value1);
                self.value1 = key;
            }
        } else if let Some(val) = self.value2{

        }
    }
}

pub fn insert<T: PartialOrd>(mut tree: Node<T>, value: T) -> Node<T>{
    if tree.isLeaf() {
        
        return tree;
    }

    return tree;
}

// ???