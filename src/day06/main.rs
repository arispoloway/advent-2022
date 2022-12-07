use common::*;
use std::collections::HashSet;

type Line = String;

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if line.eq("") {
        return None;
    }

    Some(line.to_string())
}

fn any_eq(chars: &str) -> bool {
    let set = HashSet::<_>::from_iter(chars.chars());
    set.len() != chars.len()
}

fn find_stop(line: &String, size: usize) -> Option<usize> {
    for i in 0..(line.len() - size) {
        if !any_eq(&line[i..(i + size)]) {
            return Some(i + size);
        }
    }
    None
}

fn solve1(lines: &Vec<Line>) -> String {
    let l = lines.first().unwrap();

    find_stop(l, 4).unwrap().to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    let l = lines.first().unwrap();

    find_stop(l, 14).unwrap().to_string()
}

fn main() {
    let input_lines = read_input_file("06", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
