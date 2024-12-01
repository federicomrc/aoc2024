pub fn first() {
    let res = elaborate_first(get_input());
    println!("Day 01/1 {}", res);
}

pub fn second() {
    let res = elaborate_second(get_input());
    println!("Day 01/2 {}", res);
}

fn elaborate_first(data: (Vec<u32>, Vec<u32>)) -> u32 {
    let (mut f, mut s) = data;

    f.sort();
    s.sort();

    f.into_iter()
        .zip(s)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

fn elaborate_second(data: (Vec<u32>, Vec<u32>)) -> usize {
    let (f, s) = data;

    let s = &s;

    f.into_iter()
        .map(|n| s.iter().filter(|&&el| el == n).count() * (n as usize))
        .sum::<usize>()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        let halves: Vec<&str> = line.split_whitespace().collect();

        first_list.push(halves[0].parse().expect("First value is not a number"));
        second_list.push(halves[1].parse().expect("First value is not a number"));
    }
    (first_list, second_list)
}

fn get_input() -> (Vec<u32>, Vec<u32>) {
    parse_input(&std::fs::read_to_string("./input/day01.txt").expect("Missing input file"))
}

#[cfg(test)]
mod tests {
    use crate::day01::{elaborate_first, elaborate_second, parse_input};

    #[test]
    fn first() {
        const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

        let res = elaborate_first(parse_input(INPUT));

        assert_eq!(11, res);
    }

    #[test]
    fn second() {
        const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

        let res = elaborate_second(parse_input(INPUT));

        assert_eq!(31, res);
    }
}
