use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum FSItem {
    File(File),
    Directory(Directory),
}

impl FSItem {
    fn new(curr_path: &str, info: &str, name: &str) -> FSItem {
        if info == "dir" {
            let directory_path = curr_path.to_owned() + &"/".to_owned() + &name.to_string();
            FSItem::Directory(Directory {
                name: directory_path,
                children: Vec::new(),
            })
        } else {
            FSItem::File(File {
                _name: curr_path.to_owned() + &"/".to_owned() + &name.to_string(),
                size: info.parse().unwrap(),
            })
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<FSItem>,
}

#[derive(Debug)]
struct File {
    _name: String,
    size: u64,
}

fn main() {
    let input = include_str!("../input");

    let root: Directory = build_fs(&input);
    let sizes = compute_directories_sizes(&root);

    let result: u64 = sizes
        .iter()
        .filter(|&(_dir, size)| *size <= 100_000)
        .map(|(_dir, size)| size)
        .sum();

    println!("{}", result);
}

fn compute_directories_sizes(root: &Directory) -> HashMap<&str, u64> {
    compute_directory_sizes(root).1
}

fn compute_directory_sizes(dir: &Directory) -> (u64, HashMap<&str, u64>) {
    let mut sizes = HashMap::new();
    let mut total_children_size = 0;
    for item in &dir.children {
        match item {
            FSItem::File(file) => {
                total_children_size += file.size;
            }
            FSItem::Directory(child_directory) => {
                let (children_size, all) = compute_directory_sizes(child_directory);

                total_children_size += children_size;
                sizes = sizes.into_iter().chain(all).collect();
            }
        }
    }

    sizes.insert(&dir.name, total_children_size);
    (total_children_size, sizes)
}

fn build_fs(input: &str) -> Directory {
    let mut root_directory = Directory {
        name: "/".to_string(),
        children: Vec::new(),
    };
    let mut directory_path: VecDeque<&str> = VecDeque::new();

    let lines = input.lines().collect::<Vec<_>>();
    let mut i = 0;
    while i < lines.len() {
        let current = lines[i];
        if current == "$ ls" {
            if directory_path.back().unwrap() == &"/" {
                let contents: Vec<_> = get_directory_children(input, i, "");
                i += &contents.len() + 1;
                root_directory = Directory {
                    name: "/".to_owned(),
                    children: contents,
                };
            } else {
                let curr_path = "/".to_owned()
                    + &directory_path
                        .clone()
                        .into_iter()
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join("/")
                        .to_owned();

                let contents = get_directory_children(input, i, &curr_path);

                let mut bfs_queue = VecDeque::new();
                bfs_queue.push_back(&mut root_directory);

                'bfs: while !bfs_queue.is_empty() {
                    let current = bfs_queue.pop_front().unwrap();
                    for item in &mut current.children {
                        if let FSItem::Directory(dir) = item {
                            let childpath = (&dir.name).to_owned();
                            if childpath == curr_path {
                                dir.children = get_directory_children(input, i, &curr_path);
                                break 'bfs;
                            } else {
                                bfs_queue.push_back(dir);
                            }
                        }
                    }
                }
                i += &contents.len() + 1;
            }
        } else {
            let target_directory = &current[5..];
            if target_directory == ".." {
                directory_path.pop_back();
            } else {
                directory_path.push_back(target_directory);
            }

            i += 1;
        }
    }
    root_directory
}

fn get_directory_children(input: &str, i: usize, curr_path: &str) -> Vec<FSItem> {
    input
        .lines()
        .into_iter()
        .skip(i + 1)
        .take_while(|line| !line.starts_with("$"))
        .map(|line| line.trim().split_once(" ").unwrap())
        .map(|(info, name)| FSItem::new(curr_path, info, name))
        .collect()
}
