use common::*;

#[derive(Debug)]
enum Line {
    RPS(char, char),
}

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if !line.eq("") {
        let mut split = line.split(' ');
        Some(Line::RPS(
            split.next().unwrap().chars().next().unwrap(),
            split.next().unwrap().chars().next().unwrap(),
        ))
    } else {
        None
    }
}

impl Line {
    fn score1(&self) -> i32 {
        let Line::RPS(c1, c2) = self;
        match (c1, c2) {
            ('A', 'X') => 4,
            ('A', 'Y') => 8,
            ('A', 'Z') => 3,
            ('B', 'X') => 1,
            ('B', 'Y') => 5,
            ('B', 'Z') => 9,
            ('C', 'X') => 7,
            ('C', 'Y') => 2,
            ('C', 'Z') => 6,
            _ => panic!(),
        }
    }
    fn score2(&self) -> i32 {
        let Line::RPS(c1, c2) = self;
        match (c1, c2) {
            ('A', 'X') => 3,
            ('A', 'Y') => 4,
            ('A', 'Z') => 8,
            ('B', 'X') => 1,
            ('B', 'Y') => 5,
            ('B', 'Z') => 9,
            ('C', 'X') => 2,
            ('C', 'Y') => 6,
            ('C', 'Z') => 7,
            _ => panic!(),
        }
    }
}

fn solve1(lines: &Vec<Line>) -> String {
    lines
        .iter()
        .map(|line| line.score1())
        .sum::<i32>()
        .to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    lines
        .iter()
        .map(|line| line.score2())
        .sum::<i32>()
        .to_string()
}

fn main() {
    let input_lines = read_input_file("02", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
