use std::collections::HashSet;

enum ViewingDistance {
    Blocked(usize),
    Open(usize)
}

impl ViewingDistance {
    fn is_clear(&self) -> bool {
        match self {
            Self::Open(..) => true,
            _ => false
        }
    }

    fn get_trees_visible(&self) -> &usize {
        match self {
            Self::Blocked(n) => n,
            Self::Open(n) => n
        }
    }

    fn any_clear(spread: &[ViewingDistance; 4]) -> bool {
        spread.iter().any(|vd| vd.is_clear())
    }

    fn scenic_score(spread: &[ViewingDistance; 4]) -> usize {
        spread.iter().map(|vd| vd.get_trees_visible()).product()
    }
}

fn map_to_matrix(input: &String) -> Vec<Vec<i8>> {
    let width = input.lines().map(|line| line.len()).max().unwrap();
    let height = input.lines().count();

    // trees[y][x] == height
    let mut trees = vec![vec![0i8; width]; height];

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            trees[j][i] = match c {
                '0'..='9' => (c as i8) - ('0' as i8),
                _ => panic!("unsupported character found: {c}")
            };
        }
    }

    trees
}

fn calc_visibility(trees: &Vec<Vec<i8>>, x: usize, y: usize, width: usize, height: usize) -> [ViewingDistance; 4] {
    let tree: i8 = trees[y][x];
    let mut up_vd;
    let mut down_vd;
    let mut left_vd;
    let mut right_vd;

    // return false early when encountering a blocking tree

    {
        let up = (0..y).rev();
        up_vd = ViewingDistance::Open(up.len());

        for j in up {
            if trees[j][x] >= tree {
                // println!("tree of height {} at ({x}, {j}) is in the way of tree ({x}, {y}) of height {tree}", trees[j][x]);
                up_vd = ViewingDistance::Blocked(y - j);
                break;
            }
        }
    }

    {
        let down = y+1..height;
        down_vd = ViewingDistance::Open(down.len());

        for j in down {
            if trees[j][x] >= tree {
                // println!("tree of height {} at ({x}, {j}) is in the way of tree ({x}, {y}) of height {tree}", trees[j][x]);
                down_vd = ViewingDistance::Blocked(j - y);
                break;
            }
        }
    }

    // note: the row dimension (x) could be optimized to use trees[y][range].max() >= tree as a check for disqualifying that direction

    {
        let left = (0..x).rev();
        left_vd = ViewingDistance::Open(left.len());

        for i in left {
            if trees[y][i] >= tree {
                // println!("tree of height {} at ({i}, {y}) is in the way of tree ({x}, {y}) of height {tree}", trees[y][i]);
                left_vd = ViewingDistance::Blocked(x - i);
                break;
            }
        }
    }

    {
        let right = x+1..width;
        right_vd = ViewingDistance::Open(right.len());

        for i in right {
            if trees[y][i] >= tree {
                // println!("tree of height {} at ({i}, {y}) is in the way of tree ({x}, {y}) of height {tree}", trees[y][i]);
                right_vd = ViewingDistance::Blocked(i - x);
                break;
            }
        }
    }

    // no blocking trees were found
    [up_vd, down_vd, left_vd, right_vd]
}

fn count_visible(trees: &Vec<Vec<i8>>) -> u32 {
    let mut visible = 0u32;
    let height = trees.len();

    for (j, row) in trees.iter().enumerate() {
        let width = row.len();

        for (i, _) in row.iter().enumerate() {
            if !(1..height-1).contains(&j) || !(1..width-1).contains(&i) {
                // edges are always visible, and therefore a trivial case
                println!("({i}, {j}) is visible as it's on the edge");
                visible += 1;
                continue;
            }

            if ViewingDistance::any_clear(&calc_visibility(trees, i, j, width, height)) {
                println!("({i}, {j}) is visible as per is_visible");
                visible += 1;
            }
        }
    }

    visible
}

fn max_scenic(trees: &Vec<Vec<i8>>) -> usize {
    let mut scenic_scores: HashSet<usize> = HashSet::new();
    let height = trees.len();

    for (j, row) in trees.iter().enumerate() {
        let width = row.len();

        for (i, _) in row.iter().enumerate() {
            if !(1..height-1).contains(&j) || !(1..width-1).contains(&i) {
                // edges always have a scenic score of 0, so we will skip them
                continue;
            }

            scenic_scores.insert(ViewingDistance::scenic_score(&calc_visibility(trees, i, j, width, height)));
        }
    }

    *scenic_scores.iter().max().unwrap()
}

#[aoc(day=8, part=1)]
fn part1(input: String) -> String {
    let trees = map_to_matrix(&input);

    count_visible(&trees).to_string()
}

#[aoc(day=8, part=2)]
fn part2(input: String) -> String {
    let trees = map_to_matrix(&input);

    max_scenic(&trees).to_string()
}
