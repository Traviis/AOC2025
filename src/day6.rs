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
}

#[aoc_generator(day6, part2)]
fn day6_parse_part2(input: &str) -> InputType {
    //Ok, we really should treat this like a grid, instead of numbers Construct lines as an (y,x)
    //=> char (This is a dense map, so use Vec<Vec<_>> instead of a HashMap
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    //Width is the maximum number of characters in any line

    let mut problems = Vec::new();
    let mut current_chunk = Vec::new();

    for x in 0..width {
        //Get the vertical column by going through the height.
        // This is the magic, grab an entire column by checking each line, and seeing if there
        // exists at the x a value, this also grabs the operation as well as the spaces
        let col: Vec<char> = (0..height)
            .map(|y| {
                lines
                    .get(y)
                    .and_then(|row| row.get(x))
                    .copied()
                    .unwrap_or(' ')
            })
            .collect();

        //Let's now check if the column is all whitespace or not, if it is, there isn't a chunk
        //here, so cap off with what we have
        if col.iter().all(|c| c.is_whitespace()) {
            if !current_chunk.is_empty() {
                problems.push(parse_chunk(&current_chunk));
                current_chunk.clear();
            }
        } else {
            //If it's not all whitespace, let's go ahead and just push the col to the current_chunk
            current_chunk.push(col);
        }
    }

    //Handle final column
    if !current_chunk.is_empty() {
        problems.push(parse_chunk(&current_chunk));
    }

    problems
}

fn parse_chunk(cols: &[Vec<char>]) -> Problem {
    let mut op = Op::Unknown;
    let mut nums = Vec::new();

    for (idx, col) in cols.iter().enumerate() {
        //We know that all the operators are left aligned and will show up in the left most column
        if idx == 0 {
            match col.iter().last().unwrap() {
                '*' => op = Op::Mult,
                '+' => op = Op::Add,
                _ => panic!("Unknown op"),
            }
        }

        //Filter all the digits then just jam them together; easier than bothering with special
        //logic for whitespace and operator
        let num_str: String = col.iter().filter(|c| c.is_digit(10)).collect();
        if !num_str.is_empty() {
            nums.push(num_str.parse::<i64>().unwrap());
        }
    }

    Problem { op, nums }
}

#[aoc_generator(day6, part1)]
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
pub fn part2(input: &Vec<Problem>) -> OutputType {
    input.iter().map(|p| p.execute()).sum()
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
        assert_eq!(part2(&day6_parse_part2(get_test_input())), 3263827);
    }
}
