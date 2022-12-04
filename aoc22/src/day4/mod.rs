use std::ops::RangeInclusive;

#[cfg(test)] mod test;

#[aoc(day4, part1)]
pub fn find_entirely_overlapping(input: String) -> String {
    let lines = input.lines();
    let pairs = lines.map(|line| {
        let (first, second) = parse_pairs(line);

        if entirely_overlaps(&first, &second) || entirely_overlaps(&second, &first) {
            1
        } else {
            0
        }
    }).sum::<usize>();

    pairs.to_string()
}

#[aoc(day4, part2)]
pub fn find_partially_overlapping(input: String) -> String {
    let lines = input.lines();
    let pairs = lines.map(|line| {
        let (first, second) = parse_pairs(line);

        if partially_overlaps(&first, &second) || partially_overlaps(&second, &first) {
            1
        } else {
            0
        }
    }).sum::<usize>();

    pairs.to_string()
}

fn entirely_overlaps(range1: &SectionAssignment, range2: &SectionAssignment) -> bool {
    range1.start() <= range2.start() && range1.end() >= range2.end()
}

fn partially_overlaps(range1: &SectionAssignment, range2: &SectionAssignment) -> bool {
    range1.start() <= range2.end() && range1.end() >= range2.start()
}

fn parse_pairs(line: &str) -> (SectionAssignment, SectionAssignment) {
    let mut pairs = line.split(",");
    let first = parse_pair(pairs.next().unwrap());
    let second = parse_pair(pairs.next().unwrap());
    (first, second)
}

fn parse_pair(pair: &str) -> SectionAssignment {
    let mut parts = pair.split("-");
    let first = parts.next().unwrap().parse::<u8>().unwrap();
    let last = parts.next().unwrap().parse::<u8>().unwrap();

    SectionAssignment::from(RangeInclusive::new(first, last))
}

type SectionAssignment = RangeInclusive<u8>;
