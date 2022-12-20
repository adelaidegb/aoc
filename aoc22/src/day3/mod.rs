use std::collections::HashSet;

#[derive(Debug)]
struct Sack {
    first: HashSet<char>,
    second: HashSet<char>
}

impl Sack {
    fn from(contents: &str) -> Sack {
        let mid = contents.len() / 2;

        Sack {
            first: contents[..mid].chars().collect(),
            second: contents[mid..].chars().collect()
        }
    }
}

#[derive(Debug)]
struct Group<'a> {
    sacks: [&'a Sack; 3]
}

impl Group<'_> {
    fn from(members: &[Sack]) -> Group {
        match members {
            [first, second, third, ..] => Group {
                sacks: [first, second, third]
            },
            _ => panic!("invalid group definition given {:?}", members)
        }
    }

    fn find_badge(&self) -> char {
        let candidates: Vec<char> = match self.sacks {
            [sack1, sack2, sack3] => {
                let mut common: HashSet<_> = sack1.first.union(&sack1.second).map(|c| *c).collect();

                common = common.intersection(&sack2.first.union(&sack2.second).map(|c| *c).collect()).map(|c| *c).collect();

                common.intersection(&sack3.first.union(&sack3.second).map(|c| *c).collect()).map(|c| *c).collect()
            }
        };

        if candidates.len() != 1 {
            panic!("invalid group contents: {:?}", self);
        }

        *candidates.iter().next().unwrap_or_else(|| unreachable!())
    }
}

fn value(c: &char) -> i32 {
    match c {
        'a'..='z' => (*c as i32) - ('a' as i32) + 1,
        'A'..='Z' => (*c as i32) - ('A' as i32) + 27,
        _ => panic!("invalid item char: {}", c)
    }
}

fn parse_sacks(input: &String) -> Vec<Sack> {
    input.split("\n").map(Sack::from).collect()
}

#[aoc(day3, part1)]
fn part1(input: String) -> String {
    let sacks = parse_sacks(&input);
    let mut total = 0i32;

    for sack in sacks {
        let inter: HashSet<_> = sack.first.intersection(&sack.second).map(|c| *c).collect();

        if inter.len() != 1 {
            panic!("invalid Sack: more than one item is found in both halves: {}", inter.iter().collect::<String>());
        }

        let val = value(inter.iter().next().unwrap());
        println!("sack intersection is {}, value of {}", inter.iter().collect::<String>(), val);
        total += val;
    }

    total.to_string()
}

#[aoc(day3, part2)]
fn part2(input: String) -> String {
    let sacks = parse_sacks(&input);
    let mut total = 0i32;

    for i in (0..sacks.len()).step_by(3) {
        let group = Group::from(&sacks[i..i+3]);
        let badge = group.find_badge();
        let val = value(&badge);

        println!("Group [{}..{}] has badge {} (value {})", i, i + 3, badge, val);

        total += val;
    }

    total.to_string()
}
