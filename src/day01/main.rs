use common::*;

#[derive(Debug)]
enum Line {
    IntLine(i32),
    Empty,
}

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if !line.eq("") {
        Some(Line::IntLine(line.parse::<i32>().unwrap()))
    } else {
        Some(Line::Empty)
    }
}

fn sums(lines: &Vec<Line>) -> Vec<i32> {
    let split = lines.split(|line| match line {
        Line::Empty => true,
        _ => false,
    });
    let split_sums = split.map(|lines| {
        lines
            .iter()
            .map(|line| match line {
                Line::IntLine(x) => *x,
                _ => 0,
            })
            .sum()
    });

    let mut sums: Vec<i32> = split_sums.collect();
    sums.sort();
    sums
}

fn solve1(lines: &Vec<Line>) -> String {
    let s = sums(lines);
    s.last().unwrap().to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    let s = sums(lines);
    s.iter().rev().take(3).sum::<i32>().to_string()
}

fn main() {
    let input_lines = read_input_file("01", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
