use std::collections::HashMap;

#[derive(Debug)]
enum Opcodes {
    Addx(i32),
    Noop
}

impl Opcodes {
    fn get_cycle_req(&self) -> u8 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1
        }
    }

    fn run(&self, state: &mut ProgramState) {
        match self {
            Self::Addx(dx) => state.x += dx,
            _ => return
        };
    }

    fn parse(s: &str) -> Option<Opcodes> {
        let mut parts = s.split(" ");

        match parts.next().unwrap() {
            "addx" => Some(Self::Addx(parts.next().unwrap().parse().unwrap())),
            "noop" => Some(Self::Noop),
            _ => None
        }
    }
}

struct ProgramState {
    x: i32
}

impl ProgramState {
    fn test_run(&mut self, insn: &InsnList) -> HashMap<i32, i32> {
        let mut cycle: i32 = 0;
        let mut signals: HashMap<i32, i32> = HashMap::new();

        for op in insn {
            let req = op.get_cycle_req();

            for i in 1..=req {
                cycle += 1;

                // on cycles matching 20+40k (20, 60, etc), record the signal strength
                if (cycle - 20) % 40 == 0 {
                    let strength = cycle * self.x;
                    println!("({op:?}, cycle {i} of {req}) cycle {cycle} * {} = {strength}", self.x);
                    signals.insert(cycle, strength);
                }

                if i == req {
                    op.run(self);
                }
            }
        }

        signals
    }

    fn print_run(&mut self, insn: &InsnList) -> String {
        let mut cycle: i32 = 0;
        let mut output = String::new();

        for op in insn {
            let req = op.get_cycle_req();

            for i in 1..=req {
                cycle += 1;

                let pixel = cycle - 1;

                output.push(if (self.x - pixel % 40).abs() <= 1 {
                    '#'
                } else {
                    '.'
                });

                if cycle % 40 == 0 {
                    output.push_str("\n");
                }

                if i == req {
                    op.run(self);
                }
            }
        }

        output
    }
}

type InsnList = Vec<Opcodes>;

fn parse_insn(input: &String) -> InsnList {
    let mut insn: InsnList = Vec::new();

    for line in input.lines() {
        match Opcodes::parse(&line) {
            Some(opcode) => insn.push(opcode),
            None => panic!("unsupported opcode: {line}"),
        }
    }

    insn
}

#[aoc(day=10, part=1)]
fn part1(input: String) -> String {
    let insn = parse_insn(&input);
    let mut state = ProgramState { x: 1 };

    state.test_run(&insn).values().sum::<i32>().to_string()
}

#[aoc(day=10, part=2)]
fn part2(input: String) -> String {
    let insn = parse_insn(&input);
    let mut state = ProgramState { x: 1 };

    state.print_run(&insn)
}

