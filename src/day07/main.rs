use common::*;
use std::collections::HashMap;

type Size = u128;

type FileName = String;
type DirName = String;

#[derive(Debug)]
struct File {
    name: String,
    size: Size,
}

impl File {
    fn new(name: &str, size: Size) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug)]
struct Dir {
    name: DirName,
    files: HashMap<FileName, File>,
    dirs: HashMap<DirName, Dir>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }

    fn add_dir(&mut self, dir: Dir) {
        self.dirs.insert(dir.name.clone(), dir);
    }

    // TODO: Ideally this would be an iterator, not a vec
    fn all_dirs(&self) -> Vec<&Self> {
        let mut dirs = self.dirs.values().collect::<Vec<_>>();

        for dir in self.dirs.values() {
            dirs.extend(dir.all_dirs());
        }

        dirs
    }
}

#[derive(Debug)]
struct Environment {
    root: Dir,
    current_path: Vec<DirName>,
}

trait WithSize {
    fn size(&self) -> Size;
}

impl WithSize for File {
    fn size(&self) -> Size {
        self.size
    }
}

impl WithSize for Dir {
    // Without caching the size or somehow keeping track of dir size this is
    // horribly innefficcient
    fn size(&self) -> Size {
        let mut size: Size = 0;

        for file in self.files.values() {
            size += file.size();
        }
        for dir in self.dirs.values() {
            size += dir.size();
        }

        size
    }
}

impl Environment {
    fn new() -> Self {
        Self {
            root: Dir::new("/"),
            current_path: Vec::new(),
        }
    }

    fn change_dir(&mut self, dir_name: &String) {
        if dir_name.eq("..") {
            self.current_path.pop();
        } else if dir_name.eq("/") {
            self.current_path.clear();
        } else {
            self.current_path.push(dir_name.clone())
        }
    }

    fn add_file(&mut self, file: File) {
        let mut dir = &mut self.root;
        for p in &self.current_path {
            dir = dir.dirs.get_mut(p).unwrap();
        }

        dir.add_file(file);
    }

    fn add_dir(&mut self, d: Dir) {
        let mut dir = &mut self.root;
        for p in &self.current_path {
            dir = dir.dirs.get_mut(p).unwrap();
        }

        dir.add_dir(d);
    }

    fn part1(&self) -> Size {
        let mut sum: Size = 0;

        for dir in self.root.all_dirs() {
            let s = dir.size();
            if s < 100_000 {
                sum += s
            }
        }
        sum
    }

    fn part2(&self) -> Size {
        let mut n: Size = 999999999;

        let used = self.root.size();
        let free = 70_000_000 - used;
        let needed = 30_000_000 - free;

        for dir in self.root.all_dirs() {
            let s = dir.size();
            if s > needed && s < n {
                n = s
            }
        }
        n
    }
}

#[derive(Debug)]
enum Command {
    ChangeDir(DirName),
    ListDir,
}

#[derive(Debug)]
enum CmdResult {
    LsDir { dir_name: DirName },
    File { size: Size, file_name: FileName },
}

#[derive(Debug)]
enum Line {
    CommandLine(Command),
    ResultLine(CmdResult),
}

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if line.eq("") {
        return None;
    }
    let tokens: Vec<&str> = line.split(" ").collect();

    if tokens[0].eq("$") {
        if tokens[1].eq("cd") {
            Some(Line::CommandLine(Command::ChangeDir(tokens[2].to_string())))
        } else {
            Some(Line::CommandLine(Command::ListDir))
        }
    } else if tokens[0].eq("dir") {
        Some(Line::ResultLine(CmdResult::LsDir {
            dir_name: tokens[1].to_string(),
        }))
    } else {
        Some(Line::ResultLine(CmdResult::File {
            size: tokens[0].parse().unwrap(),
            file_name: tokens[1].to_string(),
        }))
    }
}

fn build_environment(env: &mut Environment, lines: &Vec<Line>) {
    let mut i: usize = 0;

    while i < lines.len() {
        let cmd_line = &lines[i];

        if let Line::CommandLine(cmd) = cmd_line {
            if let Command::ChangeDir(dir_name) = cmd {
                env.change_dir(dir_name);
            } else {
                while i + 1 < lines.len() {
                    if let Line::ResultLine(result) = &lines[i + 1] {
                        match result {
                            CmdResult::LsDir { dir_name } => env.add_dir(Dir::new(dir_name)),
                            CmdResult::File { size, file_name } => {
                                env.add_file(File::new(file_name, *size))
                            }
                        }

                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        i += 1;
    }
}

fn solve1(lines: &Vec<Line>) -> String {
    let mut env = Environment::new();
    build_environment(&mut env, lines);
    env.part1().to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    let mut env = Environment::new();
    build_environment(&mut env, lines);
    env.part2().to_string()
}

fn main() {
    let input_lines = read_input_file("07", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
