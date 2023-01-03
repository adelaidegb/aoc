use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use bitvec::prelude::{BitVec, Lsb0};

type VisitedBits = BitVec<u8, Lsb0>;

#[derive(Copy, Clone)]
struct MapCell {
    elevation: u8,
    start: bool,
    end: bool,
}

struct ElevationMap {
    grid: Vec<Vec<MapCell>>,
    start: (usize, usize),
    end: (usize, usize),
}

type SharedImc<'a> = Rc<RefCell<IndexedMapCell<'a>>>;

struct MapIndexer<'a, 'b> {
    map: &'a ElevationMap,
    cache: HashMap<(usize, usize), SharedImc<'b>>,
    visited: VisitedBits,
    queue: Vec<SharedImc<'b>>,
    width: usize,
}

struct IndexedMapCell<'a> {
    x: usize,
    y: usize,
    index: usize,
    /**
     * If you visualise the cells in the grid as vertices on a graph, then these connections represent the edges of the graph.
     * Note that connections can have a one-way relationship; the presence of a connection in this cell does not guarantee that the converse is possible.
     */
    connections: Vec<SharedImc<'a>>,
}

struct Pathfinder<'a> {
    total_size: usize,
    start: Vec<SharedImc<'a>>,
    goal: SharedImc<'a>,
}

impl MapCell {
    fn can_reach(&self, other: &MapCell) -> bool {
        self.elevation + 1 >= other.elevation
    }

    fn from_char(mut from: char) -> MapCell {
        let start = from == 'S';
        let end = from == 'E';
        from = match from {
            'S' => 'a',
            'E' => 'z',
            other => other,
        };

        MapCell {
            elevation: match from {
                'a'..='z' => from as usize - 'a' as usize,
                _ => panic!("unsupported elevation char: {from}")
            } as u8,
            start,
            end
        }
    }
}

impl ElevationMap {
    fn print(&self) {
        let cols = self.grid.iter().next().unwrap().iter().count();

        print!(" |");

        for i in 0..cols {
            print!("{}", i % 10);
        }

        print!("\n-+");

        for _ in 0..cols {
            print!("-");
        }

        println!();

        for (j, row) in self.grid.iter().enumerate() {
            print!("{}|", j % 10);

            for cell in row.iter() {
                print!("{}", if cell.start {
                    'S'
                } else if cell.end {
                    'E'
                } else {
                    ('a' as u8 + cell.elevation) as char
                });
            }

            println!();
        }

        println!("\nstart @ {:?}\nend   @ {:?}", self.start, self.end);
    }

    fn get_cell(&self, coord: (usize, usize)) -> &MapCell {
        match coord {
            (x, y) => &self.grid[y][x]
        }
    }

    fn from_input(input: &String) -> ElevationMap {
        let mut grid = vec![Vec::new(); input.lines().count()];
        let mut start = None;
        let mut end = None;

        for (j, line) in input.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                let cell = MapCell::from_char(c);

                grid[j].push(cell);

                if cell.start {
                    start = Some((i, j));
                }

                if cell.end {
                    end = Some((i, j));
                }
            }
        }

        ElevationMap {
            grid,
            start: start.unwrap(),
            end: end.unwrap()
        }
    }
}

impl<'b> MapIndexer<'_, 'b> {
    fn process(&mut self) {
        // process until queue is empty
        loop {
            let cell = match self.queue.pop() {
                Some(imc) => imc,
                None => return
            };

            if self.visited.replace(cell.borrow().index, true) {
                continue;
            }

            self.index_adjacent(cell);
        }
    }

    fn index_adjacent(&mut self, shared_cell: SharedImc<'b>) {
        let mut indexed = shared_cell.borrow_mut();

        {
            let row = &self.map.grid[indexed.y];
            let cell = &row[indexed.x];

            if indexed.y > 0 {
                // check (0, -1)
                let coord = (indexed.x, indexed.y - 1);

                if cell.can_reach(self.map.get_cell(coord)) {
                    indexed.connections.push(self.get_or_create(coord));
                }
            }

            if indexed.x > 0 {
                // check (-1, 0)
                let coord = (indexed.x - 1, indexed.y);

                if cell.can_reach(self.map.get_cell(coord)) {
                    indexed.connections.push(self.get_or_create(coord));
                }
            }

            if indexed.y + 1 < self.map.grid.len() {
                // check (0, 1)
                let coord = (indexed.x, indexed.y + 1);

                if cell.can_reach(self.map.get_cell(coord)) {
                    indexed.connections.push(self.get_or_create(coord));
                }
            }

            if indexed.x + 1 < row.len() {
                // check (1, 0)
                let coord = (indexed.x + 1, indexed.y);

                if cell.can_reach(self.map.get_cell(coord)) {
                    indexed.connections.push(self.get_or_create(coord));
                }
            }
        }

        for connected in indexed.connections.iter().map(|imc| Rc::clone(imc)) {
            self.queue.push(connected);
        }
    }

    fn index_cells(&mut self) -> (SharedImc<'b>, SharedImc<'b>) {
        let start = self.get_or_create(self.map.start);

        self.queue.push(Rc::clone(&start));

        self.process();

        match self.cache.get(&self.map.end) {
            Some(end) => (start, Rc::clone(end)),
            _ => unreachable!("malformed map index")
        }
    }

    fn get_or_create(&mut self, coord: (usize, usize)) -> SharedImc<'b> {
        if let Some(cell) = self.cache.get(&coord) {
            return Rc::clone(cell);
        }

        let cell = match coord {
            (x, y) => Rc::new(RefCell::new(IndexedMapCell {
                x,
                y,
                index: y * self.width + x,
                connections: Vec::new()
            }))
        };

        self.cache.insert(coord, Rc::clone(&cell));

        cell
    }
}

// impl IndexedMapCell<'_> {
//     fn print(&self) {
//         println!("cell at ({}, {}) can reach {}", self.x, self.y, self.connections.iter().map(|imc| match imc.borrow() {
//             connected => format!("({}, {})", connected.x, connected.y)
//         }).collect::<Vec<String>>().join(", "));
//     }
// }

impl Pathfinder<'_> {
    fn find_optimal_dist(&mut self) -> Option<usize> {
        self.bfs(self.start.clone())
    }

    fn bfs(&mut self, mut at: Vec<SharedImc>) -> Option<usize> {
        // at represents all cells we are currently at in the current depth
        // each iteration, that view of cells will be replaced with all cells at that new depth
        // cells already in self.visited will not be added (these represent dead ends, or routes that would backtrack or loop)
        let (goal_x, goal_y) = match self.goal.borrow() {
            cell => (cell.x, cell.y)
        };
        let mut depth = 0usize;
        let mut next: Vec<SharedImc> = Vec::new();
        let mut visited: VisitedBits = Self::create_bitvec(self.total_size);

        loop {
            depth += 1;

            while let Some(imc) = at.pop() {
                match imc.borrow() {
                    cell => {
                        // add to next
                        for connected_imc in cell.connections.iter() {
                            match connected_imc.borrow() {
                                connected => {
                                    if connected.x == goal_x && connected.y == goal_y {
                                        // we reached our goal; this is either the shortest path, or tied for shortest
                                        return Some(depth);
                                    }

                                    if !visited.replace(connected.index, true) {
                                        // hasn't yet been visited; add it
                                        next.push(Rc::clone(&connected_imc));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if next.is_empty() {
                // our previous iteration yielded no new cells to visit, so we cannot reach the goal
                return None;
            }

            next.clone_into(&mut at);
            next.clear();
        }
    }

    fn new(map: &ElevationMap) -> Pathfinder {
        let width = map.grid.iter().map(|v| v.len()).max().unwrap();
        let total_size = map.grid.len() * width;
        let (start, goal) = MapIndexer {
            map,
            cache: HashMap::new(),
            visited: Self::create_bitvec(total_size),
            queue: Vec::new(),
            width
        }.index_cells();

        Pathfinder {
            total_size,
            start: vec![start],
            goal
        }
    }

    fn new_any_elevation_zero(map: &ElevationMap) -> Pathfinder {
        let width = map.grid.iter().map(|v| v.len()).max().unwrap();
        let total_size = map.grid.len() * width;
        let mut indexer = MapIndexer {
            map,
            cache: HashMap::new(),
            visited: Self::create_bitvec(total_size),
            queue: Vec::new(),
            width
        };
        let (_, goal) = indexer.index_cells();

        let mut starts: Vec<SharedImc> = Vec::new();

        for (j, row) in map.grid.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if cell.elevation == 0 {
                    starts.push(indexer.get_or_create((i, j)));
                }
            }
        }

        Pathfinder {
            total_size,
            start: starts,
            goal
        }
    }

    fn create_bitvec(len: usize) -> VisitedBits {
        let mut bv = BitVec::from_vec(vec![0u8; (len + 7) / 8]);
        bv.truncate(len);

        bv
    }
}

#[aoc(day=12, part=1)]
fn part1(input: String) -> String {
    let map = ElevationMap::from_input(&input);

    map.print();

    format!("{:?}", Pathfinder::new(&map).find_optimal_dist())
}

#[aoc(day=12, part=2)]
fn part2(input: String) -> String {
    let map = ElevationMap::from_input(&input);

    map.print();

    format!("{:?}", Pathfinder::new_any_elevation_zero(&map).find_optimal_dist())
}
