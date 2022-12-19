use std::iter::Peekable;
use std::str::Split;
use std::{collections::HashMap, fs};

use crate::PROJECT_DIRECTORY;

mod dir {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Node {
        parent: Option<String>,
        children_dirs: Vec<String>,
        children_files: HashMap<String, usize>,
    }

    impl Node {
        pub fn new(parent: Option<String>) -> Self {
            Node {
                parent,
                children_dirs: Vec::new(),
                children_files: HashMap::new(),
            }
        }

        pub fn add_file(&mut self, name: String, size: usize) {
            self.children_files.insert(name.clone(), size);
        }

        pub fn add_dir(&mut self, name: String) {
            self.children_dirs.push(name);
        }

        pub fn parent(&self) -> Option<String> {
            self.parent.clone()
        }

        pub fn dirs(&self) -> core::slice::Iter<String> {
            self.children_dirs.iter()
        }

        pub fn files(&self) -> &HashMap<String, usize> {
            &self.children_files
        }
    }
}

#[derive(PartialEq)]
enum Command {
    LS,
    CD,
}

fn parse_command_call(cmd_call: &str) -> (Command, Option<String>) {
    let mut tokens = cmd_call.split_whitespace();
    tokens.next(); // skip $
    let cmd = tokens.next().unwrap();
    if cmd.contains("ls") {
        return (Command::LS, None);
    } else {
        let arg = tokens.next().unwrap();
        return (Command::CD, Some(arg.trim().to_string()));
    }
}

#[derive(PartialEq)]
enum NodeType {
    FILE,
    DIRECTORY,
}

fn parse_ls_output_line(line: &str) -> (NodeType, String, Option<usize>) {
    let mut tokens = line.split_whitespace();
    let dir_or_size = tokens.next().unwrap();
    let name = tokens.next().unwrap();
    if dir_or_size.contains("dir") {
        return (NodeType::DIRECTORY, name.to_string(), None);
    }
    return (
        NodeType::FILE,
        name.to_string(),
        Some(str::parse::<usize>(dir_or_size).unwrap()),
    );
}

fn handle_ls(
    lines: &mut Peekable<Split<char>>,
    cwd: &String,
    directories: &mut HashMap<String, dir::Node>,
) {
    let mut current_line = lines.next();
    while current_line.is_some()
        && !current_line.unwrap().starts_with('$')
        && !current_line.unwrap().is_empty()
    {
        let (t, n, s) = parse_ls_output_line(current_line.unwrap());
        if t == NodeType::FILE {
            let mut full_name = cwd.clone();
            full_name.push_str(&n);
            directories
                .get_mut(cwd)
                .unwrap()
                .add_file(full_name, s.unwrap());
        } else if t == NodeType::DIRECTORY {
            let mut full_name = cwd.clone();
            full_name.push_str(&n);
            full_name.push_str("/");
            let new_node = dir::Node::new(Some(cwd.clone()));
            directories.insert(full_name.clone(), new_node);
            directories.get_mut(cwd).unwrap().add_dir(full_name);
        }
        if lines.peek().is_some() && !lines.peek().unwrap().starts_with("$") {
            current_line = lines.next();
        } else {
            break;
        }
    }
}

fn dirname_from_dirarg(
    cwd: &String,
    directories: &HashMap<String, dir::Node>,
    arg: String,
) -> String {
    if arg.contains("..") {
        let cwd_node = directories.get(cwd).unwrap();
        let parent = cwd_node.parent().unwrap().clone();
        return parent;
    }
    let mut full_name = cwd.clone();
    full_name.push_str(&arg);
    full_name.push_str("/");
    return full_name.clone();
}

fn build_dir_tree(input: String) -> HashMap<String, dir::Node> {
    let mut lines = input.split('\n').peekable();
    let mut current_line = lines.next();
    let mut directories = HashMap::new();

    if !current_line.unwrap().contains("$ cd /") {
        panic!("First line should enter root dir");
    }
    let root = dir::Node::new(None);
    directories.insert(String::from("/"), root);

    current_line = lines.next();
    let mut cwd = String::from("/");
    while current_line.is_some() {
        let cl = current_line.unwrap();

        if cl.starts_with('$') {
            let (cmd, args) = parse_command_call(cl);
            if cmd == Command::LS {
                handle_ls(&mut lines, &cwd, &mut directories);
            } else if cmd == Command::CD {
                cwd = dirname_from_dirarg(&cwd, &directories, args.unwrap());
            }
        } else {
            panic!("Wrong implementation!");
        }
        current_line = lines.next();
    }

    directories
}

fn calculate_dir_size(
    directory: &String,
    directories: &HashMap<String, dir::Node>,
    sizes: &mut HashMap<String, usize>,
) -> usize {
    if sizes.contains_key(directory) {
        return *sizes.get(directory).unwrap();
    }
    let node = directories.get(directory).unwrap();
    let mut sub_dir_sum: usize = 0;
    for child_dir in node.dirs() {
        sub_dir_sum += calculate_dir_size(child_dir, directories, sizes);
    }

    let mut file_sum: usize = 0;
    for child_file in node.files() {
        file_sum += child_file.1;
    }
    sizes.insert(directory.clone(), sub_dir_sum + file_sum);
    return sub_dir_sum + file_sum;
}

fn calculate_dir_sizes(
    directories: &HashMap<String, dir::Node>,
    sizes: &mut HashMap<String, usize>,
) {
    for dir in directories.keys() {
        calculate_dir_size(dir, directories, sizes);
    }
}

fn print_file(file: &String, size: &usize, depth: usize) {
    let indent = str::repeat("--", depth);
    let file_name = file.split('/').last().unwrap().to_string();
    println!("{}{} {}", indent, file_name, size);
}

fn print_dir(
    directory: &String,
    directories: &HashMap<String, dir::Node>,
    directory_sizes: &HashMap<String, usize>,
    depth: usize,
) {
    let indent = str::repeat("--", depth);
    let mut relative_name = directory.split('/').nth_back(1).unwrap().to_string();
    if relative_name.is_empty() {
        relative_name = directory.clone();
    }
    println!(
        "{}{} dir {}",
        indent,
        relative_name,
        directory_sizes.get(directory).unwrap()
    );

    for file in directories.get(directory).unwrap().files() {
        print_file(file.0, file.1, depth + 1)
    }
    for child in directories.get(directory).unwrap().dirs() {
        print_dir(child, directories, directory_sizes, depth + 1);
    }
}

fn print_file_system(
    directories: &HashMap<String, dir::Node>,
    directory_sizes: &HashMap<String, usize>,
) {
    print_dir(&String::from("/"), directories, directory_sizes, 0);
}

fn part1() {
    let input = fs::read_to_string(format!("{}/day7/input.txt", PROJECT_DIRECTORY)).unwrap();
    let dirs = build_dir_tree(input);

    let mut sizes = HashMap::new();
    calculate_dir_sizes(&dirs, &mut sizes);

    let mut size_vec = Vec::new();
    for size in sizes {
        size_vec.push(size.1);
    }

    size_vec.sort();

    let mut sum = 0;
    for size in size_vec {
        if size <= 100000 {
            sum += size;
        }
    }
    println!("{}", sum);
}

fn part2() {
    let total_fs_size = 70000000;
    let total_needed = 30000000;
    let input = fs::read_to_string(format!("{}/day7/input.txt", PROJECT_DIRECTORY)).unwrap();
    let dirs = build_dir_tree(input);

    let mut sizes = HashMap::new();
    calculate_dir_sizes(&dirs, &mut sizes);

    let mut size_vec = Vec::new();
    let used_size = sizes.get("/").unwrap().clone();

    if used_size < total_needed || used_size > total_fs_size {
        panic!("Wrong numbers!");
    }

    let need_to_free = total_needed - (total_fs_size - used_size);

    print_file_system(&dirs, &sizes);
    for size in sizes {
        size_vec.push(size.1);
    }

    size_vec.sort();
    for size in size_vec {
        if size >= need_to_free {
            println!("Need to free: {}", size);
            break;
        }
    }
}

pub fn solve() {
    part1();
    part2();
}
