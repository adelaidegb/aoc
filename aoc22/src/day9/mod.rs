use std::borrow::Borrow;
use std::cell::Cell;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, MulAssign, Sub};
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl MulAssign for Coord {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Default for Coord {
    fn default() -> Self {
        Coord { x: 0, y: 0 }
    }
}

trait RopeSegment {
    fn get_coord(&self) -> &Cell<Coord>;

    fn tail_chase_head(&self);

    fn visualize_stack(&self);
}

struct RopeHead {
    head: Cell<Coord>,
}

impl RopeSegment for RopeHead {
    fn get_coord(&self) -> &Cell<Coord> {
        self.head.borrow()
    }

    fn tail_chase_head(&self) {
        // no-op
    }

    fn visualize_stack(&self) {
        println!("head [{}, {}]", self.head.get().x, self.head.get().y);
    }
}

impl Default for RopeHead {
    fn default() -> Self {
        RopeHead {
            head: Cell::new(Coord::default())
        }
    }
}

struct RopeTail {
    head: Rc<dyn RopeSegment>,
    tail: Cell<Coord>,
}

impl RopeTail {
    fn head_delta(&self) -> Coord {
        self.head.get_coord().get() - self.tail.get()
    }

    fn of(segment: &Rc<dyn RopeSegment>) -> RopeTail {
        RopeTail {
            head: Rc::clone(segment),
            tail: Cell::new(segment.get_coord().get())
        }
    }
}

impl RopeSegment for RopeTail {
    fn get_coord(&self) -> &Cell<Coord> {
        self.tail.borrow()
    }

    fn tail_chase_head(&self) {
        /*
         * Recursive head call.. literally.
         * This will make sure the most shallow RopeTail updates first,
         * and all other RopeTails that inherit from it update in the correct sequence.
         */
        self.head.tail_chase_head();

        let move_delta = match self.head_delta() {
            // no movement required
            Coord { x, y } if x.abs() <= 1 && y.abs() <= 1 => return,
            // move one vertically or horizontally, if that is sufficient
            Coord { x, y } if (x == 0 || y == 0) && (x.abs() + y.abs() == 2) => Coord { x: x.signum(), y: y.signum() },
            // move one diagonally, if that is sufficient
            Coord { x, y } if x.abs() <= 2 && y.abs() <= 2 => Coord { x: x.signum(), y: y.signum() },
            // otherwise unsupported
            Coord { x, y } => panic!("unsupported head delta [{x}, {y}]")
        };

        self.tail.set(self.tail.get() + move_delta);
    }

    fn visualize_stack(&self) {
        self.head.visualize_stack();

        println!("tail [{}, {}]", self.tail.get().x, self.tail.get().y);
    }
}

#[derive(Debug)]
enum Movement {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Movement {
    fn get_magnitude(&self) -> &i32 {
        match self {
            Self::Up(n) | Self::Down(n) | Self::Left(n) | Self::Right(n) => n
        }
    }
    fn get_unit_delta(&self) -> Coord {
        match self {
            Self::Up(..) => Coord { x: 0, y: 1 },
            Self::Down(..) => Coord { x: 0, y: -1 },
            Self::Left(..) => Coord { x: -1, y: 0 },
            Self::Right(..) => Coord { x: 1, y: 0 },
        }
    }
}

fn parse_moves(input: &String) -> Vec<Movement> {
    let mut moves: Vec<Movement> = Vec::new();

    for line in input.lines() {
        let mut split = line.splitn(2, " ");
        let dir = split.next().unwrap();
        let mag: i32 = split.next().unwrap().parse().unwrap();

        moves.push(match dir {
            "U" => Movement::Up(mag),
            "D" => Movement::Down(mag),
            "L" => Movement::Left(mag),
            "R" => Movement::Right(mag),
            _ => panic!("unrecognized direction command: {dir}")
        });
    }

    moves
}

fn simulate_tracked_tail(moves: &Vec<Movement>) -> HashSet<Coord> {
    let head: Rc<dyn RopeSegment> = Rc::new(RopeHead::default());
    let tail = RopeTail::of(&head);
    let mut tracks: HashSet<Coord> = HashSet::new();
    tracks.insert(tail.tail.get());

    for m in moves.iter() {
        let delta = m.get_unit_delta();

        for _ in 0..*m.get_magnitude() {
            head.get_coord().set(head.get_coord().get() + delta);
            tail.tail_chase_head();

            tracks.insert(tail.tail.get());
        }
    }

    tracks
}

fn simulate_n_tails(moves: &Vec<Movement>, n: u32) -> HashSet<Coord> {
    if n < 1 {
        panic!("cannot simulate movement with no tail");
    }

    let head: Rc<dyn RopeSegment> = Rc::new(RopeHead::default());
    let mut tail: Rc<dyn RopeSegment> = Rc::clone(&head);

    for _ in 0..n {
        tail = Rc::new(RopeTail::of(&tail));
    }

    let mut tracks: HashSet<Coord> = HashSet::new();
    tracks.insert(tail.get_coord().get());

    for m in moves.iter() {
        let delta = m.get_unit_delta();

        for _ in 0..*m.get_magnitude() {
            head.get_coord().set(head.get_coord().get() + delta);
            tail.tail_chase_head();

            tracks.insert(tail.get_coord().get());
        }
    }

    tracks
}

#[aoc(day=9, part=1)]
fn part1(input: String) -> String {
    let moves = parse_moves(&input);

    let visited = simulate_tracked_tail(&moves);

    visited.len().to_string()
}

#[aoc(day=9, part=2)]
fn part2(input: String) -> String {
    let moves = parse_moves(&input);

    let visited = simulate_n_tails(&moves, 9);

    visited.len().to_string()
}
