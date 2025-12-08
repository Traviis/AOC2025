use std::collections::HashSet;

type InputType = HashSet<Coordinate>;
type OutputType = u64;

type Coordinate = (i64, i64);

#[aoc_generator(day4)]
fn day4_parse(input: &str) -> InputType {
    let mut map = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                map.insert((x as i64, y as i64));
            }
        }
    }

    map
}

#[aoc(day4, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //There is probably a trick here... but, just brute force this?

    let mut res = 0;
    'map_move: for (cx, cy) in input.iter() {
        let mut nearby_rolls = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                //println!("{},{}",dx,dy);
                if dx == 0 && dy == 0 {
                    continue;
                }
                if input.get(&(cx + dx, cy + dy)).is_some() {
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
    let mut remove_round: HashSet<Coordinate> = HashSet::new();
    let mut found_this_round = true;
    let mut input = input.clone();
    let mut total_removed = 0;

    while found_this_round {
        // At the beginning, remove all the rolls that we found could be removed last round
        for (rx, ry) in remove_round.iter() {
            input.remove(&(*rx, *ry));
            total_removed += 1;
        }
        //println!("Removed {} rolls",remove_round.len());
        remove_round.clear();
        found_this_round = false;
        'map_move: for (cx, cy) in input.iter() {
            let mut nearby_rolls = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    //println!("{},{}",dx,dy);
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if input.get(&(cx + dx, cy + dy)).is_some() {
                        nearby_rolls += 1;
                        if nearby_rolls > 3 {
                            continue 'map_move;
                        }
                    }
                }
            }

            // This is a roll to remove
            remove_round.insert((*cx, *cy));
            found_this_round = true;
        }
    }

    total_removed as u64
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
        assert_eq!(part2(&day4_parse(get_test_input())), 43);
    }
}
