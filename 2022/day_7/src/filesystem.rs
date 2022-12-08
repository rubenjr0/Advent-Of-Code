use crate::node::Node;

#[derive(Debug)]
pub struct Filesystem {
    home: Node,
    pwd: Vec<usize>,
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {
            home: Node::new_directory("/"),
            pwd: vec![],
        }
    }

    fn get_node(&self) -> &Node {
        let mut sub = &self.home;
        for i in &self.pwd {
            sub = &sub.children()[*i];
        }
        sub
    }

    fn get_node_mut(&mut self) -> &mut Node {
        let mut sub = &mut self.home;
        for i in &self.pwd {
            sub = &mut sub.children_mut()[*i];
        }
        sub
    }

    fn get_idx(&self, dir: &str) -> Option<usize> {
        self.get_node()
            .children()
            .iter()
            .enumerate()
            .find(|(_, node)| node.name() == dir)
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
        self.get_node_mut()
            .children_mut()
            .push(Node::new_directory(name));
    }

    pub fn create_file(&mut self, name: &str, size: usize) {
        let node = self.get_node_mut();
        node.children_mut().push(Node::new_file(name, size));
        let mut node = &mut self.home;
        node.increment_size(size);
        for i in &self.pwd {
            node = &mut node.children_mut()[*i];
            node.increment_size(size);
        }
    }

    pub fn small_directories(&self) -> Vec<&Node> {
        self.home
            .children()
            .iter()
            .flat_map(|n| get_small_directories_total(n))
            .collect()
    }

    pub fn deletion_candidate(&self, free_space: usize) -> Option<&Node> {
        deletion_candidate(&self.home, free_space)
    }
}

fn get_small_directories_total(node: &Node) -> Vec<&Node> {
    let mut out = vec![];
    if node.is_directory() {
        let filtered = node
            .children()
            .iter()
            .flat_map(|sub| get_small_directories_total(sub));
        if node.size() <= 100000 {
            out.push(node);
        }
        out.extend(filtered);
    }
    out
}

fn deletion_candidate(node: &Node, target_space: usize) -> Option<&Node> {
    let mut out = None;
    if node.size() >= target_space {
        out = Some(node);
        if let Some(best) = node
            .children()
            .iter()
            .filter(|n| n.is_directory() && n.size() >= target_space)
            .min_by_key(|n| n.size() - target_space)
        {
            let candidate = deletion_candidate(best, target_space);
            out = candidate.and_then(|c| if c.size() < node.size() { Some(c) } else { out });
        }
    }
    out
}
