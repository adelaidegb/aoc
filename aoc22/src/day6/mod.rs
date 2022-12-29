use std::collections::HashSet;

struct CommSignal {
    message: Vec<char>,
}

impl CommSignal {
    fn find_start(&self, block_size: usize) -> Option<usize> {
        for i in block_size..self.message.len() {
            if self.message[i-block_size..i].iter().collect::<HashSet<_>>().len() >= block_size {
                return Some(i);
            }
        }

        return None;
    }
}

#[aoc(day=6, part=1)]
fn part1(input: String) -> String {
    CommSignal {
        message: input.chars().collect()
    }.find_start(4).unwrap().to_string()
}

#[aoc(day=6, part=2)]
fn part2(input: String) -> String {
    CommSignal {
        message: input.chars().collect()
    }.find_start(14).unwrap().to_string()
}
