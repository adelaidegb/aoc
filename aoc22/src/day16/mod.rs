use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use lazy_static::lazy_static;
use regex::Regex;

type SharedValve = Rc<RefCell<Valve>>;
type PathCache = Box<RefCell<HashMap<String, Option<u32>>>>;

struct Valve {
    name: String,
    flow: i32,
    connected: HashMap<String, SharedValve>,
    cached_paths: PathCache,
}

impl Valve {
    fn get_path_length(&self, goal: &Valve) -> Option<u32> {
        if let Some(cached) = self.cached_paths.borrow().get(&goal.name) {
            // return cached result
            return cached.clone();
        }

        // otherwise, calculate and cache
        let path = self.force_calc_path_length(goal);

        self.cached_paths.borrow_mut().insert(goal.name.clone(), path.clone());
        goal.cached_paths.borrow_mut().insert(self.name.clone(), path.clone());

        path
    }

    fn force_calc_path_length(&self, goal: &Valve) -> Option<u32> {
        // conduct a breadth-first search to find the shortest path between Valves
        let mut current: Vec<HashMap<String, SharedValve>> = vec![self.connected.clone()];
        let mut visited: HashSet<String> = HashSet::from([self.name.clone()]);
        let mut depth = 0;
        let mut next: Vec<HashMap<String, SharedValve>> = Vec::new();

        loop {
            depth += 1;

            while let Some(connections) = current.pop() {
                for connected_sv in connections.values() {
                    let connected_valve = connected_sv.borrow();

                    if connected_valve.name == goal.name {
                        return Some(depth);
                    }

                    if visited.insert(connected_valve.name.clone()) {
                        next.push(connected_valve.connected.clone());
                    }
                }
            }

            if next.is_empty() {
                return None;
            }

            next.clone_into(&mut current);
            next.clear();
        }
    }
}

#[derive(Clone)]
struct Action {
    minute_cost: u32,
    pressure_released: i32,
}

impl Action {
    fn consider(start: SharedValve, dest: SharedValve, minutes_remaining: i32) -> Option<Action> {
        let valve = dest.borrow();

        if valve.flow < 1 {
            // take no action, as there is nothing to gain
            return None;
        }

        let minute_cost = match start.borrow().get_path_length(&valve) {
            Some(movement_cost) => movement_cost + 1,
            None => return None
        };
        let pressure_released = (minutes_remaining - minute_cost as i32) * valve.flow;

        if pressure_released <= 0 {
            // take no action, as we don't have time to benefit
            return None;
        }

        Some(Action {
            minute_cost,
            pressure_released
        })
    }

    fn noop() -> Action {
        Action {
            minute_cost: 0,
            pressure_released: 0,
        }
    }
}

struct Graph {
    significant_nodes: HashMap<String, SharedValve>,
    start: SharedValve,
    minute_limit: i32,
}

impl Graph {
    fn find_optimal_moves(&self) -> Option<i32> {
        // conduct a depth-first search to find the optimal sequence of moves
        let max = Cell::new(None);

        self.find_next_optimal_move(Rc::clone(&self.start), &mut Vec::new(), &max);

        max.get()
    }

    fn find_next_optimal_move(&self, from: SharedValve, action_stack: &mut Vec<(String, Action)>, max: &Cell<Option<i32>>) {
        let minutes_remaining = self.minute_limit - action_stack.iter().map(|(_, action)| action.minute_cost).sum::<u32>() as i32;
        let mut end = true;

        for (name, candidate) in self.significant_nodes.iter() {
            if action_stack.iter().any(|(visited, _)| visited == name) {
                // we've already visited this valve
                continue;
            }

            if let Some(action) = Action::consider(Rc::clone(&from), Rc::clone(candidate), minutes_remaining) {
                end = false;
                action_stack.push((name.clone(), action));

                // proceed to this valve and continue from there
                self.find_next_optimal_move(Rc::clone(candidate), action_stack, max);

                action_stack.pop();
            }
        }

        if end {
            // when there's no more progress to be made, record our result
            let total_released = action_stack.iter().map(|(_, action)| action.pressure_released).sum();

            max.set(Some(max.get().map_or(total_released, |former_max| former_max.max(total_released))));
        }
    }

    fn find_optimal_moves_with_elephant(&self) -> Option<i32> {
        // conduct a depth-first search with two cursors to find the optimal sequence of moves with an elephant helper
        let max = Cell::new(None);

        self.find_next_optimal_move_with_elephant(Rc::clone(&self.start), Rc::clone(&self.start), &mut Vec::new(), &mut Vec::new(), &max);

        max.get()
    }

    fn find_next_optimal_move_with_elephant(&self, from_1: SharedValve, from_2: SharedValve, action_stack_1: &mut Vec<(String, Action)>, action_stack_2: &mut Vec<(String, Action)>, max: &Cell<Option<i32>>) {
        // TODO this is horribly unoptimized; would be nice to clean it up
        let minutes_remaining_1 = self.minute_limit - action_stack_1.iter().map(|(_, action_1)| action_1.minute_cost).sum::<u32>() as i32;
        let minutes_remaining_2 = self.minute_limit - action_stack_2.iter().map(|(_, action_2)| action_2.minute_cost).sum::<u32>() as i32;
        let mut end_1 = true;
        let mut end_2 = true;

        let mut visited: Vec<String> = Vec::new();
        visited.extend(action_stack_1.iter().map(|(name, _)| name.clone()));
        visited.extend(action_stack_2.iter().map(|(name, _)| name.clone()));

        for (name_1, candidate_1) in self.significant_nodes.iter() {
            if visited.contains(name_1) {
                continue;
            }

            if let Some(action_1) = Action::consider(Rc::clone(&from_1), Rc::clone(candidate_1), minutes_remaining_1) {
                end_1 = false;
                action_stack_1.push((name_1.clone(), action_1));
                visited.push(name_1.clone());

                for (name_2, candidate_2) in self.significant_nodes.iter() {
                    if visited.contains(name_2) {
                        continue;
                    }

                    if let Some(action_2) = Action::consider(Rc::clone(&from_2), Rc::clone(candidate_2), minutes_remaining_2) {
                        end_2 = false;
                        action_stack_2.push((name_2.clone(), action_2));

                        self.find_next_optimal_move_with_elephant(Rc::clone(candidate_1), Rc::clone(candidate_2), action_stack_1, action_stack_2, max);

                        action_stack_2.pop();
                    }
                }

                if end_2 {
                    // allow 1 to continue even when 2 doesn't
                    action_stack_2.push((action_stack_2.last().unwrap().0.clone(), Action::noop()));

                    self.find_next_optimal_move_with_elephant(Rc::clone(candidate_1), Rc::clone(&from_2), action_stack_1, action_stack_2, max);

                    action_stack_2.pop();
                }

                visited.pop();
                action_stack_1.pop();
            }
        }

        if end_1 {
            // allow 2 to continue even when 1 doesn't
            action_stack_1.push((action_stack_1.last().unwrap().0.clone(), Action::noop()));

            // 1 didn't go anywhere, so 2 had no chance to
            // iterate; if 2 goes anywhere, push an Action::noop() onto stack 1 and recurse
            for (name_2, candidate_2) in self.significant_nodes.iter() {
                if visited.contains(name_2) {
                    continue;
                }

                if let Some(action_2) = Action::consider(Rc::clone(&from_2), Rc::clone(candidate_2), minutes_remaining_2) {
                    end_2 = false;
                    action_stack_2.push((name_2.clone(), action_2));

                    self.find_next_optimal_move_with_elephant(Rc::clone(&from_1), Rc::clone(candidate_2), action_stack_1, action_stack_2, max);

                    action_stack_2.pop();
                }
            }

            action_stack_1.pop();
        }

        if end_1 && end_2 {
            // when nobody can make any more progress, record our result
            let total_released = action_stack_1.iter().map(|(_, action)| action.pressure_released).sum::<i32>()
                + action_stack_2.iter().map(|(_, action)| action.pressure_released).sum::<i32>();

            max.set(Some(max.get().map_or(total_released, |former_max| former_max.max(total_released))));
        }
    }
}

const STARTING_VALVE: &'static str = "AA";
const STARTING_MINUTES: i32 = 30;

fn parse_valves(input: &String) -> Graph {
    lazy_static! {
        static ref VALVE_REGEX: Regex = Regex::new(r"Valve (\w+) has flow rate=([\d-]+); tunnels? leads? to valves? ([\w, ]+)").unwrap();
    }

    let mut valves = HashMap::new();
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let cap = VALVE_REGEX.captures(line).unwrap();
        let name = cap[1].to_string();

        connections.insert(name.clone(), cap[3].split(", ").map(|s| s.to_string()).collect());
        valves.insert(name.clone(), Rc::new(RefCell::new(Valve {
            name,
            flow: cap[2].parse().unwrap(),
            connected: HashMap::new(),
            cached_paths: Box::new(RefCell::new(HashMap::new()))
        })));
    }

    for (name, valve) in valves.iter() {
        for connected in connections.get(name).unwrap().iter() {
            valve.borrow_mut().connected.insert(connected.clone(), Rc::clone(valves.get(connected).unwrap()));
        }
    }

    // note: any fully orphaned SharedValue nodes will be dropped as we filter valves into only values with flow > 0
    Graph {
        start: Rc::clone(valves.get(STARTING_VALVE).unwrap()),
        significant_nodes: valves.iter()
            .filter(|(_, sv)| sv.borrow().flow > 0)
            .map(|(k, v)| (k.clone(), Rc::clone(v))).collect(),
        minute_limit: STARTING_MINUTES
    }
}

#[aoc(day=16, part=1)]
fn part1(input: String) -> String {
    let graph = parse_valves(&input);

    match graph.find_optimal_moves() {
        Some(pressure_released) => pressure_released.to_string(),
        None => "no valid paths found".to_string()
    }
}

#[aoc(day=16, part=2)]
fn part2(input: String) -> String {
    let mut graph = parse_valves(&input);

    // elephant tax
    graph.minute_limit -= 4;

    match graph.find_optimal_moves_with_elephant() {
        Some(pressure_released) => pressure_released.to_string(),
        None => "no valid paths found".to_string()
    }
}
