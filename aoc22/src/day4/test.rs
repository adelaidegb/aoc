use super::*;

#[aoc_test(day4)]
pub fn test_part1(input: String) {
    assert_eq!(find_entirely_overlapping(input), "569");
}

#[aoc_test(day4)]
pub fn test_part2(input: String) {
    assert_eq!(find_partially_overlapping(input), "936");
}
