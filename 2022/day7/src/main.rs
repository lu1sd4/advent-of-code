struct FileSystemTree {
    nodes: Vec<FileTreeNode>,
    current: usize,
}

impl FileSystemTree {
    fn new() -> Self {
        Self {
            nodes: vec![],
            current: 0,
        }
    }
    fn navigate_root(&mut self) {
        if self.nodes.is_empty() {
            self.nodes.push(FileTreeNode::create_root_node());
        }
        self.current = 0;
    }
    fn navigate_into(&mut self, folder_name: &str) {
        let current_node = &self.nodes[self.current];

        if let Some(sought_child_index) = current_node
            .children
            .iter()
            .position(|&child_index| self.nodes[child_index].name == *folder_name)
        {
            self.current = current_node.children[sought_child_index];
        } else {
            let new_index = self.nodes.len();
            let parent_index = self.current;

            let new_node = FileTreeNode::create_folder_node(folder_name, parent_index);
            self.nodes.push(new_node);

            self.nodes[parent_index].children.push(new_index);
            self.current = new_index;
        }
    }
    fn navigate_up(&mut self) {
        if let Some(new_index) = self.nodes[self.current].parent {
            self.current = new_index;
        }
    }
    fn add_directory(&mut self, folder_name: &str) {
        let new_index = self.nodes.len();
        let parent_index = self.current;
        let new_node = FileTreeNode::create_folder_node(folder_name, parent_index);
        self.nodes.push(new_node);
        self.nodes[parent_index].children.push(new_index);
    }
    fn add_file(&mut self, file_name: &str, file_size: u32) {
        let new_index = self.nodes.len();
        let parent_index = self.current;
        let new_node = FileTreeNode::create_file_node(file_name, file_size, parent_index);
        self.nodes.push(new_node);
        self.nodes[parent_index].children.push(new_index);
        self.update_current_size(file_size);
    }
    fn update_current_size(&mut self, file_size: u32) {
        let mut current_index = self.current;
        loop {
            let current_node = &mut self.nodes[current_index];
            current_node.update_size(file_size);
            if let Some(parent_index) = current_node.parent {
                current_index = parent_index;
            } else {
                break;
            }
        }
    }
}
struct FileTreeNode {
    name: String,
    size: u32,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl FileTreeNode {
    fn create_file_node(name: &str, size: u32, parent: usize) -> Self {
        Self {
            name: String::from(name),
            size,
            parent: Some(parent),
            children: vec![],
        }
    }
    fn create_folder_node(name: &str, parent: usize) -> Self {
        Self {
            name: String::from(name),
            size: 0,
            parent: Some(parent),
            children: vec![],
        }
    }
    fn create_root_node() -> Self {
        Self {
            name: String::from("/"),
            size: 0,
            parent: None,
            children: vec![],
        }
    }
    fn update_size(&mut self, file_size: u32) {
        self.size += file_size;
    }
}

trait Command {
    fn execute(&self, filesystem: &mut FileSystemTree);
    fn matches_string(string: &str) -> bool;
}

struct GoRoot;
struct GoUp;
struct GoTo {
    dir_name: String,
}
struct Ls {
    lines: Vec<String>,
}

impl Command for GoRoot {
    fn execute(&self, filesystem: &mut FileSystemTree) {
        filesystem.navigate_root();
    }
    fn matches_string(string: &str) -> bool {
        return string == "$ cd /";
    }
}

impl Command for GoUp {
    fn execute(&self, filesystem: &mut FileSystemTree) {
        filesystem.navigate_up();
    }
    fn matches_string(string: &str) -> bool {
        return string == "$ cd ..";
    }
}

impl GoTo {
    fn get_end(line: &str) -> Option<&str> {
        let prefix = "$ cd ";
        if let Some(index) = line.find(prefix) {
            return Some(&line[(index + prefix.len())..]);
        } else {
            None
        }
    }
    fn from_line(line: &str) -> Self {
        Self {
            dir_name: Self::get_end(line).unwrap().to_string(),
        }
    }
}

impl Command for Ls {
    fn execute(&self, filesystem: &mut FileSystemTree) {
        for line in self.lines.iter() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "dir" {
                filesystem.add_directory(parts[1]);
            } else {
                filesystem.add_file(parts[1], parts[0].parse::<u32>().unwrap());
            }
        }
    }
    fn matches_string(string: &str) -> bool {
        return string == "$ ls";
    }
}

impl Command for GoTo {
    fn execute(&self, filesystem: &mut FileSystemTree) {
        filesystem.navigate_into(&self.dir_name);
    }
    fn matches_string(string: &str) -> bool {
        if let Some(end) = GoTo::get_end(string) {
            return end != ".." && end != "/";
        } else {
            false
        }
    }
}

struct FsOutputProcessor {
    file_tree: FileSystemTree,
}

impl FsOutputProcessor {
    fn new() -> Self {
        Self {
            file_tree: FileSystemTree::new(),
        }
    }
    fn line_is_command(line: &str) -> bool {
        return line.starts_with("$");
    }
    fn process_lines(&mut self, input: &str) {
        let mut lines_iter = input.lines().peekable();
        while let Some(line) = lines_iter.next() {
            // println!("{}", line);
            if GoRoot::matches_string(line) {
                GoRoot.execute(&mut self.file_tree);
            } else if GoUp::matches_string(line) {
                GoUp.execute(&mut self.file_tree);
            } else if GoTo::matches_string(line) {
                GoTo::from_line(line).execute(&mut self.file_tree);
            } else if Ls::matches_string(line) {
                let mut instance_lines: Vec<String> = Vec::new();
                while Option::is_some(&lines_iter.peek())
                    && !FsOutputProcessor::line_is_command(lines_iter.peek().unwrap())
                {
                    instance_lines.push(String::from(lines_iter.next().unwrap()));
                }
                Ls {
                    lines: instance_lines,
                }
                .execute(&mut self.file_tree);
            }
        }
    }
}

fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

fn part_one(file_contents: &str) {
    let mut proc = FsOutputProcessor::new();
    proc.process_lines(file_contents);
    let mut result = 0;
    let nodes = &proc.file_tree.nodes;
    for node in nodes.iter() {
        if !node.children.is_empty() && node.size <= 100_000 {
            result += node.size;
        }
    }
    println!("{}", result);
}

fn part_two(file_contents: &str) {
    let mut proc = FsOutputProcessor::new();
    proc.process_lines(file_contents);
    let nodes = &proc.file_tree.nodes;
    let needed = 30_000_000 - (70_000_000 - nodes[0].size);
    let mut result = u32::MAX;
    for node in nodes.iter() {
        if !node.children.is_empty() && node.size > needed {
            result = u32::min(result, node.size);
        }
    }
    println!("{}", result);
}
