use regex::Regex;

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).fold(0, |acc, c| {
        acc + c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap()
    })
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .fold((0, 1), |(acc, enabled), c| {
            if c[0].len() == 4 {
                (acc, 1)
            } else if c[0].len() == 7 {
                (acc, 0)
            } else {
                (
                    acc + enabled * c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap(),
                    enabled,
                )
            }
        })
        .0
}

fn main() {
    let input = std::fs::read_to_string("input/03.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/03-1.txt").unwrap();
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/03-2.txt").unwrap();
        assert_eq!(part2(&input), 48);
    }
}
