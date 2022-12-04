pub struct AocEntry {
    pub day: u32,
    pub part: u32,
    pub executor: fn(&str) -> String,
}

impl AocEntry {
    pub const fn new(day: u32, part: u32, executor: fn(&str) -> String) -> Self {
        Self { day, part, executor }
    }

    pub fn execute(&self, input: &str) -> String {
        (self.executor)(input)
    }
}

inventory::collect!(AocEntry);

pub fn aoclib_main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <day> <part> <input file>", args[0]);
        return;
    }

    let day = args[1].parse::<u32>().unwrap();
    let part = args[2].parse::<u32>().unwrap();
    let input = std::fs::read_to_string(args[3].clone()).unwrap();

    let mut output: Option<String> = None;
    for entry in inventory::iter::<AocEntry> {
        if entry.day == day && entry.part == part {
            output = Some(entry.execute(&input));
            break;
        }
    }

    if let Some(output) = output {
        println!("==================== Output ====================");
        println!("{output}");
    } else {
        println!("No entry found for day {day} part {part}");
    }
}

#[macro_export]
macro_rules! add_entry {
    ($day:expr, $part:expr, $executor:expr) => {
        ::inventory::submit!(::aoclib::AocEntry::new($day, $part, $executor));
    };
}
