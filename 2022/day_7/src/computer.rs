use crate::{filesystem::Filesystem, node::Node};

enum Entry {
    Dir(String),
    Cd(String),
    File(String, usize),
}

impl Entry {
    fn parse(entry: &str) -> Entry {
        let mut entry = entry.split_whitespace();
        let command = entry.next().unwrap();
        let arg = entry.next().unwrap();
        match command {
            "cd" => Entry::Cd(arg.to_string()),
            "dir" => Entry::Dir(arg.to_string()),
            _ => {
                let size = command.parse().unwrap();
                Entry::File(arg.to_string(), size)
            }
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    filesystem: Filesystem,
    free_space: usize,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            filesystem: Filesystem::new(),
            free_space: 70000000,
        }
    }

    pub fn interpret_entry(&mut self, entry: &str) {
        let entries = Entry::parse(entry);
        match entries {
            Entry::Cd(dir) => self.filesystem.cd(&dir),
            Entry::Dir(dir) => self.filesystem.create_dir(&dir),
            Entry::File(name, size) => {
                self.filesystem.create_file(&name, size);
                self.free_space -= size;
            }
        }
    }

    pub fn small_directories(&self) -> Vec<&Node> {
        self.filesystem.small_directories()
    }

    pub fn deletion_candidate(&self) -> Option<&Node> {
        self.filesystem
            .deletion_candidate(30000000 - self.free_space)
    }
}
