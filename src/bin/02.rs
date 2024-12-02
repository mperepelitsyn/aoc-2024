fn solve(input: &str, dampen: bool) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut levels = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if is_safe(&levels) {
                return 1;
            }
            if dampen {
                for i in 0..levels.len() {
                    let n = levels[i];
                    levels.remove(i);
                    if is_safe(&levels) {
                        return 1;
                    }
                    levels.insert(i, n);
                }
            }
            0
        })
        .sum()
}

fn is_safe(nums: &[i32]) -> bool {
    nums.windows(2)
        .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
        || nums
            .windows(2)
            .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", solve(&input, false));
    println!("Part 2: {}", solve(&input, true));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(solve(&input, false), 2);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(solve(&input, true), 4);
    }
}
