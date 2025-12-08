use std::{fmt, str::FromStr};
use anyhow::Result;

type InputType = Vec<Problem>;
type OutputType = i64;

enum Op {
    Mult,
    Add,
    Unknown,
}

struct Problem {
   op: Op,
   nums: Vec<i64>
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
            write!(f, "{} {}",n, self.op.to_string());
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
        self.nums.iter().fold(init_value, |acc,v| 
            match self.op {
            Op::Mult => acc * v,
            Op::Add => acc + v,
            _ => panic!("Bad op"),
            }
            )
    }
}


#[aoc_generator(day6)]
fn day6_parse(input: &str) -> InputType {

    let mut probs = Vec::new();
    //This is just really complicated Parsing
    for (line_num,line) in input.lines().enumerate() {
        for (prob_num, item) in line.split_whitespace().enumerate() {
            let prob = match probs.get_mut(prob_num) {
                Some(prob) => prob,
                None => {
                    probs.push(Problem{op: Op::Unknown, nums: vec!()});
                    probs.last_mut().unwrap()
                }
            };

            if let Ok(n) = item.parse::<i64>() {
                prob.nums.push(n);
            } else if let Ok(op) = Op::from_str(item) {
                prob.op = op;
            } else {
                panic!("Unparsable {:?}",item);
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
    todo!();
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
        assert_eq!(part2(&day6_parse(get_test_input())), 0);
    }
}
