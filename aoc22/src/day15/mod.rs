use std::collections::HashSet;
use std::ops::RangeInclusive;
use lazy_static::lazy_static;
use regex::Regex;

type CoordinateType = i32;
const DELTA_SIGNS: [[CoordinateType; 2]; 4] = [[-1, -1], [1, -1], [-1, 1], [1, 1]];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: CoordinateType,
    y: CoordinateType,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Signal {
    pos: Coord,
    dist: CoordinateType,
}

impl Coord {
    fn dist(&self, other: &Coord) -> CoordinateType {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Signal {
    fn contains(&self, other: &Coord) -> bool {
        self.pos.dist(other) <= self.dist
    }

    /**
     * Return the range of x values that this Signal contains at the given y value.
     */
    fn contained_at_y(&self, y: CoordinateType) -> RangeInclusive<CoordinateType> {
        let remaining_dist = self.dist - (self.pos.y - y).abs();

        self.pos.x-remaining_dist..=self.pos.x+remaining_dist
    }

    fn get_edges(&self, into: &mut HashSet<Coord>) {
        let max_dist = self.dist + 1;

        for dx in 0..=max_dist {
            let dy = max_dist - dx;

            // note that this technically makes two redundant insertions when one of dx or dy is zero
            for [mx, my] in DELTA_SIGNS {
                into.insert(Coord {
                    x: self.pos.x + dx * mx,
                    y: self.pos.y + dy * my
                });
            }
        }
    }

    fn from_distance_to(point: Coord, other: &Coord) -> Signal {
        Signal {
            dist: point.dist(other),
            pos: point,
        }
    }
}

fn parse_signals(input: &String) -> (Vec<Signal>, Vec<Coord>) {
    lazy_static! {
        static ref SIGNAL_REGEX: Regex = Regex::new(r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)").unwrap();
    }

    let mut signals = Vec::new();
    let mut beacons = Vec::new();

    for line in input.lines() {
        let cap = SIGNAL_REGEX.captures(line).unwrap();
        let beacon = Coord {
            x: cap[3].parse().unwrap(),
            y: cap[4].parse().unwrap()
        };

        signals.push(Signal::from_distance_to(Coord {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap()
        }, &beacon));
        beacons.push(beacon);
    }

    (signals, beacons)
}

// const DESIRED_Y: CoordinateType = 10;
const DESIRED_Y: CoordinateType = 2_000_000;
const COORD_LIMIT: CoordinateType = DESIRED_Y * 2;

#[aoc(day=15, part=1)]
fn part1(input: String) -> String {
    let (signals, beacons) = parse_signals(&input);
    let mut set: HashSet<CoordinateType> = HashSet::new();

    for range in signals.iter().map(|signal| signal.contained_at_y(DESIRED_Y)) {
        set.extend(range);
    }

    for coord in beacons.iter().filter(|coord| coord.y == DESIRED_Y) {
        set.remove(&coord.x);
    }

    set.len().to_string()
}

#[aoc(day=15, part=2)]
fn part2(input: String) -> String {
    let (signals, beacons) = parse_signals(&input);
    let mut edges = HashSet::new();

    for signal in signals.iter() {
        signal.get_edges(&mut edges);
    }

    for coord in beacons.iter() {
        edges.remove(coord);
    }

    edges.retain(|coord| {
        if coord.x < 0 || coord.x > COORD_LIMIT || coord.y < 0 || coord.y > COORD_LIMIT {
            return false;
        }

        !signals.iter().any(|signal| signal.contains(coord))
    });

    let result = match edges.len() {
        0 => return "search yielded no beacons".to_string(),
        1 => edges.iter().next().unwrap(),
        _ => return format!("search yielded multiple beacons: {edges:?}")
    };

    println!("yielded beacon: {result:?}");

    (result.x as i64 * 4_000_000i64 + result.y as i64).to_string()
}
