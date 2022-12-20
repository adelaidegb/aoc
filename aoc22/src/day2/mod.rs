#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    fn decode(token: &str) -> Option<Shape> {
        match token {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None
        }
    }

    fn force_result(&self, result: GameResult) -> Shape {
        if result == GameResult::Draw {
            return *self;
        }

        let beaten = result == GameResult::Win;

        match self {
            Self::Rock => match beaten {
                true => Self::Paper,
                false => Self::Scissors
            },
            Self::Paper => match beaten {
                true => Self::Scissors,
                false => Self::Rock
            },
            Self::Scissors => match beaten {
                true => Self::Rock,
                false => Self::Paper
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum GameResult {
    Loss,
    Draw,
    Win
}

impl GameResult {
    fn value(&self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6
        }
    }

    fn decode(token: &str) -> Option<GameResult> {
        match token {
            "X" => Some(Self::Loss),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None
        }
    }
}

struct Game {
    opponent: Shape,
    player: Shape
}

impl Game {
    fn to_result(&self) -> GameRecord {
        if self.player == self.opponent {
            return GameRecord(GameResult::Draw, self.player);
        }

        // defines the Shape that the player would win against
        let win_vs = match self.player {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        };

        match self.opponent == win_vs {
            true => GameRecord(GameResult::Win, self.player),
            false => GameRecord(GameResult::Loss, self.player)
        }
    }
}

#[derive(Debug)]
struct GameRecord(GameResult, Shape);

impl GameRecord {
    fn score(&self) -> i32 {
        match self {
            GameRecord(result, shape) => result.value() + shape.value()
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: String) -> String {
    let lines = input.split("\n");
    let mut running_total = 0i32;

    for line in lines {
        let (opponent, player) = match line.split(' ').take(2).collect::<Vec<&str>>()[..] {
            [first, second, ..] => (Shape::decode(first).unwrap(), Shape::decode(second).unwrap()),
            _ => panic!("invalid game definition given")
        };

        let result = Game { opponent, player }.to_result();
        println!("game (opponent {:?}, player {:?}) -> {:?} (score: {})", opponent, player, result, result.score());

        running_total += result.score();
    }

    println!("total after all games is {}", running_total);

    running_total.to_string()
}

#[aoc(day2, part2)]
pub fn part2(input: String) -> String {
    let lines = input.split("\n");
    let mut running_total = 0i32;

    for line in lines {
        let (opponent, player) = match line.split(' ').take(2).collect::<Vec<&str>>()[..] {
            [first, second, ..] => {
                let opponent = Shape::decode(first).unwrap();
                let outcome = GameResult::decode(second).unwrap();

                (opponent, opponent.force_result(outcome))
            },
            _ => panic!("invalid game definition given")
        };

        let result = Game { opponent, player }.to_result();
        println!("game (opponent {:?}, player {:?}) -> {:?} (score: {})", opponent, player, result, result.score());

        running_total += result.score();
    }

    println!("total after all games is {}", running_total);

    running_total.to_string()
}
