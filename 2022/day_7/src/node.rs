#[derive(Debug)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
pub struct Node {
    name: String,
    size: usize,
    node_type: NodeType,
    children: Vec<Node>,
}

impl Node {
    pub fn new_directory(name: &str) -> Node {
        Node {
            name: String::from(name),
            size: 0,
            node_type: NodeType::Directory,
            children: vec![],
        }
    }

    pub fn new_file(name: &str, size: usize) -> Node {
        Node {
            name: String::from(name),
            size,
            node_type: NodeType::File,
            children: vec![],
        }
    }

    pub fn is_directory(&self) -> bool {
        match self.node_type {
            NodeType::Directory => true,
            _ => false,
        }
    }

    pub fn increment_size(&mut self, size: usize) {
        self.size += size;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }
}
