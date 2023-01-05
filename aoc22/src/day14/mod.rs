use std::cmp::{max, min};
use std::collections::HashMap;

type Coord = [i32; 2];

const SAND_ENTRY: Coord = [500, 0];
const DOWN_DELTAS: [Coord; 3] = [[0, 1], [-1, 1], [1, 1]];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Rock,
    Sand,
    Air,
}

impl Tile {
    fn is_solid(&self) -> bool {
        self != &Self::Air
    }
}

struct Cave {
    grid: HashMap<Coord, Tile>,
    floor: Option<(i32, Tile)>
}

impl Cave {
    fn get_tile(&self, at: &Coord) -> Tile {
        match self.grid.get(at) {
            Some(tile) => *tile,
            None => match self.floor {
                Some((y, tile)) if y == at[1] => tile,
                _ => Tile::Air,
            },
        }
    }

    fn drop_from(&self, mut point: Coord) -> Option<Coord> {
        if self.get_tile(&point).is_solid() {
            return None;
        }

        let deepest = self.max_y();

        'outer: loop {
            if point[1] >= deepest {
                // we'll fall forever
                return None;
            }

            for mut check in DOWN_DELTAS {
                check[0] += point[0];
                check[1] += point[1];

                if !self.get_tile(&check).is_solid() {
                    // move down and continue looping
                    point = check;
                    continue 'outer;
                }
            }

            // the dropped tile came to a rest at this point
            return Some(point);
        }
    }

    fn max_y(&self) -> i32 {
        self.grid.keys().map(|coord| coord[1]).max().unwrap_or(0).max(self.floor.map(|floor| floor.0).unwrap_or(0))
    }

    fn fill(&mut self, from: &Coord, to: &Coord, with: Tile) {
        let [x_min, y_min, x_max, y_max] = match (*from, *to) {
            ([x1, y1], [x2, y2]) => [min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)]
        };

        for i in x_min..=x_max {
            for j in y_min..=y_max {
                self.grid.insert([i, j], with);
            }
        }
    }

    fn drop_and_set(&mut self, start: Coord, tile: Tile) -> bool {
        match self.drop_from(start) {
            Some(stop) => {
                self.grid.insert(stop, tile);
                true
            },
            None => false
        }
    }
}

fn input_fill_cave(input: &String, cave: &mut Cave) {
    for line in input.lines() {
        let mut coord_chain = line.split(" -> ").map(|coord| -> Coord {
            let mut split = coord.splitn(2, ",");

            [split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap()]
        });
        let mut prev = coord_chain.next().unwrap();

        for next in coord_chain {
            cave.fill(&prev, &next, Tile::Rock);

            prev = next;
        }
    }
}

#[aoc(day=14, part=1)]
fn part1(input: String) -> String {
    let mut cave = Cave {
        grid: HashMap::new(),
        floor: None
    };

    input_fill_cave(&input, &mut cave);

    let mut sand_dropped = 0;

    while cave.drop_and_set(SAND_ENTRY, Tile::Sand) {
        sand_dropped += 1;
    }

    sand_dropped.to_string()
}

#[aoc(day=14, part=2)]
fn part2(input: String) -> String {
    let mut cave = Cave {
        grid: HashMap::new(),
        floor: None
    };

    input_fill_cave(&input, &mut cave);
    cave.floor = Some((cave.max_y() + 2, Tile::Rock));

    let mut sand_dropped = 0;

    while cave.drop_and_set(SAND_ENTRY, Tile::Sand) {
        sand_dropped += 1;
    }

    sand_dropped.to_string()
}
