use std::fmt; 


#[derive(Debug, PartialEq, Eq,  Clone)]
struct SearchTreeNode {
    key: String,
    left: Option<Box<SearchTreeNode>>,
    right: Option<Box<SearchTreeNode>>,
}

impl SearchTreeNode {
    fn new(key: String) -> Self  {
        SearchTreeNode {
            key,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq,  Clone)]
#[allow(dead_code)]
pub struct SearchTree {
    root: Option<Box<SearchTreeNode>>,
}

impl SearchTree {
    pub fn new() -> Self {
        SearchTree {root: None}
    }

    pub fn push(&mut self, key: String) {
        if self.root.is_none() {
            let node = Box::new(SearchTreeNode::new(key)); 
            self.root = Some(node);
        }
        else {
            let mut current = &mut self.root; 
            while let Some(node) = current {
                if key < node.key {
                    if node.left.is_none() {
                        let new_node = Box::new(SearchTreeNode::new(key));
                        node.left = Some(new_node);
                        return;
                    }
                    else {
                        current = &mut node.left
                    }
                }
                else if key > node.key {
                    if node.right.is_none() {
                        let new_node = Box::new(SearchTreeNode::new(key));
                        node.right = Some(new_node);
                        return;
                    }
                    else {
                        current = &mut node.right;
                    }
                }
                else {
                    return;
                }
            }
        }
    }
    pub fn lookup(&self, lower: String, upper: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if let Some(root) = &self.root {
        SearchTree::lookup_recursive(lower, upper, root, &mut result);
    }
    result
}

    fn lookup_recursive(lower: String, upper: String, node: &Box<SearchTreeNode>, result: &mut Vec<String>) {
        if node.key > lower {
            if let Some(ref left) = &node.left {
                SearchTree::lookup_recursive(lower.clone(), upper.clone(), left, result);
            }
        }
        if node.key >= lower && node.key <= upper {
            result.push(node.key.clone());
        }
        if node.key < upper {
            if let Some(ref right) = &node.right {
                SearchTree::lookup_recursive(lower.clone(), upper.clone(), right, result);
            }
        }
    }
}

// functions to print the tree in a human readable format
impl fmt::Display for SearchTreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.key, repr(&self.left), repr(&self.right))
    }
}

fn repr(node: &Option<Box<SearchTreeNode>>) -> String {
    match node {
        Some(inner) => format!("{}", inner),
        None => "None".to_string(),
    }
}

impl fmt::Display for SearchTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.root {
            Some(root_node) => write!(f, "{}", root_node),
            None => write!(f, "None"),
        }
    }
}
