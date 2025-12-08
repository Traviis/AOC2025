use anyhow::Result;
use std::{fmt, str::FromStr};

type InputType = Vec<Problem>;
type OutputType = i64;

#[derive(Clone)]
enum Op {
    Mult,
    Add,
    Unknown,
}

struct Problem {
    op: Op,
    nums: Vec<i64>,
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mult),
            _ => panic!("Unknown Op"),
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Mult => "*".to_string(),
            Op::Add => "+".to_string(),
            Op::Unknown => "?".to_string(),
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for n in self.nums.iter() {
            write!(f, "{} {}", n, self.op.to_string());
        })
    }
}

impl Problem {
    fn execute(&self) -> i64 {
        let init_value = match self.op {
            Op::Mult => 1,
            Op::Add => 0,
            Op::Unknown => panic!("Unknown op"),
        };
        self.nums.iter().fold(init_value, |acc, v| match self.op {
            Op::Mult => acc * v,
            Op::Add => acc + v,
            _ => panic!("Bad op"),
        })
    }

    fn execute_part2(&self) -> i64 {
        //Ok, so the numbers are all parsed correctly, however, we need to rejigger things a bit
        //for part 2

        // We can ignore outside of the problem (the text goes right-to-left for the whole set of
        // problems, but that's irrelevant since we are summing at the end

        // 64
        // 23
        // 314
        // +
        // This now equals 4 + 431 + 632
        //Let's parse each number to a string, then, in order
        let str_nums = self.nums.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        let max_len = str_nums.iter().map(|x| x.len()).max().unwrap();
        let mut new_nums = Vec::new();
        for idx in 1..=max_len {
            //Going from the max_len, down to length , grab the idx of each string (or nothing)
            println!("idx {}", idx);
            let mut con_num: Vec<char> = Vec::new();
            for sn in str_nums.iter() {
                if let Some(c) = sn.chars().rev().nth(idx - 1) {
                    con_num.push(c);
                }
            }
            println!("{:?}", con_num);
            if con_num.len() > 0 {
                new_nums.push(con_num.iter().collect::<String>().parse::<i64>().unwrap());
            }
        }

        // Be lazy
        Problem {
            op: self.op.clone(),
            nums: new_nums,
        }
        .execute()
    }
}

#[aoc_generator(day6)]
fn day6_parse(input: &str) -> InputType {
    let mut probs = Vec::new();
    //This is just really complicated Parsing
    for (line_num, line) in input.lines().enumerate() {
        for (prob_num, item) in line.split_whitespace().enumerate() {
            let prob = match probs.get_mut(prob_num) {
                Some(prob) => prob,
                None => {
                    probs.push(Problem {
                        op: Op::Unknown,
                        nums: vec![],
                    });
                    probs.last_mut().unwrap()
                }
            };

            if let Ok(n) = item.parse::<i64>() {
                prob.nums.push(n);
            } else if let Ok(op) = Op::from_str(item) {
                prob.op = op;
            } else {
                panic!("Unparsable {:?}", item);
            }
        }
    }
    probs
}

#[aoc(day6, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input.iter().map(|p| p.execute()).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &InputType) -> OutputType {
    input.iter().map(|p| p.execute_part2()).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +"
    }

    #[test]
    fn day6_part1() {
        assert_eq!(part1(&day6_parse(get_test_input())), 4277556);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(&day6_parse(get_test_input())), 3263827);
    }

    #[test]
    fn day6_part2_simple() {
        assert_eq!(
            Problem {
                nums: vec!(64, 23, 314),
                op: Op::Add
            }
            .execute_part2(),
            1058
        );
    }
    #[test]
    fn day6_part2_simple2() {
        assert_eq!(
            Problem {
                nums: vec!(123, 45, 6),
                op: Op::Mult
            }
            .execute_part2(),
            8544
        );
    }
}
