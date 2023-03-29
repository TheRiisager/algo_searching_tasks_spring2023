pub struct Node {
    pub key: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}

pub fn run() {
    let mut root: Node = Node {left: None, right: None, key: 50};
    root = insert(root, 30);
    root = insert(root, 20);
    root = insert(root, 40);
    root = insert(root, 70);
    root = insert(root, 60);
    root = insert(root, 80);

    inorder(root);
}

pub fn insert( mut node: Node, key: i32) -> Node
{
    if key < node.key
        {
            if let Some(left_node) = node.left {
                node.left = Some(Box::new(insert(*left_node, key)));
            } else 
            {
                node.left = Some(Box::new(Node{ left: None, right: None, key }));
            }
        }
        else if key > node.key
        {
            if let Some(right_node) = node.right {
                node.right = Some(Box::new(insert(*right_node, key)));
            } else 
            {
                node.right = Some(Box::new(Node{ left: None, right: None, key }));
            }
        }

    return node;
}

pub fn inorder (root: Node)
{
    if let Some(left_node) = root.left {
        inorder(*left_node);
    }
    println!("{} ", root.key);
    if let Some(right_node) = root.right {
        inorder(*right_node);
    }
}