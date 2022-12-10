use common::*;

fn line_parser(line: &str, _previous_lines: &Vec<Line>) -> Option<Line> {
    if line.eq("") {
        return None;
    }

    if line.eq("noop") {
        return Some(Instruction::Noop);
    }

    let mut split = line.split(" ");
    split.next();

    Some(Instruction::Addx(split.next().unwrap().parse().unwrap()))
}

type Line = Instruction;

#[derive(Clone, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn process(&self, cpu: &mut CPU) {
        match self {
            Self::Noop => {}
            Self::Addx(n) => cpu.x += n,
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

struct Program(Vec<Instruction>);

impl Program {
    fn new(lines: &Vec<Line>) -> Self {
        Self(lines.clone())
    }

    fn instruction(&self, n: usize) -> &Instruction {
        &self.0[n]
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}

struct CPU {
    x: i32,
}

impl CPU {
    fn new() -> Self {
        Self { x: 1 }
    }
}
enum ProgramStatus {
    Running,
    Completed,
}

struct ExecutingProgram {
    program: Program,
    pc: usize,
    cpu: CPU,
    cycles: usize,
    instruction_cycles_left: usize,
    status: ProgramStatus,
}

impl ExecutingProgram {
    fn new(program: Program) -> Self {
        let cycles = program.instruction(0).cycles();
        Self {
            program,
            pc: 0,
            cycles: 0,
            cpu: CPU::new(),
            instruction_cycles_left: cycles,
            status: ProgramStatus::Running,
        }
    }

    fn tick(&mut self) {
        if let ProgramStatus::Completed = self.status {
            return;
        }

        self.cycles += 1;
        self.instruction_cycles_left -= 1;
        if self.instruction_cycles_left == 0 {
            self.program.instruction(self.pc).process(&mut self.cpu);
            self.pc += 1;
            if self.pc < self.program.size() {
                self.instruction_cycles_left = self.program.instruction(self.pc).cycles();
            } else {
                self.status = ProgramStatus::Completed;
            }
        }
    }
}

fn solve1(lines: &Vec<Line>) -> String {
    let mut pg = ExecutingProgram::new(Program::new(lines));
    let mut strength: i32 = 0;
    while let ProgramStatus::Running = pg.status {
        if vec![19, 59, 99, 139, 179, 219].contains(&pg.cycles) {
            strength += (pg.cycles + 1) as i32 * pg.cpu.x;
        }
        pg.tick()
    }
    strength.to_string()
}
fn solve2(lines: &Vec<Line>) -> String {
    let mut pg = ExecutingProgram::new(Program::new(lines));
    let mut result = String::new();

    let mut current_line: [char; 40] = ['.'; 40];
    while let ProgramStatus::Running = pg.status {
        let cycle_pos = (pg.cycles % 40) as i32;
        let x_pos = pg.cpu.x - 1;

        if x_pos == cycle_pos || x_pos + 1 == cycle_pos || x_pos + 2 == cycle_pos {
            current_line[cycle_pos as usize] = '#';
        }
        pg.tick();

        if pg.cycles % 40 == 0 {
            for c in current_line {
                result.push(c);
            }
            result.push('\n');
            current_line = ['.'; 40];
        }
    }
    result
}

fn main() {
    let input_lines = read_input_file("10", line_parser);

    println!("{}", solve1(&input_lines));
    println!("{}", solve2(&input_lines));
}
