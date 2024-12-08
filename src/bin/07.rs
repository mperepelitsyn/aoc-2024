fn solve(input: &str, enable_concat: bool) -> i64 {
    let mut ret = 0;
    for line in input.lines() {
        let (p1, p2) = line.split_once(": ").unwrap();
        let val = p1.parse::<i64>().unwrap();
        let operands = p2
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        if is_solvable(&operands[1..], operands[0], val, enable_concat) {
            ret += val
        }
    }
    ret
}

fn is_solvable(operands: &[i64], cur: i64, val: i64, enable_concat: bool) -> bool {
    if operands.is_empty() {
        return val == cur;
    }
    enable_concat && is_solvable(&operands[1..], concat(cur, operands[0]), val, enable_concat)
        || is_solvable(&operands[1..], cur + operands[0], val, enable_concat)
        || is_solvable(&operands[1..], cur * operands[0], val, enable_concat)
}

fn concat(mut a: i64, b: i64) -> i64 {
    let mut src = b;
    while src > 0 {
        src /= 10;
        a *= 10;
    }
    a + b
}

fn main() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();
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
        let input = std::fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(solve(&input, false), 3749);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(solve(&input, true), 11387);
    }
}
