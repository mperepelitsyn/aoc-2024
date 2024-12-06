fn search_p1(g: &[Vec<u8>], i: usize, j: usize) -> i32 {
    if g[i][j] != b'X' {
        return 0;
    }
    [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
    .iter()
    .filter(|dir| {
        let (mut ni, mut nj) = (i, j);
        for b in "MAS".bytes() {
            ni = ni.wrapping_add_signed(dir.0);
            nj = nj.wrapping_add_signed(dir.1);
            if g.get(ni).and_then(|r| r.get(nj)).unwrap_or(&0) != &b {
                return false;
            }
        }
        true
    })
    .count() as _
}

fn search_p2(g: &[Vec<u8>], i: usize, j: usize) -> i32 {
    if g[i][j] != b'A' {
        return 0;
    }
    let letters = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
        .into_iter()
        .map(|dir| {
            *g.get(i.wrapping_add_signed(dir.0))
                .and_then(|r| r.get(j.wrapping_add_signed(dir.1)))
                .unwrap_or(&0)
        })
        .collect::<Vec<_>>();
    ["MMSS", "SSMM", "MSMS", "SMSM"]
        .into_iter()
        .any(|pattern| pattern.as_bytes() == letters) as _
}

fn solve(input: &str, cb: fn(&[Vec<u8>], usize, usize) -> i32) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let mut ret = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            ret += cb(&grid, i, j);
        }
    }
    ret
}

fn main() {
    let input = std::fs::read_to_string("input/04.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", solve(&input, search_p1));
    println!("Part 2: {}", solve(&input, search_p2));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(solve(&input, search_p1), 18);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(solve(&input, search_p2), 9);
    }
}
