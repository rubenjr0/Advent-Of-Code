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
    fn is_directory(&self) -> bool {
        match self.node_type {
            NodeType::Directory => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Filesystem {
    home: Node,
    pwd: Vec<usize>,
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {
            home: Node {
                node_type: NodeType::Directory,
                name: String::from("/"),
                children: vec![],
                size: 0,
            },
            pwd: vec![],
        }
    }

    fn get_node(&self) -> &Node {
        let mut sub = &self.home;
        for i in &self.pwd {
            sub = &sub.children[*i];
        }
        sub
    }

    fn get_node_mut(&mut self) -> &mut Node {
        let mut sub = &mut self.home;
        for i in &self.pwd {
            sub = &mut sub.children[*i];
        }
        sub
    }

    fn get_idx(&self, dir: &str) -> Option<usize> {
        self.get_node()
            .children
            .iter()
            .enumerate()
            .find(|(_, node)| node.name == dir)
            .and_then(|(i, _)| Some(i))
    }

    pub fn cd(&mut self, dir: &str) {
        match dir {
            "/" => self.pwd = vec![],
            ".." => {
                self.pwd.pop();
            }
            dir => {
                if let Some(idx) = self.get_idx(dir) {
                    self.pwd.push(idx);
                }
            }
        }
    }

    pub fn create_dir(&mut self, name: &str) {
        self.get_node_mut().children.push(Node {
            node_type: NodeType::Directory,
            name: name.to_string(),
            children: vec![],
            size: 0,
        });
    }

    pub fn create_file(&mut self, name: &str, size: usize) {
        let node = self.get_node_mut();
        node.children.push(Node {
            node_type: NodeType::File,
            name: name.to_string(),
            children: vec![],
            size,
        });
        let mut node = &mut self.home;
        node.size += size;
        for i in &self.pwd {
            node = &mut node.children[*i];
            node.size += size;
        }
    }

    pub fn size(&self) -> usize {
        self.home.size
    }

    pub fn small_directories_size(&self) -> usize {
        self.home
            .children
            .iter()
            .map(|n| get_small_directories_total(n, 0))
            .sum()
    }
}

fn get_small_directories_total(node: &Node, depth: usize) -> usize {
    if !node.is_directory() {
        0
    } else {
        node.children
            .iter()
            .map(|sub| get_small_directories_total(sub, depth + 1))
            .sum::<usize>()
            + if node.size <= 100000 { node.size } else { 0 }
    }
}
