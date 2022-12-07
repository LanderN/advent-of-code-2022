use std::{collections::HashMap, vec};

use advent_of_code_2022::get_input;
use itertools::Itertools;

#[derive(Debug)]
struct Directory {
    parent: Vec<String>,
    name: String,
    dirs: Vec<Directory>,
    files: HashMap<String, u64>,
}

impl Directory {
    fn has_dir(&self, name: &str) -> bool {
        self.dirs.iter().any(|d| d.name == name)
    }

    pub fn ensure_dir_exists(&mut self, name: &str) {
        if !self.has_dir(name) {
            self.dirs.push(Directory {
                parent: self
                    .parent
                    .iter()
                    .cloned()
                    .chain(vec![self.name.to_string()].into_iter())
                    .collect(),
                name: name.to_string(),
                dirs: vec![],
                files: HashMap::new(),
            })
        }
    }

    pub fn dir(&mut self, name: &str) -> &mut Directory {
        self.dirs.iter_mut().find(|d| d.name == name).unwrap()
    }

    pub fn add_file(&mut self, name: &str, size: u64) {
        self.files.insert(name.to_string(), size);
    }

    pub fn total_size(&self) -> u64 {
        let mut total_size = 0;
        for dir in &self.dirs {
            total_size += dir.total_size();
        }
        for size in self.files.values() {
            total_size += size;
        }

        total_size
    }

    pub fn find_dirs<F>(&self, filter: &F) -> Vec<&Directory>
    where
        F: Fn(&Directory) -> bool,
    {
        let mut result: Vec<&Directory> = vec![];
        if filter(self) {
            result.push(self);
        }

        for dir in &self.dirs {
            result.extend(dir.find_dirs(filter));
        }

        result
    }

    pub fn print(&self, indent: usize) {
        println!(
            "{}- {} (dir, size={})",
            vec![" "; indent].join(""),
            self.name,
            self.total_size()
        );
        for dir in &self.dirs {
            dir.print(indent + 2);
        }
        for (file, size) in &self.files {
            println!(
                "{}- {} (file, size={})",
                vec![" "; indent + 2].join(""),
                file,
                size
            );
        }
    }
}

fn main() {
    let _demo_input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let input = get_input("day-7/input");

    let mut root_dir = Directory {
        name: "/".to_string(),
        dirs: vec![],
        files: HashMap::new(),
        parent: vec![],
    };
    let mut current_dir = &mut root_dir;
    let mut current_path: Vec<String> = vec![];

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            // Command
            let command = &line[2..];
            match command.split(' ').collect_vec()[..] {
                ["cd", dir] => match dir {
                    ".." => {
                        current_path.pop();
                        current_dir = &mut root_dir;
                        for el in &current_path {
                            current_dir = current_dir.dir(el);
                        }
                    }
                    "/" => {
                        current_path = vec![];
                        current_dir = &mut root_dir;
                    }
                    _ => {
                        current_dir.ensure_dir_exists(dir);
                        current_path.push(dir.to_string());
                        current_dir = current_dir.dir(dir);
                    }
                },
                ["ls"] => {
                    // Next few lines will be results from ls
                    while let Some(line) = lines.peek() {
                        if line.starts_with('$') {
                            break;
                        }

                        let file_or_dir = lines.next().unwrap();
                        match file_or_dir.split(' ').collect_vec()[..] {
                            ["dir", dirname] => {
                                current_dir.ensure_dir_exists(dirname);
                            }
                            [size, file] => {
                                current_dir.add_file(file, size.parse().unwrap());
                            }
                            _ => panic!("Could not parse ls output!"),
                        }
                    }
                }
                _ => panic!("unknown command!"),
            }
        }
    }

    //root_dir.print(0);
    println!(
        "Part one answer: {:?}",
        root_dir
            .find_dirs(&|d| d.total_size() <= 100000)
            .iter()
            .map(|d| d.total_size())
            .sum::<u64>()
    );

    let in_use = root_dir.total_size();
    let fs_size = 70000000;
    let space_left = fs_size - in_use;
    let space_needed = 30000000;
    let to_free = space_needed - space_left;

    println!(
        "Part two answer: {:?}",
        root_dir
            .find_dirs(&|d| d.total_size() >= to_free)
            .iter()
            .sorted_by(|x, y| x.total_size().cmp(&y.total_size()))
            .take(1)
            .collect_vec()
            .first()
            .unwrap()
            .total_size()
    );
}
