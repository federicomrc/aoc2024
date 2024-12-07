pub fn first() {
    let res = elaborate_first(get_input());
    println!("Day 02/1 {}", res);
}

pub fn second() {
    let res = elaborate_second(get_input());
    println!("Day 02/2 {}", res);
}

fn elaborate_first(data: Vec<String>) -> u32 {
    let mut res = 0;
    for row in data {
        let unpacked = row
            .split_whitespace()
            .map(|l| l.parse::<u32>().expect("Cannot convert into number"))
            .collect::<Vec<u32>>();

        if is_valid(&unpacked) {
            res += 1;
        }
    }

    res
}

fn is_valid(row: &[u32]) -> bool {
    let mut valid = true;
    let mut previous = Option::None;
    let mut desc = Option::None;
    for &level in row {
        if let Some(p) = previous {
            let diff = level.abs_diff(p);
            if diff < 1 || diff > 3 {
                valid = false;
                break;
            }

            if let Some(prev_desc) = desc {
                if prev_desc && level > p || !prev_desc && p > level {
                    valid = false;
                    break;
                }
            } else {
                desc = Option::Some(level < p);
            }
        }

        previous = Some(level);
    }
    valid
}

fn elaborate_second(data: Vec<String>) -> u32 {
    let mut res = 0;
    for row in data {
        let unpacked = row
            .split_whitespace()
            .map(|l| l.parse::<u32>().expect("Cannot convert into number"))
            .collect::<Vec<u32>>();

        for idx in 0..unpacked.len() {
            let new_vec = unpacked
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != idx)
                .map(|(_, &v)| v)
                .collect::<Vec<u32>>();

            if is_valid(&new_vec) {
                res += 1;
                break;
            }
        }
    }

    res
}

fn parse_input(input: &str) -> Vec<String> {
    let mut res = Vec::new();
    for line in input.lines() {
        res.push(line.to_owned());
    }

    return res;
}

fn get_input() -> Vec<String> {
    parse_input(&std::fs::read_to_string("./input/day02.txt").expect("Missing input file"))
}

#[cfg(test)]
mod tests {
    use crate::day02::{elaborate_first, elaborate_second, parse_input};

    #[test]
    fn first() {
        const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let res = elaborate_first(parse_input(INPUT));

        assert_eq!(2, res);
    }

    #[test]
    fn second() {
        const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let res = elaborate_second(parse_input(INPUT));
        assert_eq!(4, res);
    }
}
