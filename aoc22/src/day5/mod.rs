use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

struct SupplyStacks {
    stacks: HashMap<i8, Box<Vec<char>>>,
    /** (move qty, move from key, move to key) */
    insn: Vec<(i8, i8, i8)>,
}

impl SupplyStacks {
    fn process(&mut self) {
        for (ref repeat, ref from, ref to) in &self.insn {
            for _ in 0..*repeat {
                let c = self.stacks.get_mut(from).unwrap().pop().unwrap().clone();
                self.stacks.get_mut(to).unwrap().push(c);
            }
        }
    }

    fn process_9001(&mut self) {
        for (ref count, ref from, ref to) in &self.insn {
            let src = self.stacks.get(from).unwrap();
            let starting_at = (src.len() as i8 - count) as usize;
            let moving: Vec<char> = (&src[starting_at..]).iter().map(|c| *c).collect();

            let dest = self.stacks.get_mut(to).unwrap();

            for char in moving {
                dest.push(char.clone());
            }

            self.stacks.get_mut(from).unwrap().truncate(starting_at);
        }
    }

    fn pretty_print(&self) {
        let mut sorted_keys: Vec<&i8> = self.stacks.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            print!("{key}:");

            for c in self.stacks.get(key).unwrap().iter() {
                print!(" {c}");
            }

            println!();
        }
    }

    fn peek_string(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        let mut sorted_keys: Vec<&i8> = self.stacks.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            chars.push(self.stacks.get(key).unwrap().last().unwrap().clone());
        }

        chars.iter().collect()
    }
}

fn parse_input(input: &String) -> SupplyStacks {
    lazy_static! {
        static ref CRATE_LABEL_REGEX: Regex = Regex::new(r"(\d)").unwrap();
        static ref INSN_LIST_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let mut crates: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        crates.push(line);
    }

    let mut result = SupplyStacks {
        stacks: HashMap::new(),
        insn: Vec::new()
    };
    let labels = crates.pop().unwrap();
    crates.reverse();

    println!("labels are {labels}");

    for line in &crates {
        println!("crates are {line}");
    }

    println!();

    let mut idx = 0;

    loop {
        if !CRATE_LABEL_REGEX.is_match_at(labels, idx) {
            break;
        }

        let m = CRATE_LABEL_REGEX.find_at(labels, idx).unwrap();
        let mut stack: Vec<char> = Vec::new();

        println!("looking for stack {} at index {}", m.as_str(), m.start());

        // crates has been reversed; proceed in natural order to correctly assemble the stack
        for line in crates.iter() {
            if m.start() >= line.len() {
                break;
            }

            let c = line.chars().nth(m.start()).unwrap();

            if !('A'..='Z').contains(&c) {
                break;
            }

            stack.push(c);
        }

        result.stacks.insert(m.as_str().parse().unwrap(), Box::new(stack));
        idx = m.end();
    }

    println!();

    for line in input.lines().skip(crates.len() + 2) {
        let cap = INSN_LIST_REGEX.captures(line).unwrap();
        result.insn.push((cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()));
    }

    result
}

#[aoc(day = 5, part = 1)]
fn part1(input: String) -> String {
    let mut stacks = parse_input(&input);

    stacks.process();
    stacks.pretty_print();

    stacks.peek_string()
}

#[aoc(day = 5, part = 2)]
fn part2(input: String) -> String {
    let mut stacks = parse_input(&input);

    stacks.process_9001();
    stacks.pretty_print();

    stacks.peek_string()
}
