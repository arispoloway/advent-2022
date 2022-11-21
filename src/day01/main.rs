use common::*;

#[derive(Debug)]
enum Line {
    IntLine(i32),
}

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if !line.eq("") {
        Some(Line::IntLine(line.parse::<i32>().unwrap()))
    } else {
        None
    }
}

fn generate_input_1(lines: &Vec<Line>) -> Input1 {
    lines
}
fn generate_input_2(lines: &Vec<Line>) -> Input2 {
    lines
}

type Input<'a> = &'a Vec<Line>;
type Input1<'a> = Input<'a>;
type Input2<'a> = Input<'a>;

#[allow(dead_code)]
struct Part1Solver<'a> {
    input: Input1<'a>,
}

impl<'a> Solver<Input1<'a>> for Part1Solver<'a> {
    fn new(input: Input1<'a>) -> Self {
        Self { input }
    }
    fn solve(&mut self) -> String {
        String::from("part 1 solution")
    }
}

#[allow(dead_code)]
struct Part2Solver<'a> {
    input: Input2<'a>,
}

impl<'a> Solver<Input2<'a>> for Part2Solver<'a> {
    fn new(input: Input2<'a>) -> Self {
        Self { input }
    }
    fn solve(&mut self) -> String {
        String::from("part 2 solution")
    }
}

fn main() {
    let input_lines = read_input_file("01", line_parser);
    println!("{:?}", input_lines);

    println!(
        "{}",
        Part1Solver::new(generate_input_1(&input_lines)).solve()
    );
    println!(
        "{}",
        Part2Solver::new(generate_input_2(&input_lines)).solve()
    );
}
