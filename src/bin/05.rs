static MAX_NUM: usize = 100;

fn part1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    updates
        .filter(|update| is_valid(update, &rules))
        .map(|update| update[update.len() / 2] as i32)
        .sum()
}

fn part2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    updates
        .filter(|update| !is_valid(update, &rules))
        .map(|mut update| {
            fixup(&mut update, &rules);
            update[update.len() / 2] as i32
        })
        .sum()
}

fn parse_input(input: &str) -> (Vec<Vec<bool>>, impl Iterator<Item = Vec<usize>> + '_) {
    let (p1, p2) = input.split_once("\n\n").unwrap();

    let mut rules = vec![vec![false; MAX_NUM]; MAX_NUM];
    p1.lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|n| (n.0.parse::<usize>().unwrap(), n.1.parse::<usize>().unwrap()))
        .for_each(|n| rules[n.0][n.1] = true);
    let updates = p2.lines().map(|line| {
        line.split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });
    (rules, updates)
}

fn is_valid(update: &[usize], rules: &[Vec<bool>]) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if !rules[update[i]][update[j]] && rules[update[j]][update[i]] {
                return false;
            }
        }
    }
    true
}

fn fixup(update: &mut [usize], rules: &[Vec<bool>]) {
    'begin: loop {
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                if !rules[update[i]][update[j]] && rules[update[j]][update[i]] {
                    update.swap(i, j);
                    continue 'begin;
                }
            }
        }
        break;
    }
}

fn main() {
    let input = std::fs::read_to_string("input/05.txt").unwrap();
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
        let input = std::fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(part1(&input), 143);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(part2(&input), 123);
    }
}
