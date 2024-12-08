fn solve(input: &str, mark_all: bool) -> i32 {
    let (mut grid, freqs) = parse_input(input);
    mark_grid(&mut grid, &freqs, mark_all);
    grid.into_iter().flatten().filter(|&c| c == b'#').count() as _
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<(i32, i32)>>) {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut freqs = vec![Vec::new(); 128];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != b'.' {
                freqs[grid[i][j] as usize].push((i as i32, j as i32));
            }
        }
    }
    (grid, freqs)
}

fn mark_grid(grid: &mut [Vec<u8>], freqs: &[Vec<(i32, i32)>], mark_all: bool) {
    for points in freqs {
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let (p1, p2) = (points[i], points[j]);
                let diff = (p2.0 - p1.0, p2.1 - p1.1);
                fill(grid, p1, -1, diff, mark_all);
                fill(grid, p2, 1, diff, mark_all);
            }
        }
    }
}

fn fill(grid: &mut [Vec<u8>], p: (i32, i32), op: i32, diff: (i32, i32), all: bool) {
    let (mut i, mut j) = if all {
        (p.0, p.1)
    } else {
        (p.0 + op * diff.0, p.1 + op * diff.1)
    };
    while i >= 0 && j >= 0 && i < grid.len() as i32 && j < grid[0].len() as i32 {
        grid[i as usize][j as usize] = b'#';
        i += op * diff.0;
        j += op * diff.1;
        if !all {
            break;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();
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
        let input = std::fs::read_to_string("examples/08.txt").unwrap();
        assert_eq!(solve(&input, false), 14);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/08.txt").unwrap();
        assert_eq!(solve(&input, true), 34);
    }
}
