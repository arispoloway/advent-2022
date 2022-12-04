use common::*;

struct Line((i32, i32), (i32, i32));

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if !line.eq("") {
        let mut parts = line.split(|c| c == ',' || c == '-').into_iter();

        Some(Line(
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            ),
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            ),
        ))
    } else {
        None
    }
}

enum OverlapState {
    NonOverlapping,
    PartialOverlap,
    FullOverlap,
}

impl Line {
    fn overlap(&self) -> OverlapState {
        if self.0 .1 < self.1 .0 || self.0 .0 > self.1 .1 {
            OverlapState::NonOverlapping
        } else if (self.0 .0 >= self.1 .0 && self.0 .1 <= self.1 .1)
            || (self.1 .0 >= self.0 .0 && self.1 .1 <= self.0 .1)
        {
            OverlapState::FullOverlap
        } else {
            OverlapState::PartialOverlap
        }
    }
}

fn solve1(lines: &Vec<Line>) -> String {
    lines
        .iter()
        .filter(|line| match line.overlap() {
            OverlapState::FullOverlap => true,
            _ => false,
        })
        .count()
        .to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    lines
        .iter()
        .filter(|line| match line.overlap() {
            OverlapState::PartialOverlap => true,
            OverlapState::FullOverlap => true,
            _ => false,
        })
        .count()
        .to_string()
}

fn main() {
    let input_lines = read_input_file("04", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
