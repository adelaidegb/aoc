use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;

type MonkeyId = i32;
type ItemWorryLevel = i64;
type InspectionCount = u64;

struct Monkey {
    id: MonkeyId,
    items: VecDeque<ItemWorryLevel>,
    operation: Box<dyn Fn(ItemWorryLevel) -> ItemWorryLevel>,
    next_monkey: Box<dyn Fn(ItemWorryLevel) -> MonkeyId>,
    inspected: InspectionCount,
}

fn parse_monkeys(input: &String) -> (Vec<Monkey>, ItemWorryLevel) {
    lazy_static! {
        static ref SPLIT_REGEX: Regex = Regex::new(r"\r?\n\r?\n").unwrap();
        static ref MONKEY_REGEX: Regex = Regex::new(r"Monkey (\d+):\r?\n  Starting items: ([\w, ]+)\r?\n  Operation: new = (\w+) ([+\-*/]) (\w+)\r?\n  Test: divisible by (\d+)\r?\n    If true: throw to monkey (\d+)\r?\n    If false: throw to monkey (\d+)").unwrap();
    }

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut cm: ItemWorryLevel = 1;
    let split = SPLIT_REGEX.split(input);

    for mstr in split {
        let m = MONKEY_REGEX.captures(mstr).unwrap_or_else(|| panic!("couldn't parse regex"));
        let arg1: Option<ItemWorryLevel> = m[3].parse().ok();
        let op = m[4].chars().next().unwrap();
        let arg2: Option<ItemWorryLevel> = m[5].parse().ok();
        let modulus: ItemWorryLevel = m[6].parse().unwrap();
        let when_true: MonkeyId = m[7].parse().unwrap();
        let when_false: MonkeyId = m[8].parse().unwrap();

        cm *= &modulus;

        monkeys.push(Monkey {
            id: m[1].parse().unwrap(),
            items: m[2].split(", ").map(|item| item.parse::<ItemWorryLevel>().unwrap()).collect(),
            operation: Box::new(move |value| -> ItemWorryLevel {
                let l = arg1.clone().unwrap_or(value.clone());
                let r = arg2.clone().unwrap_or(value.clone());

                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    _ => panic!("unsupported op {op}")
                }
            }),
            next_monkey: Box::new(move |value| if value % &modulus == 0 { when_true } else { when_false }),
            inspected: 0
        });
    }

    println!("Of all modules, common multiple is {cm}");

    (monkeys, cm)
}

fn conduct_monkey_business(monkeys: &mut Vec<Monkey>, max_rounds: i32, relief_divisor: ItemWorryLevel, lcm: ItemWorryLevel) {
    let mut index: HashMap<MonkeyId, &mut Monkey> = HashMap::new();

    for monkey in monkeys.iter_mut() {
        index.insert(monkey.id, monkey);
    }

    let mut current_id = *index.keys().min().unwrap();
    let mut round = 0;

    while round < max_rounds {
        let monkey = index.get_mut(&current_id).unwrap();

        if let Some(item) = monkey.items.pop_front() {
            // this monkey still has items to inspect
            let resulting_worry = (*monkey.operation)(item) / &relief_divisor % &lcm;
            let targeted_monkey = (*monkey.next_monkey)(resulting_worry.clone());
            monkey.inspected += 1;

            index.get_mut(&targeted_monkey).unwrap().items.push_back(resulting_worry.clone());
            // println!("An item with worry {} has been inspected and passed to monkey id {}", resulting_worry, targeted_monkey);
        } else {
            // this monkey has run out of items, proceed to the next one
            current_id += 1;

            if !index.contains_key(&current_id) {
                // as monkey IDs are guaranteed to be contiguous, we've concluded a round when we reach this
                current_id = *index.keys().min().unwrap();
                round += 1;
                // println!("Proceeding to round {}", round + 1);
            }
        }
    }
}

fn report_monkey_business(monkeys: &Vec<Monkey>) -> InspectionCount {
    let mut sorted: Vec<(InspectionCount, &Monkey)> = Vec::new();

    for monkey in monkeys.iter() {
        println!("Monkey {} has inspected items {} times", monkey.id, monkey.inspected);
        sorted.push((monkey.inspected, monkey));
    }

    sorted.sort_by(|l, r| r.0.cmp(&l.0));

    sorted.get(0).unwrap().0 * sorted.get(1).unwrap().0
}

#[aoc(day=11, part=1)]
fn part1(input: String) -> String {
    let (mut monkeys, lcm) = parse_monkeys(&input);

    conduct_monkey_business(&mut monkeys, 20, 3, lcm);

    report_monkey_business(&monkeys).to_string()
}

#[aoc(day=11, part=2)]
fn part2(input: String) -> String {
    let (mut monkeys, lcm) = parse_monkeys(&input);

    conduct_monkey_business(&mut monkeys, 10_000, 1, lcm);

    report_monkey_business(&monkeys).to_string()
}
