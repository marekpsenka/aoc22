use core::slice::Iter;
use std::iter::Peekable;
use itertools::Itertools;

#[derive(Debug)]
struct File {
    size: usize,
    #[allow(dead_code)]
    name: String
}

#[derive(Debug)]
struct Directory {
    #[allow(dead_code)]
    name: String,
    files: Vec<File>,
    subdirs: Vec<Directory>
}

impl Directory {
    fn files_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
    }
}

fn parse_dirname_from_cd(s: &str) -> String {
    let (_, cmd, dir_name) = s
        .split_ascii_whitespace()
        .collect_tuple()
        .expect("Line can be collected to 3-element tuple");

    if cmd != "cd" {
        panic!("Expected cd command");
    }

    dir_name.to_string()
}

fn get_sizes_of_dirs(result: &mut Vec<usize>,  dir: &Directory) -> usize {
    let mut this_dir_size = 0usize;
    if !dir.subdirs.is_empty() {
        dir.subdirs.iter().for_each(
            |sd| { this_dir_size += get_sizes_of_dirs(result, sd); }
        )
    }

    this_dir_size += dir.files_size();

    (*result).push(this_dir_size);

    this_dir_size
}

fn parse_dir(dir_name: String, lines: &mut Peekable<Iter<String>>) -> Directory {
    let mut files = Vec::<File>::new();
    let mut subdirs = Vec::<Directory>::new();

    _ = lines.next().expect("line containing 'ls' is yielded");

    while let Some(line) = lines.next_if(|l| !l.starts_with("$ ")) {
        let (p1, p2) = line.split_ascii_whitespace().collect_tuple().expect("Either dir or file");
        if p1 == "dir" {

        }
        else {
            let size = p1.parse::<usize>().expect("Size of file is found in first part");
            files.push(File {size, name:p2.to_string()})
        }
    }

    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            break;
        }
        else {
            let subdir_name = parse_dirname_from_cd(line);
            subdirs.push(parse_dir(subdir_name, lines));
        }
    }

    Directory { name: dir_name, files, subdirs }
}

fn main() {
    let lines =std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.unwrap())
        .collect::<Vec<String>>();

    let mut k = lines.iter().peekable();
    let root_name = parse_dirname_from_cd(k.next().expect("First line is root"));
    let dir = parse_dir(root_name, &mut k);

    let mut sizes = Vec::<usize>::new();
    let total_size = get_sizes_of_dirs(&mut sizes, &dir);

    let limit = 70_000_000usize;
    let unused = limit - total_size;

    let smallest_to_delete_size = sizes
        .iter()
        .sorted()
        .find(|&size| unused + size >= 30_000_000)
        .expect("There is such a size");

    println!("{smallest_to_delete_size}");
}
