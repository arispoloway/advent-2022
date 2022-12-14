use common::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
struct Packet {
    list: Option<Vec<Box<Packet>>>,
    val: Option<i32>,
}

impl Packet {
    fn from_val(val: i32) -> Self {
        Self {
            list: None,
            val: Some(val),
        }
    }

    fn new_list() -> Self {
        Self {
            list: Some(Vec::new()),
            val: None,
        }
    }
    fn list_push(&mut self, packet: Packet) {
        self.list_mut().push(Box::new(packet));
    }

    fn is_list(&self) -> bool {
        self.list.is_some()
    }

    fn is_val(&self) -> bool {
        self.val.is_some()
    }

    fn list_from_val(&self) -> Self {
        Self {
            list: Some(vec![Box::new(Packet::from_val(self.val()))]),
            val: None,
        }
    }

    fn val(&self) -> i32 {
        self.val.unwrap()
    }

    fn list(&self) -> &Vec<Box<Packet>> {
        self.list.as_ref().unwrap()
    }

    fn list_mut(&mut self) -> &mut Vec<Box<Packet>> {
        self.list.as_mut().unwrap()
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        if let Ordering::Equal = self.cmp(other) {
            return true;
        }
        false
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_val() && other.is_val() {
            return self.val().cmp(&other.val());
        } else if self.is_list() && other.is_list() {
            for (p1, p2) in self.list().iter().zip(other.list().iter()) {
                match p1.cmp(p2) {
                    Ordering::Equal => {}
                    x => return x,
                }
            }

            // Left ran out of items first
            if self.list().len() < other.list().len() {
                return Ordering::Less;
            } else if self.list().len() > other.list().len() {
                return Ordering::Greater;
            }
            Ordering::Equal
        } else if self.is_list() {
            return self.cmp(&other.list_from_val());
        } else {
            return self.list_from_val().cmp(&other);
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_val() {
            write!(f, "{}", self.val())
        } else {
            write!(f, "[")?;
            let mut first = true;
            for item in self.list() {
                if first {
                    first = false;
                    write!(f, "{}", item)?;
                } else {
                    write!(f, ",{}", item)?;
                }
            }
            write!(f, "]")
        }
    }
}

#[derive(Debug)]
enum Line {
    Entry(Packet),
    Empty,
}

fn parse_packet(line: &str) -> (Packet, usize) {
    if line.chars().nth(0).unwrap().eq(&'[') {
        let mut packet = Packet::new_list();
        let mut end_idx = 1;
        while line.chars().nth(end_idx).unwrap() != ']' {
            let (sub_packet, idx) = parse_packet(&line[end_idx..]);
            packet.list_push(sub_packet);
            end_idx += idx;
        }
        if line.chars().nth(end_idx + 1).unwrap_or(' ') == ',' {
            (packet, end_idx + 2)
        } else {
            (packet, end_idx + 1)
        }
    } else {
        match line.find(|x| x == ']' || x == ',') {
            Some(idx) => (
                Packet::from_val(line[..idx].parse().unwrap()),
                if line.chars().nth(idx).unwrap() == ',' {
                    idx + 1
                } else {
                    idx
                },
            ),
            _ => (Packet::from_val(line.parse().unwrap()), line.len()),
        }
    }
}

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if line.eq("") {
        return Some(Line::Empty);
    }

    Some(Line::Entry(parse_packet(line).0))
}

fn solve1(lines: &Vec<Line>) -> String {
    let mut cur: Vec<&Packet> = Vec::new();
    let mut cur_pair = 1;
    let mut sum: usize = 0;

    for line in lines {
        match line {
            Line::Entry(packet) => cur.push(packet),
            Line::Empty => {
                if cur.windows(2).all(|w| w[0] < w[1]) {
                    sum += cur_pair;
                }
                cur.clear();
                cur_pair += 1;
            }
        }
    }

    sum.to_string()
}

fn solve2(lines: &Vec<Line>) -> String {
    let mut packets: Vec<&Packet> = Vec::new();
    for line in lines {
        match line {
            Line::Entry(packet) => packets.push(packet),
            _ => {}
        }
    }

    let mut p1 = Packet::new_list();
    p1.list_push(Packet::from_val(2).list_from_val());
    let mut p2 = Packet::new_list();
    p2.list_push(Packet::from_val(6).list_from_val());
    packets.push(&p1);
    packets.push(&p2);
    packets.sort();

    let idx1 = packets.iter().position(|elem| (*elem).eq(&p1)).unwrap();
    let idx2 = packets.iter().position(|elem| (*elem).eq(&p2)).unwrap();

    ((idx1 + 1) * (idx2 + 1)).to_string()
}

fn main() {
    let input_lines = read_input_file("13", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
