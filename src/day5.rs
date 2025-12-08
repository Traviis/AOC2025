type InputType = (Vec<Range>, Vec<u64>);
type OutputType = u64;

type Range = (u64, u64);

#[aoc_generator(day5)]
fn day5_parse(input: &str) -> InputType {
    // grab ranges first
    let mut parsing_ranges = true;
    let mut ranges: Vec<Range> = Vec::new();
    let mut vals = Vec::new();

    for line in input.lines() {
        if line == "" {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let mut x = line.split("-");
            ranges.push((
                x.next().unwrap().parse::<u64>().unwrap(),
                x.next().unwrap().parse::<u64>().unwrap(),
            ));
        } else {
            vals.push(line.parse::<u64>().unwrap());
        }
    }
    (ranges, vals)
}

#[aoc(day5, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let (good_ranges, checks) = input;

    let mut good_ingredients = 0;

    'checks: for check in checks.iter() {
        for (low, high) in good_ranges.iter() {
            if check >= low && check <= high {
                good_ingredients += 1;
                continue 'checks;
            }
        }
    }

    good_ingredients
}

#[aoc(day5, part2)]
pub fn part2(input: &InputType) -> OutputType {
    //We obviously don't want to just try every single value So we need to combine, all the ranges
    //as they may overlap; Since we know they're contigious on a range, we can start combining
    //ranges until we cannot combine anymore. You can combine iff the start of another range is <=
    //the top end of the range in question
    let (mut ranges, _) = input.clone();
    ranges.sort_unstable_by_key(|x| x.0);
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if merged.is_empty() || start > merged.last().unwrap().1 {
            merged.push((start, end));
        } else {
            let last = merged.last_mut().unwrap();
            //The range can be fully enclosed, so find the max of the two, we don't want to trim off
            last.1 = last.1.max(end);
        }
    }
    merged.into_iter().map(|(l, h)| h - l + 1).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
    }

    #[test]
    fn day5_part1() {
        assert_eq!(part1(&day5_parse(get_test_input())), 3);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(&day5_parse(get_test_input())), 14);
    }
}
