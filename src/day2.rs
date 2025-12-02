use std::str::FromStr;

use anyhow::{Result, anyhow};

type InputType = Vec<Range>;
type OutputType = u64;

#[derive(Clone)]
struct Range {
    lower: u64,
    upper: u64,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (lower, upper) = s.split_once('-').ok_or_else(|| anyhow!("Idon'tknow"))?;
        //println!("{}-{}",lower,upper);
        let lower = lower.parse::<u64>()?;
        let upper = upper.parse::<u64>()?;
        Ok(Range { lower, upper })
    }
}

fn is_number_repeating(n: u64) -> bool {
    let s = n.to_string();
    let half = s.len() / 2; //TODO: Need to ceil?

    //println!("{}-{}", s[0..half].to_string(), s[half..].to_string());

    s[0..half] == s[half..]
}

impl Range {
    fn find_invalid(&self) -> Vec<u64> {
        (self.lower..=self.upper)
            .filter(|x| is_number_repeating(*x))
            .collect::<Vec<_>>()
    }
}

#[aoc_generator(day2)]
fn day2_parse(input: &str) -> InputType {
    input
        .split(",")
        .map(|x| Range::from_str(x))
        .collect::<Result<Vec<Range>>>()
        .unwrap()
}

#[aoc(day2, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|x| x.find_invalid().into_iter().sum::<u64>())
        .sum::<u64>()
}

#[aoc(day2, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    #[test]
    fn day2_simple_count() {
        let range = Range {
            lower: 11,
            upper: 22,
        };
        assert_eq!(range.find_invalid().len(), 2);
    }

    #[test]
    fn day2_part1() {
        assert_eq!(part1(&day2_parse(get_test_input())), 1227775554);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(part2(&day2_parse(get_test_input())), 0);
    }
}
