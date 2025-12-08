use std::collections::{HashMap, HashSet};

type InputType = HashMap<(u64,u64),Item>;
type OutputType = u64;

#[derive(PartialEq,Clone)]
enum Item {
    Start,
    Splitter,
}

#[aoc_generator(day7)]
fn day7_parse(input: &str) -> InputType {
    //Do a simple sparse parse, Most of it is empty space, but we care about the start and the splitters
    // Assume a missing item is a blank
    input.lines().enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let pos = (x as u64, y as u64);
                match c {
                    '^' => Some((pos, Item::Splitter)),
                    'S' => Some((pos, Item::Start)),
                    _ => None,
                }
            })
        })
        .collect()
}


#[aoc(day7, part1)]
pub fn part1(input: &InputType) -> OutputType {
    // let's just do this simply by step
    let mut splits = 0;
    let max_y = input.iter().map(|((x,y),_)| *y).max().unwrap();

    let ((s_x,_),_) = input.iter().find(|((x,y),i)| **i == Item::Start).unwrap();

    let mut beams = HashSet::new();
    let mut new_beams = HashSet::new();
    beams.insert((*s_x,0));

    for c_y in 0..max_y {
        // We know which y we are at, let's see what the beam(s) do at this height
        for (beam_x,_) in beams.iter() {
            let beam_coor = (*beam_x, c_y);
            if let Some(loc) = input.get(&beam_coor) {
                match loc {
                    Item::Splitter => {
                        // Int this case, Split the beams to one on either side of the splitter (but also 1 down), using a hashset so I combine properly
                        new_beams.insert((*beam_x-1,c_y+1));
                        new_beams.insert((*beam_x+1,c_y+1));
                        splits += 1;
                    }
                    //air
                    _ => {
                        new_beams.insert((*beam_x,c_y+1));
                    }
                }
            }
        }
        beams = new_beams.clone();

    }

    splits



}

#[aoc(day7, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............

....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
    }

    #[test]
    fn day7_part1() {
        assert_eq!(part1(&day7_parse(get_test_input())), 21);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(&day7_parse(get_test_input())), 0);
    }
}
