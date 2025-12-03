type InputType = Vec<Bank>;
type OutputType = u64;

type Bank = Vec<u8>;

#[aoc_generator(day3)]
fn day3_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            line.chars().fold(Bank::new(), |mut acc, c| {
                acc.push(c.to_digit(10).unwrap() as u8);
                acc
            })
        })
        .collect::<Vec<Bank>>()
}

fn max_joltage(bank: &Bank) -> u64 {
    let bank_len = bank.len();
    //print!("{:?}", bank);
    for n in (1..=9).rev() {
        let n = n as u8;
        //println!("Checking bank for {}",n);
        let found_n = bank.iter().position(|i| *i == n);
        let idx = match found_n {
            Some(n) if n < bank_len - 1 => n + 1,
            _ => continue, // We didn't find any n, or it was at the end
        };

        let next_digit = bank.iter().skip(idx).max().unwrap();

        //println!(" = {}{}",n, next_digit);
        return ((n * 10) + next_digit).into();
    }
    //println!("{:?}",bank);
    panic!("Didn't find max joltage in bank");
}

fn max_joltage_part_2(bank: &Bank) -> u64 {
    0
}

#[aoc(day3, part1)]
pub fn part1(input: &InputType) -> OutputType {
    // Let's be naive. Start by searching for 9, go down each missed step, if you find n, make sure
    // that there is at least on additional digit behind it (for 2), then you have your maximum
    // number, so go from there, if you have more than one, find the max number in that list.
    input.iter().map(|bank| max_joltage(bank)).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &InputType) -> OutputType {
    input.iter().map(|bank| max_joltage_part_2(bank)).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "987654321111111
811111111111119
234234234234278
818181911112111"
    }

    #[test]
    fn day3_part1() {
        assert_eq!(part1(&day3_parse(get_test_input())), 357);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(&day3_parse(get_test_input())), 3121910778619);
    }
}
