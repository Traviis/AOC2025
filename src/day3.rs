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

fn calculate_joltage_part_2(bank: &Bank, indexes: &Vec<usize>) -> u64 {
    let mut res = 0;
    // Find this value by taking the index of the number, and multipling that by 10^length of
    // string remaining.
    for inv_mag in 1..=12 {
        let value = bank[indexes[inv_mag - 1]];
        let mag_mult = 10u64.pow((12 - inv_mag) as u32);
        res += value as usize * mag_mult as usize;
    }
    return res as u64;
}

fn max_joltage_part_2(bank: &Bank) -> u64 {
    let mut chosen_indexes = Vec::<usize>::new();

    //Build vector until we have enough indexes
    while chosen_indexes.len() < 12 {
        //Start at the next spot from the last found index
        let start_search_at = chosen_indexes.last().map_or(0, |&i| i + 1);
        let mut found_in_step = false;

        // Starting with 9, find the biggest number
        for n in (1..=9).rev() {
            let n = n as u8;

            // Now that we have n, look through the bank (from the start) and try to find the
            // number
            let found_n_idx = match bank[start_search_at..].iter().position(|&x| x == n) {
                Some(rel_idx) => start_search_at + rel_idx,
                None => continue, // Number not found, try next n
            };

            // The logic here is you can only use that found_n if you have characters available
            // (enough to finish). The remaining needed is given by (requested_len -
            // chosen_indexes.len() - 1) and the characters available (if you were to use this
            // n) = (bank.len() - found_n - 1), found_n being the candidate index
            let remaining_needed = 12 - chosen_indexes.len() - 1;
            let remaining_available = bank.len() - found_n_idx - 1;

            // We can do this, because since they are power of 10 each slot, it is always the
            // correct move to get the biggest n as long as there is space left over to finish
            if remaining_needed <= remaining_available {
                chosen_indexes.push(found_n_idx);
                found_in_step = true;
                break; // Stop searching for n, go back to the while loop
            }
        }

        if !found_in_step {
            panic!("Could not find a valid number to continue sequence");
        }
    }

    return calculate_joltage_part_2(bank, &chosen_indexes);
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
    fn day3_calc_jolt_part2() {
        let bank: Bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        let indexes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(calculate_joltage_part_2(&bank, &indexes), 987654321111);
    }
    #[test]
    fn day3_part2_max() {
        let bank: Bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(max_joltage_part_2(&bank), 888911112111);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(&day3_parse(get_test_input())), 3121910778619);
    }
}
