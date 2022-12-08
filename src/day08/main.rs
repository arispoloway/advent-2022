use common::*;

type Height = u32;
type Line = Vec<Height>;

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if line.eq("") {
        return None;
    }
    Some(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
}

struct Forest {
    rows: Vec<Line>,
}

enum Direction {
    North,
    South,
    East,
    West,
}

type Pos = (usize, usize);

impl Forest {
    fn new(lines: &Vec<Line>) -> Self {
        Self {
            rows: lines.iter().map(|l| l.clone()).collect(),
        }
    }

    fn get(&self, pos: Pos) -> Option<Height> {
        let (x, y) = pos;
        Some(*self.rows.get(y)?.get(x)?)
    }

    // TODO: This should return an iterator
    fn heights_to_edge(&self, pos: Pos, dir: &Direction) -> Vec<Height> {
        let (x, y) = pos;

        if self.get(pos).is_none() {
            panic!();
        }

        match dir {
            Direction::North => self.rows[0..y].iter().rev().map(|r| r[x]).collect(),
            Direction::South => self.rows[(y + 1)..].iter().map(|r| r[x]).collect(),
            Direction::West => self.rows[y][0..x].iter().rev().map(|x| *x).collect(),
            Direction::East => self.rows[y][(x + 1)..].iter().map(|x| *x).collect(),
        }
    }

    fn part1(&self) -> usize {
        let mut count: usize = 0;
        for x in 0..self.rows[0].len() {
            for y in 0..self.rows.len() {
                let val = self.get((x, y)).unwrap();

                use Direction::*;
                for dir in vec![North, South, East, West] {
                    if self.heights_to_edge((x, y), &dir).iter().all(|e| *e < val) {
                        count += 1;
                        break;
                    }
                }
            }
        }
        count
    }

    fn scenic_score(&self, pos: Pos) -> usize {
        let val = self.get(pos).unwrap();
        use Direction::*;
        vec![North, South, East, West]
            .iter()
            .map(|dir| {
                let to_edge = self.heights_to_edge(pos, dir);
                let matching_count = to_edge.iter().take_while(|h| **h < val).count();
                if matching_count == to_edge.len() {
                    matching_count
                } else {
                    matching_count + 1
                }
            })
            .product::<usize>()
    }

    fn part2(&self) -> usize {
        let mut max: usize = 0;
        for x in 1..(self.rows[0].len() - 1) {
            for y in 1..(self.rows.len() - 1) {
                let score = self.scenic_score((x, y));
                if score > max {
                    max = score
                }
            }
        }
        max
    }
}

fn solve1(lines: &Vec<Line>) -> String {
    let forest = Forest::new(lines);
    forest.part1().to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    let forest = Forest::new(lines);
    forest.part2().to_string()
}

fn main() {
    let input_lines = read_input_file("08", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
