use common::*;
use std::collections::HashSet;

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if line.eq("") {
        return None;
    }

    let mut split = line.split(" ");
    Some(Line {
        dir: Direction::from(split.next().unwrap()).unwrap(),
        amount: split.next().unwrap().parse().unwrap(),
    })
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(s: &str) -> Option<Self> {
        match s {
            "R" => return Some(Self::Right),
            "L" => return Some(Self::Left),
            "U" => return Some(Self::Up),
            "D" => return Some(Self::Down),
            _ => None,
        }
    }
}

struct Line {
    dir: Direction,
    amount: usize,
}

type Pos = (i32, i32);

struct Rope {
    knots: Vec<Pos>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut knots: Vec<Pos> = vec![];
        for _ in 0..size {
            knots.push((0, 0));
        }
        Self { knots }
    }

    fn move_head(&mut self, dir: &Direction) {
        use Direction::*;
        let (x, y) = self.knots[0];

        self.knots[0] = match dir {
            Right => (x + 1, y),
            Left => (x - 1, y),
            Up => (x, y + 1),
            Down => (x, y - 1),
        };

        for i in 1..self.knots.len() {
            let (hx, hy) = self.knots[i - 1];
            let (rx, ry) = self.knots[i];
            if hx.abs_diff(rx).max(hy.abs_diff(ry)) <= 1 {
                break;
            }
            self.knots[i] = Self::new_rope_pos(self.knots[i - 1], self.knots[i]);
        }
    }

    fn tail_pos(&self) -> Pos {
        self.knots.last().unwrap().clone()
    }

    fn new_rope_pos(head: Pos, knot: Pos) -> Pos {
        let (hx, hy) = head;
        let (rx, ry) = knot;
        let dx = hx.abs_diff(rx);
        let dy = hy.abs_diff(ry);

        if dx > dy {
            ((hx + rx) / 2, hy)
        } else if dy > dx {
            (hx, (hy + ry) / 2)
        } else {
            ((hx + rx) / 2, (hy + ry) / 2)
        }
    }
}

fn solve1(lines: &Vec<Line>) -> String {
    let mut rope = Rope::new(2);
    let mut positions = HashSet::new();

    for Line { dir, amount } in lines {
        for _ in 0..*amount {
            rope.move_head(dir);
            positions.insert(rope.tail_pos());
        }
    }
    positions.len().to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    let mut rope = Rope::new(10);
    let mut positions = HashSet::new();

    for Line { dir, amount } in lines {
        for _ in 0..*amount {
            rope.move_head(dir);
            positions.insert(rope.tail_pos());
        }
    }
    positions.len().to_string()
}

fn main() {
    let input_lines = read_input_file("09", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
