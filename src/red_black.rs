
//Turns out doing these kinds of trees in rust quickly becomes a mess... So fuck me i guess...

pub enum Color {
    RED,
    BLACK
}

pub struct Node<T: PartialOrd> {
    value: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub fn search<T: PartialOrd>(root: Node<T>, value: T) -> Node<T>{
    if root.value == value {
        return root;
    }
    if root.value > value {
        if let Some(node) = root.left {
            return search(*node, value);
        }
    }

    if root.value < value {
        if let Some(node) = root.right {
            return search(*node, value);
        }
    }

    return root;
}

pub fn insert<T: PartialOrd>( mut node: Node<T>, value: T) -> Node<T>
{
    if value < node.value
        {
            if let Some(left_node) = node.left {
                node.left = Some(Box::new(insert(*left_node, value)));
            } else 
            {
                node.left = Some(Box::new(Node { value: value, color: Color::RED, left: None, right: None}));
            }
        }
        else if value > node.value
        {
            if let Some(right_node) = node.right {
                node.right = Some(Box::new(insert(*right_node, value)));
            } else 
            {
                node.right = Some(Box::new(Node { value: value, color: Color::RED, left: None, right: None}));
            }
        }

    return node;
}