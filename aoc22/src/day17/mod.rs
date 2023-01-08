use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::iter::from_fn;

type CoordinateType = i64;
type Coord = [CoordinateType; 2];
type RockDef = &'static [&'static [bool]];

const ROCK_DEFINITIONS: [RockDef; 5] = [
    &[
        &[true, true, true, true]
    ],
    &[
        &[false, true, false],
        &[ true, true,  true],
        &[false, true, false]
    ],
    &[
        &[false, false, true],
        &[false, false, true],
        &[ true,  true, true],
    ],
    &[
        &[true],
        &[true],
        &[true],
        &[true],
    ],
    &[
        &[true, true],
        &[true, true],
    ],
];

struct Rock {
    def: RockDef,
    pos: Coord,
}

impl Rock {
    const NO_DELTA: Coord = [0, 0];

    fn fits(&self, chamber: &Chamber) -> bool {
        self.fits_with_delta(chamber, &Self::NO_DELTA)
    }

    fn fits_with_delta(&self, chamber: &Chamber, delta: &Coord) -> bool {
        for (y, row) in self.def.iter().enumerate() {
            for (x, cell_state) in row.iter().enumerate() {
                if !cell_state {
                    continue;
                }

                if chamber.is_blocked(&[self.pos[0] + delta[0] + x as CoordinateType, self.pos[1] + delta[1] - y as CoordinateType]) {
                    return false;
                }
            }
        }

        true
    }

    fn move_if_fits(&mut self, chamber: &Chamber, delta: &Coord) -> bool {
        if !self.fits_with_delta(chamber, delta) {
            return false;
        }

        self.pos[0] += delta[0];
        self.pos[1] += delta[1];

        true
    }
}

#[derive(Copy, Clone, Debug)]
enum Movement {
    Left,
    Right,
}

impl Movement {
    const DOWN: Coord = [0, -1];

    fn get_delta(&self) -> Coord {
        match self {
            Self::Left => [-1, 0],
            Self::Right => [1, 0],
        }
    }
}

struct Chamber {
    grid: HashSet<Coord>,
    max_y: CoordinateType,
}

impl Chamber {
    const WIDTH: CoordinateType = 7;
    const SPAWN_X: CoordinateType = 2;
    const SPAWN_DY: CoordinateType = 3;

    #[inline]
    fn is_blocked(&self, at: &Coord) -> bool {
        at[0] < 0 || at[1] < 0 || at[0] >= Self::WIDTH || self.grid.contains(at)
    }

    #[inline]
    fn spawn_rock(&self, def: RockDef) -> Rock {
        Rock {
            def,
            pos: [Self::SPAWN_X, self.max_y + Self::SPAWN_DY + def.len() as CoordinateType],
        }
    }

    fn insert(&mut self, rock: &Rock) {
        for (y, row) in rock.def.iter().enumerate() {
            for (x, cell_state) in row.iter().enumerate() {
                if !cell_state {
                    continue;
                }

                self.grid.insert([rock.pos[0] + x as CoordinateType, rock.pos[1] - y as CoordinateType]);
            }
        }

        self.max_y = max(self.max_y,rock.pos[1]);
    }

    fn simulate_rocks(&mut self, count: u64, rocks: &mut impl Iterator<Item = (usize, RockDef)>, jets: &mut impl Iterator<Item = (usize, Movement)>) {
        for _ in 0..count {
            let mut rock = self.spawn_rock(rocks.next().unwrap().1);

            assert!(rock.fits(self));

            loop {
                rock.move_if_fits(self, &jets.next().unwrap().1.get_delta());

                if !rock.move_if_fits(self, &Movement::DOWN) {
                    break;
                }
            }

            self.insert(&rock);
        }
    }

    fn find_pattern_and_guess(&mut self, count: u64, rocks: &mut impl Iterator<Item = (usize, RockDef)>, jets: &mut impl Iterator<Item = (usize, Movement)>) -> CoordinateType {
        let mut starting_combinations: HashMap<(usize, usize), (u64, CoordinateType)> = HashMap::new();
        let mut last_pattern: Option<(u64, CoordinateType)> = None;
        let mut consecutive_pattern = 0;

        for li in 0..count {
            let (ri, rock_def) = rocks.next().unwrap();
            let mut check_for_pattern = true;
            let mut rock = self.spawn_rock(rock_def);

            assert!(rock.fits(self));

            loop {
                let (ji, movement) = jets.next().unwrap();

                if check_for_pattern {
                    let combination = (ri, ji);

                    if let Some((former_li, former_height)) = starting_combinations.get(&combination) {
                        let current_height = self.max_y + 1;
                        let li_delta = li - former_li;
                        let height_delta = current_height - former_height;

                        match last_pattern {
                            Some((li_pattern, height_pattern)) if li_pattern == li_delta && height_pattern == height_delta => {
                                consecutive_pattern += 1;

                                if consecutive_pattern == li_pattern {
                                    println!("Confirmed pattern +{li_pattern} indices, +{height_pattern} height at index {li}");
                                    println!("Estimated return at index {}, if pattern continues to hold",
                                             li / li_pattern * li_pattern + count % li_pattern);
                                }

                                if consecutive_pattern >= li_pattern && (count - li) % li_pattern == 0 {
                                    let guess = current_height + (count - li) as CoordinateType / li_pattern as CoordinateType * height_pattern;

                                    println!("Pattern implies height will be {guess}; returning early");

                                    return guess;
                                }
                            },
                            _ => {
                                consecutive_pattern = 0;
                                last_pattern = Some((li_delta, height_delta));
                            }
                        }
                    }

                    starting_combinations.insert(combination, (li, self.max_y + 1));
                    check_for_pattern = false;
                }

                rock.move_if_fits(self, &movement.get_delta());

                if !rock.move_if_fits(self, &Movement::DOWN) {
                    break;
                }
            }

            self.insert(&rock);
        }

        self.max_y + 1
    }

    fn new() -> Chamber {
        Chamber {
            grid: HashSet::new(),
            max_y: -1
        }
    }
}

fn rock_def_iter() -> impl Iterator<Item = (usize, RockDef)> {
    let mut i = 0;

    from_fn(move || {
        let next = Some((i, ROCK_DEFINITIONS[i]));

        i = (i + 1) % ROCK_DEFINITIONS.len();

        next
    })
}

fn jet_movement_iter(input: String) -> impl Iterator<Item = (usize, Movement)> {
    let jets: Vec<Movement> = input.chars().map(|c| match c {
        '<' => Movement::Left,
        '>' => Movement::Right,
        _ => panic!("unsupported jet movement character: {c}")
    }).collect();
    let mut i = 0;

    from_fn(move || {
        let next = Some((i, jets[i]));

        i = (i + 1) % jets.len();

        next
    })
}

#[aoc(day=17, part=1)]
fn part1(input: String) -> String {
    const GOAL: u64 = 2022;

    let mut chamber = Chamber::new();

    // chamber.find_pattern_and_guess from part 2 would also be sufficient for this
    chamber.simulate_rocks(GOAL, &mut rock_def_iter(), &mut jet_movement_iter(input));

    (chamber.max_y + 1).to_string()
}

#[aoc(day=17, part=2)]
fn part2(input: String) -> String {
    const GOAL: u64 = 1_000_000_000_000;

    Chamber::new()
        .find_pattern_and_guess(GOAL, &mut rock_def_iter(), &mut jet_movement_iter(input))
        .to_string()
}
