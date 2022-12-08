#[derive(Debug)]
enum NodeType {
    File(usize),
    Directory,
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    name: String,
    sub_nodes: Vec<Node>,
}

impl Node {
    fn is_directory(&self) -> bool {
        match self.node_type {
            NodeType::Directory => true,
            _ => false,
        }
    }

    fn size(&self) -> usize {
        match self.node_type {
            NodeType::File(size) => size,
            NodeType::Directory => self.sub_nodes.iter().map(|n| n.size()).sum(),
        }
    }
}

#[derive(Debug)]
pub struct Filesystem {
    home: Vec<Node>,
    pwd: Vec<usize>,
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {
            home: vec![],
            pwd: vec![],
        }
    }

    fn get_sub(&self) -> &Vec<Node> {
        let mut sub = &self.home;
        for i in &self.pwd {
            sub = &sub[*i].sub_nodes;
        }
        sub
    }

    fn get_idx(&self, dir: &str) -> Option<usize> {
        self.get_sub()
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
        let mut sub = &mut self.home;
        for i in &self.pwd {
            sub = &mut sub[*i].sub_nodes;
        }
        sub.push(Node {
            node_type: NodeType::Directory,
            name: name.to_string(),
            sub_nodes: vec![],
        });
    }

    pub fn create_file(&mut self, name: &str, size: usize) {
        let mut sub = &mut self.home;
        for i in &self.pwd {
            sub = &mut sub[*i].sub_nodes;
        }
        sub.push(Node {
            node_type: NodeType::File(size),
            name: name.to_string(),
            sub_nodes: vec![],
        });
    }

    pub fn size(&self) -> usize {
        self.home.iter().map(|n| n.size()).sum()
    }

    pub fn small_directories_size(&self) -> usize {
        println!("Home:");
        for item in &self.home {
            println!("[{:?}] {:?}", item.node_type, item.name);
        }
        println!();
        self.home
            .iter()
            .map(|n| get_small_directories_total(n, 0))
            .sum()
    }
}

fn get_small_directories_total(node: &Node, depth: usize) -> usize {
    let mut out = 0;
    if node.is_directory() {
        let s = node.size();
        out = node
            .sub_nodes
            .iter()
            .map(|sub| get_small_directories_total(sub, depth + 1))
            .sum();
        if s <= 100000 {
            out += s;
        }
    }
    for _ in 0..depth {
        print!(" ");
    }
    println!("> {} -> {out}", node.name);
    out
}
