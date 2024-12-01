fn part1(input: &str) -> i32 {
    let (mut list1, mut list2) = (Vec::new(), Vec::new());
    for (n1, n2) in parse_input(input) {
        list1.push(n1);
        list2.push(n2);
    }
    list1.sort_unstable();
    list2.sort_unstable();

    list1
        .into_iter()
        .zip(list2)
        .map(|(n1, n2)| n1.abs_diff(n2) as i32)
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut list1 = Vec::new();
    let mut list2 = std::collections::HashMap::new();
    for (n1, n2) in parse_input(input) {
        list1.push(n1);
        list2.entry(n2).and_modify(|v| *v += 1).or_insert(1);
    }
    list1
        .into_iter()
        .map(|n| n * list2.get(&n).unwrap_or(&0))
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input.lines().map(|line| {
        let (n1, n2) = line.split_once("   ").unwrap();
        (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap())
    })
}

fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();
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
        let input = std::fs::read_to_string("examples/01.txt").unwrap();
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/01.txt").unwrap();
        assert_eq!(part2(&input), 31);
    }
}
