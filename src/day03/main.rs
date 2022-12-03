use common::*;
use std::collections::HashSet;

type Line = String;

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if !line.eq("") {
        Some(Line::from(line))
    } else {
        None
    }
}

fn l_score(l: char) -> i32 {
    let r = l as i32;
    if r > 95 {
        r - 96
    } else {
        r - 38
    }
}

fn p1_score(l: &Line) -> i32 {
    let half = l.len() / 2;
    let last = HashSet::<_>::from_iter(l[half..].chars());
    for item in l[..half].chars() {
        if last.contains(&item) {
            return l_score(item);
        }
    }
    panic!()
}

fn p2_score(chunk: &[Line]) -> i32 {
    let mut iter = chunk.iter();
    let set = HashSet::<_>::from_iter(iter.next().unwrap().chars());
    let set2 = HashSet::<_>::from_iter(iter.next().unwrap().chars());
    let set3 = HashSet::<_>::from_iter(iter.next().unwrap().chars());
    for item in set.intersection(&set2) {
        if set3.contains(item) {
            return l_score(*item);
        }
    }

    panic!()
}

fn solve1(lines: &Vec<Line>) -> String {
    lines
        .iter()
        .map(|line| p1_score(line))
        .sum::<i32>()
        .to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    lines
        .chunks(3)
        .map(|line| p2_score(line))
        .sum::<i32>()
        .to_string()
}

fn main() {
    let input_lines = read_input_file("03", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
