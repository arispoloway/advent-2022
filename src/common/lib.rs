use std::fs;

pub fn read_input_file<T>(day: &str, line_parser: fn(&str, &Vec<T>) -> Option<T>) -> Vec<T> {
    let source_file: String = format!("inputs/day{}.txt", day);

    let mut result: Vec<T> = Vec::new();
    for line in lines(source_file.as_str()) {
        if let Some(val) = line_parser(line.as_str(), &result) {
            result.push(val);
        }
    }

    result
}

pub fn lines(filename: &str) -> Vec<String> {
    let file_string = fs::read_to_string(filename).expect("Something went wrong reading the file");
    file_string
        .split("\n")
        .map(|x| x.trim_end().to_string())
        .collect()
}
