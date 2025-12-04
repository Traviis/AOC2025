use std::collections::HashSet;

type InputType = HashSet<Coordinate>;
type OutputType = u64;

type Coordinate = (i64,i64);

#[aoc_generator(day4)]
fn day4_parse(input: &str) -> InputType {
    let mut map=  HashSet::new();
    for (y,line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == '@' {
                map.insert((x as i64,y as i64));
            }
        }
    }

    map
}

#[aoc(day4, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //There is probably a trick here... but, just brute force this?

    let max_x = input.iter().map(|(x,_)| *x).max().unwrap();
    let max_y = input.iter().map(|(_,y)| *y).max().unwrap();

    let mut res = 0;
    'map_move: for (cx,cy) in input.iter() {


            let mut nearby_rolls = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    println!("{},{}",dx,dy);
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if input.get(&(cx+dx,cy+dy)).is_some() {
                        nearby_rolls += 1;
                        if nearby_rolls > 3 {
                            continue 'map_move;
                        }
                    }
                }
            }

            // If we didn't error out, go ahead and add as a good roll
            res += 1;
    }

    res

}

#[aoc(day4, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
    }

    #[test]
    fn day4_part1() {
        assert_eq!(part1(&day4_parse(get_test_input())), 13);
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(&day4_parse(get_test_input())), 0);
    }
}
