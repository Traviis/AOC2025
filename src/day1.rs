use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

type InputType = Vec<DialAction>;
type OutputType = i32;

#[derive(Debug)]
pub enum DialAction {
    L(i32),
    R(i32),
}

impl FromStr for DialAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars();
        let dir = chars.next().unwrap();
        let numbers = chars.collect::<String>().parse::<i32>()?;
        match dir {
            'L' => Ok(DialAction::L(numbers)),
            'R' => Ok(DialAction::R(numbers)),
            _ => return Err(anyhow!("Unknown direction")),
        }
    }
}

#[aoc_generator(day1)]
fn day1_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| DialAction::from_str(line))
        .collect::<Result<Vec<_>>>()
        .expect("Failed to parse")
}

#[aoc(day1, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut position = 50;
    let mut zeroes = 0;

    for dial_move in input.iter() {
        position = match dial_move {
            DialAction::L(n) => (position - n).rem_euclid(100),
            DialAction::R(n) => (position + n).rem_euclid(100),
        };
        if position == 0 {
            zeroes = zeroes + 1;
        }
    }

    zeroes
}

#[aoc(day1, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let mut position = 50;
    let mut zero_passes = 0;

    for dial_move in input.iter() {
        position = match dial_move {
            DialAction::L(n) => {
                // You can't use standard logic here for division. We also need to account for the
                // case where we are starting at 0, and we take -1 from the position from both the
                // start and end to see if we pass over 0. The logic is simply confusing when you
                // start at 0 at the start of an instruction
                let start_gen = (position - 1 as i32).div_euclid(100);
                let end_gen = (position - n - 1).div_euclid(100);

                zero_passes += (start_gen - end_gen).abs();
                (position - n).rem_euclid(100)
            }
            DialAction::R(n) => {
                zero_passes += (position + n) / 100;
                (position + n).rem_euclid(100)
            }
        };
    }
    zero_passes
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
    }

    #[test]
    fn day1_part1() {
        assert_eq!(part1(&day1_parse(get_test_input())), 3);
    }

    #[test]
    fn day1_matchcheck() {
        assert_eq!((-1i32).rem_euclid(100), 99);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(&day1_parse(get_test_input())), 6);
    }
}
