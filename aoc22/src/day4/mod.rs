use regex::Regex;

#[derive(Clone, Debug)]
struct SectionAssignment {
    min: i32,
    max: i32,
}

impl SectionAssignment {
    fn contains(&self, n: i32) -> bool {
        n >= self.min && n <= self.max
    }

    fn fully_contains(&self, other: &SectionAssignment) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }
}

#[derive(Debug)]
struct AssignedPair(SectionAssignment, SectionAssignment);

impl AssignedPair {
    fn has_full_overlap(&self) -> bool {
        match self {
            AssignedPair(assign1, assign2) => assign1.fully_contains(assign2) || assign2.fully_contains(assign1)
        }
    }

    fn has_partial_overlap(&self) -> bool {
        match self {
            AssignedPair(assign1, assign2) => assign1.contains(assign2.min) || assign1.contains(assign2.max) || assign2.contains(assign1.min) || assign2.contains(assign1.max)
        }
    }
}

fn parse_sections(input: &String) -> (Vec<SectionAssignment>, Vec<AssignedPair>) {
    let regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut assigns: Vec<SectionAssignment> = vec![];
    let mut pairs: Vec<AssignedPair> = vec![];

    for line in input.lines() {
        let cap = regex.captures(line).unwrap();
        let elf1 = SectionAssignment {
            min: cap[1].parse().unwrap(),
            max: cap[2].parse().unwrap()
        };
        let elf2 = SectionAssignment {
            min: cap[3].parse().unwrap(),
            max: cap[4].parse().unwrap()
        };

        pairs.push(AssignedPair(elf1.clone(), elf2.clone()));
        assigns.push(elf1);
        assigns.push(elf2);
    }

    (assigns, pairs)
}

fn find_overlaps(input: &String, pred: &dyn Fn(&AssignedPair) -> bool) -> String {
    let (_assigns, pairs) = parse_sections(input);
    let mut overlaps = 0u32;

    for pair in pairs {
        if pred(&pair) {
            println!("{:?} has overlap", pair);
            overlaps += 1;
        }
    }

    overlaps.to_string()
}

#[aoc(day4, part1)]
fn part1(input: String) -> String {
    find_overlaps(&input, &AssignedPair::has_full_overlap)
}

#[aoc(day4, part2)]
fn part2(input: String) -> String {
    find_overlaps(&input, &AssignedPair::has_partial_overlap)
}
