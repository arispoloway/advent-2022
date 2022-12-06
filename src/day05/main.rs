use common::*;

#[derive(Debug)]
enum Line {
    StackLayer(Vec<Option<char>>),
    Move { num: usize, from: usize, to: usize },
}

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    let chars: Vec<char> = line.chars().collect();
    if line.eq("") || chars[1].eq(&'1') {
        return None;
    }

    if chars[0].eq(&'m') {
        let mut split = line.split(" ");
        split.next();
        let num = split.next().unwrap().parse::<usize>().unwrap();
        split.next();
        let from = split.next().unwrap().parse::<usize>().unwrap();
        split.next();
        let to = split.next().unwrap().parse::<usize>().unwrap();
        Some(Line::Move { num, from, to })
    } else {
        let layer: Vec<Option<char>> = chars
            .chunks(4)
            .into_iter()
            .map(|chunk| {
                if chunk.len() <= 1 || chunk[1].eq(&' ') {
                    None
                } else {
                    Some(chunk[1])
                }
            })
            .collect();
        Some(Line::StackLayer(layer))
    }
}

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn mv_p2(&mut self, num: usize, from: usize, to: usize) {
        let from_len = self.0[from - 1].len();
        let slice_start = if from_len < num { 0 } else { from_len - num };

        let slice = self.0[from - 1][slice_start..].to_vec();

        self.0[to - 1].extend(slice);
        self.0[from - 1].truncate(slice_start);
    }

    fn mv_p1(&mut self, num: usize, from: usize, to: usize) {
        for _ in 0..num {
            if let Some(c) = self.0[from - 1].pop() {
                self.0[to - 1].push(c);
            }
        }
    }

    fn tops(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().map(|stack| *stack.last().unwrap_or(&' '))
    }
}

fn build_stacks(lines: &Vec<Line>) -> Stacks {
    let mut stacks: Vec<Vec<char>> = vec![];

    let height = lines
        .iter()
        .take_while(|l| match l {
            Line::StackLayer(_) => true,
            _ => false,
        })
        .count();

    for i in (0..height).rev() {
        let line = &lines[i];
        if let Line::StackLayer(layer) = &line {
            while layer.len() > stacks.len() {
                stacks.push(vec![]);
            }
            for (s, val) in layer.into_iter().enumerate() {
                if let Some(c) = val {
                    stacks[s].push(*c)
                }
            }
        } else {
            panic!()
        }
    }
    Stacks(stacks)
}

fn solve1(lines: &Vec<Line>) -> String {
    let mut stacks = build_stacks(lines);

    for line in lines {
        if let Line::Move { num, from, to } = line {
            stacks.mv_p1(*num, *from, *to);
        }
    }
    stacks.tops().into_iter().collect::<String>()
}
fn solve2(lines: &Vec<Line>) -> String {
    let mut stacks = build_stacks(lines);

    for line in lines {
        if let Line::Move { num, from, to } = line {
            stacks.mv_p2(*num, *from, *to);
        }
    }
    stacks.tops().into_iter().collect::<String>()
}

fn main() {
    let input_lines = read_input_file("05", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
