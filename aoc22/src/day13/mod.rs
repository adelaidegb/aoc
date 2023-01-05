use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketData {
    List(Vec<PacketData>),
    Item(u32),
}

impl PacketData {
    fn compare(&self, other: &PacketData) -> Ordering {
        match (self, other) {
            (Self::List(l), Self::List(r)) => Self::compare_lists(l, r),
            (Self::Item(l), Self::Item(r)) => l.cmp(r),
            (Self::List(_), Self::Item(_)) => self.compare(&Self::List(vec![other.clone()])),
            (Self::Item(_), Self::List(_)) => Self::List(vec![self.clone()]).compare(other),
        }
    }

    fn compare_lists(l: &Vec<PacketData>, r: &Vec<PacketData>) -> Ordering {
        let len = l.len().min(r.len());

        for i in 0..len {
            match l[i].compare(&r[i]) {
                Ordering::Equal => continue,
                other => return other
            }
        }

        l.len().cmp(&r.len())
    }
}

impl PacketData {
    /**
     * Parse a list at the current position of the given iterator.
     *
     * Expects the first character to be a opening bracket, and will consume
     * from the iterator until after it pops the associated closing bracket.
     */
    fn parse_list(it: &mut Peekable<Chars>) -> PacketData {
        if it.next().unwrap() != '[' {
            panic!("parse_list expects an iterator positioned at the start of a list definition");
        }

        let mut vec: Vec<PacketData> = Vec::new();

        // peek into iterator, and then:
        // if the next value is '[', recursively pass to parse_list; it will return after the associated ']' is popped; add the return value to vec and continue looping
        // if the next value is '0'..='9', pass to parse_item; it will return before the next ']' or ','; add the return value to vec and continue looping
        // if the next value is ',', pop it and continue looping
        // if the next value is ']', pop it and return PacketData::List(vec)
        loop {
            let add = match it.peek() {
                Some('[') => Self::parse_list(it),
                Some('0'..='9') => Self::parse_item(it),
                Some(',') => {
                    it.next().unwrap();
                    continue;
                },
                Some(']') => {
                    it.next().unwrap();
                    return PacketData::List(vec);
                },
                Some(un) => panic!("unsupported character: {un}"),
                None => panic!("unexpected end of iterator while parsing list")
            };

            vec.push(add);
        }
    }

    /**
     * Parse an item at the current position of the given iterator.
     *
     * Expects the leading characters of the iterator to be digits.
     *
     * This processing will consume all leading digits from the iterator.
     */
    fn parse_item(it: &mut Peekable<Chars>) -> PacketData {
        let mut item = String::new();

        while let Some(digit) = it.next_if(|c| ('0'..='9').contains(c)) {
            item.push(digit);
        }

        PacketData::Item(item.parse().unwrap())
    }
}

fn parse_pairs(input: &String) -> Vec<(PacketData, PacketData)> {
    let mut tuples: Vec<(PacketData, PacketData)> = Vec::new();
    let mut lines = input.lines();

    loop {
        match (lines.next(), lines.next()) {
            (Some(list1), Some(list2)) => {
                tuples.push((PacketData::parse_list(&mut list1.chars().peekable()), PacketData::parse_list(&mut list2.chars().peekable())));
            },
            _ => panic!("found unbalanced pairs")
        }

        if let None = lines.next() {
            return tuples;
        }
    }
}

fn parse_flat(input: &String) -> Vec<PacketData> {
    let mut data: Vec<PacketData> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        data.push(PacketData::parse_list(&mut line.chars().peekable()));
    }

    data
}

#[aoc(day=13, part=1)]
fn part1(input: String) -> String {
    let pairs = parse_pairs(&input);
    let mut right_sum = 0;

    for (i, (list1, list2)) in pairs.iter().enumerate() {
        // println!("list 1:\t{:?}\nlist 2:\t{:?}\n\ncmp:\t{:?} (pair {})\n", list1, list2, list1.cmp(list2), i + 1);
        if list1.compare(list2) == Ordering::Less {
            right_sum += i + 1;
            println!("Pair {}: RIGHT", i + 1);
        } else {
            println!("Pair {}:       WRONG", i + 1);
        }
    }

    right_sum.to_string()
}

#[aoc(day=13, part=2)]
fn part2(input: String) -> String {
    let mut vec = parse_flat(&input);
    let divider_one = PacketData::List(vec![PacketData::List(vec![PacketData::Item(2)])]);
    let divider_two = PacketData::List(vec![PacketData::List(vec![PacketData::Item(6)])]);

    vec.push(divider_one.clone());
    vec.push(divider_two.clone());
    vec.sort_by(|l, r| l.compare(r));

    // puzzle output uses one-based ordinals
    let pos_one = vec.iter().position(|data| data == &divider_one).unwrap() + 1;
    let pos_two = vec.iter().position(|data| data == &divider_two).unwrap() + 1;

    (pos_one * pos_two).to_string()
}
