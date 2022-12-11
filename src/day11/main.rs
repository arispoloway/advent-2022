use common::*;
use std::collections::HashMap;

enum Line {
    MonkeyN(MonkeyNum),
    Items(Vec<Item>),
    Operation(Operator),
    Divisible(usize),
    TrueMonkey(MonkeyNum),
    FalseMonkey(MonkeyNum),
}

#[derive(Clone)]
enum OperationTarget {
    Itself,
    Num(Item),
}

#[derive(Clone)]
struct Operator {
    operator: char,
    target: OperationTarget,
}

impl Operator {
    fn new(operator: char, target: OperationTarget) -> Self {
        Self { operator, target }
    }

    fn call(&self, item: Item) -> Item {
        use OperationTarget::*;

        match self.operator {
            '*' => {
                if let Num(x) = &self.target {
                    item * x
                } else {
                    item * item
                }
            }
            '+' => {
                if let Num(x) = &self.target {
                    item + x
                } else {
                    item + item
                }
            }
            _ => panic!(),
        }
    }
}

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    use Line::*;

    if line.eq("") {
        return None;
    }

    if line.chars().nth(0).unwrap() == 'M' {
        return Some(MonkeyN(
            line.chars().nth(7).unwrap().to_digit(10).unwrap() as usize
        ));
    }

    if line.chars().nth(2).unwrap() == 'S' {
        return Some(Items(
            line.split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|e| e.parse().unwrap())
                .collect(),
        ));
    }

    if line.chars().nth(2).unwrap() == 'O' {
        let mut op_str = line.split("old ").nth(1).unwrap().split(" ");
        let operator = op_str.next().unwrap();
        let op_target = op_str.next().unwrap();

        let operator = if op_target.eq("old") {
            Operator::new(operator.chars().nth(0).unwrap(), OperationTarget::Itself)
        } else {
            Operator::new(
                operator.chars().nth(0).unwrap(),
                OperationTarget::Num(op_target.parse().unwrap()),
            )
        };
        return Some(Operation(operator));
    }

    if line.chars().nth(2).unwrap() == 'T' {
        return Some(Divisible(
            line.split("by ").nth(1).unwrap().parse().unwrap(),
        ));
    }

    if line.chars().nth(7).unwrap() == 't' {
        return Some(TrueMonkey(
            line.split("monkey ").nth(1).unwrap().parse().unwrap(),
        ));
    }

    if line.chars().nth(7).unwrap() == 'f' {
        return Some(FalseMonkey(
            line.split("monkey ").nth(1).unwrap().parse().unwrap(),
        ));
    }
    panic!()
}

type Item = usize;
type MonkeyNum = usize;

struct Monkey {
    items: Vec<Item>,
    op: Operator,
    divisible_test: usize,
    true_monkey: MonkeyNum,
    false_monkey: MonkeyNum,
    lcm: usize,
}

impl Monkey {
    fn new(
        items: Vec<Item>,
        op: Operator,
        divisible_test: usize,
        true_monkey: MonkeyNum,
        false_monkey: MonkeyNum,
    ) -> Self {
        Self {
            items,
            op,
            divisible_test,
            true_monkey,
            false_monkey,
            lcm: divisible_test,
        }
    }

    fn set_lcm(&mut self, lcm: usize) {
        self.lcm = lcm;
    }

    fn throw_items(&mut self, div_3: bool) -> Vec<(MonkeyNum, Item)> {
        let mut thrown = vec![];
        for item in &self.items {
            let mut item = self.op.call(*item);
            if div_3 {
                item = item / 3;
            }
            item = item % self.lcm;
            if item % self.divisible_test == 0 {
                thrown.push((self.true_monkey, item));
            } else {
                thrown.push((self.false_monkey, item));
            }
        }
        self.items.clear();
        thrown
    }
    fn push_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

fn build_monkeys(lines: &Vec<Line>) -> Vec<Monkey> {
    use Line::*;

    let mut iter = lines.iter();
    let mut monkeys: Vec<Monkey> = Vec::new();
    while let Some(MonkeyN(_)) = iter.next() {
        let items = if let Items(items) = iter.next().unwrap() {
            items
        } else {
            panic!()
        };
        let operator = if let Operation(op) = iter.next().unwrap() {
            op
        } else {
            panic!()
        };
        let divisible_test = if let Divisible(div) = iter.next().unwrap() {
            div
        } else {
            panic!()
        };
        let true_monkey = if let TrueMonkey(m) = iter.next().unwrap() {
            m
        } else {
            panic!()
        };
        let false_monkey = if let FalseMonkey(m) = iter.next().unwrap() {
            m
        } else {
            panic!()
        };
        monkeys.push(Monkey::new(
            items.clone(),
            operator.clone(),
            *divisible_test,
            *true_monkey,
            *false_monkey,
        ))
    }
    monkeys
}

fn solve(lines: &Vec<Line>, count: usize, div_3: bool) -> String {
    let mut monkeys = build_monkeys(lines);
    let mut lcm = 1;
    for monkey in &monkeys {
        lcm *= monkey.divisible_test;
    }
    for monkey in &mut monkeys {
        monkey.set_lcm(lcm);
    }
    let mut counts: HashMap<MonkeyNum, usize> = HashMap::new();
    for _ in 0..count {
        for monkey_num in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_num];
            let thrown_items = monkey.throw_items(div_3);

            *counts.entry(monkey_num).or_insert(0) += thrown_items.len();
            for (m_num, item) in thrown_items {
                monkeys[m_num].push_item(item)
            }
        }
    }
    let mut counts = counts.values().map(|x| *x).collect::<Vec<_>>();
    counts.sort();
    let mut most = counts.iter().rev().take(2);
    (most.next().unwrap() * most.next().unwrap()).to_string()
}

fn solve1(lines: &Vec<Line>) -> String {
    solve(lines, 20, true)
}

fn solve2(lines: &Vec<Line>) -> String {
    solve(lines, 10_000, false)
}

fn main() {
    let input_lines = read_input_file("11", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
