#[derive(Debug)]
struct Food {
    calories: u64,
}

#[derive(Debug)]
struct Elf {
    food: Vec<Food>,
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            food: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.food.is_empty()
    }

    pub fn total_calories(&self) -> u64 {
        let mut cal = 0u64;

        for f in &self.food {
            cal += f.calories;
        }

        return cal;
    }

    pub fn add(&mut self, calories: u64) {
        self.food.push(Food { calories });
    }
}

fn read_elves(elves: &mut Vec<Elf>, input: &String) {
    elves.push(Elf::new());

    let mut head = elves.last_mut().unwrap();
    let lines = input.split("\n");

    for line in lines {
        if !line.is_empty() {
            head.add(line.parse().unwrap());
            continue;
        }

        if !head.is_empty() {
            elves.push(Elf::new());
            head = elves.last_mut().unwrap();
        }
    }

    // pedantic: avoid trailing empty-handed Elf
    if head.is_empty() {
        elves.pop();
    }
}

#[aoc(day1, part1)]
pub fn part1(input: String) -> String {
    let mut elves: Vec<Elf> = vec![];

    read_elves(&mut elves, &input);

    elves.sort_by_key(|elf| elf.total_calories());

    let cal = elves.last().map_or(0, |elf| elf.total_calories());
    println!("top elf carries {:?} calories total", cal);

    cal.to_string()
}

#[aoc(day1, part2)]
pub fn part2(input: String) -> String {
    let mut elves: Vec<Elf> = vec![];

    read_elves(&mut elves, &input);

    elves.sort_by_key(|elf| elf.total_calories());

    let top_3_cal = elves.iter().rev().take(3).map(|elf| elf.total_calories()).reduce(|l, r| l + r).unwrap_or(0);
    println!("top 3 elves carry {:?} calories total", top_3_cal);

    top_3_cal.to_string()
}
