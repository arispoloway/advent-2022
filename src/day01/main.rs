use std::fs;

fn main() {
    let mut int_lines: Vec<i32> = Vec::new();
    for line in lines("inputs/01.txt") {
        int_lines.push(line.parse().unwrap());
    }
}

fn lines(filename: &str) -> Vec<String> {
    let file_string = fs::read_to_string(filename).expect("Something went wrong reading the file");
    file_string
        .split("\n")
        .map(|x| x.trim_end().to_string())
        .collect()
}
