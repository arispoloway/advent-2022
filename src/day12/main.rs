use common::*;
use priority_queue::DoublePriorityQueue;
use std::collections::HashSet;

type Line = Vec<char>;
fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if line.eq("") {
        return None;
    }
    Some(line.chars().collect())
}

type Pos = (usize, usize);

fn val(coord: Pos, grid: &Vec<Line>) -> i32 {
    let (x, y) = coord;
    let c = grid[y][x];
    if c.eq(&'S') {
        return 0;
    } else if c.eq(&'E') {
        return 27;
    } else {
        return (c as i32) - 96;
    }
}

fn neighbors(pos: Pos, grid: &Vec<Line>) -> Vec<Pos> {
    let (x, y) = pos;
    let mut n = Vec::new();
    if x > 0 {
        n.push((x - 1, y))
    }
    if y > 0 {
        n.push((x, y - 1))
    }
    if y + 1 < grid.len() {
        n.push((x, y + 1))
    }
    if x + 1 < grid[0].len() {
        n.push((x + 1, y))
    }
    n.retain(|n| val(n.clone(), grid) - val(pos, grid) <= 1);
    n
}

// Doubt this handles anything other than 1 space jumps properly, but whatever :)
fn djikstra(starts: Vec<Pos>, end: Pos, grid: &Vec<Line>) -> Option<usize> {
    let mut pq = DoublePriorityQueue::new();
    let mut seen: HashSet<Pos> = HashSet::new();
    for start in starts {
        pq.push(start, 0);
        seen.insert(start);
    }

    while let Some((cur, score)) = pq.pop_min() {
        if cur == end {
            return Some(score);
        }
        for neighbor in neighbors(cur, grid) {
            if !seen.contains(&neighbor) {
                seen.insert(neighbor);
                pq.push(neighbor, score + 1);
            }
        }
    }
    None
}

fn solve1(lines: &Vec<Line>) -> String {
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.eq(&'S') {
                start = Some((x, y))
            } else if c.eq(&'E') {
                end = Some((x, y))
            }
        }
    }
    djikstra(vec![start.unwrap()], end.unwrap(), lines)
        .unwrap()
        .to_string()
}

fn solve2(lines: &Vec<Line>) -> String {
    let mut starts: Vec<Pos> = Vec::new();
    let mut end: Option<Pos> = None;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.eq(&'S') || c.eq(&'a') {
                starts.push((x, y));
            } else if c.eq(&'E') {
                end = Some((x, y))
            }
        }
    }
    djikstra(starts, end.unwrap(), lines).unwrap().to_string()
}

fn main() {
    let input_lines = read_input_file("12", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
