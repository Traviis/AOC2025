type InputType = (Vec<Range>, Vec<u64>);
type OutputType = u64;

type Range = (u64,u64);


#[aoc_generator(day5)]
fn day5_parse(input: &str) -> InputType {
    // grab ranges first
    let mut parsing_ranges = true;
    let mut ranges : Vec<Range> = Vec::new();
    let mut vals = Vec::new();

    for line in input.lines(){
        if line == "" {
            parsing_ranges = false;
            continue;
        }


        if parsing_ranges {
            let mut x = line.split("-");
            ranges.push((x.next().unwrap().parse::<u64>().unwrap(),x.next().unwrap().parse::<u64>().unwrap()));
        } else {
            vals.push(line.parse::<u64>().unwrap());
        }
    }
    (ranges,vals)
}

#[aoc(day5, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let (good_ranges, checks) = input;

    let mut good_ingredients = 0;

    'checks: for check in checks.iter() {
        for (low,high) in good_ranges.iter() {
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
    todo!();
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
        assert_eq!(part2(&day5_parse(get_test_input())), 0);
    }
}
