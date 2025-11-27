use std::collections::BTreeMap;

type InputType = (Vec<i32>, Vec<i32>);
type OutputType = i32;

#[aoc_generator(day1)]
fn day1_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day1, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day1, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "3   4
4   3
2   5
1   3
3   9
3   3"
    }

    #[test]
    fn day1_part1() {
        assert_eq!(part1(&day1_parse(get_test_input())), 11);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(&day1_parse(get_test_input())), 31);
    }
}
