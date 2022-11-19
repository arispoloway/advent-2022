use common::*;

fn parser(line: &str) -> i32 {
    line.parse::<i32>().unwrap()
}

fn main() {
    let input_lines = read_input_file("01", parser);
    println!("{:?}", input_lines);
}
