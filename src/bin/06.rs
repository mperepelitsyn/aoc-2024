static DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> i32 {
    let (grid, pos, dir) = parse_input(input);
    visit(grid, pos.0, pos.1, dir)
}

fn part2(input: &str) -> i32 {
    let (grid, pos, dir) = parse_input(input);

    // Nice!
    let mut ret = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'#' || (pos.0 == i && pos.1 == j) {
                continue;
            }
            let mut copy = grid.clone();
            copy[i][j] = b'#';
            if is_loop(copy, pos.0, pos.1, dir) {
                ret += 1
            }
        }
    }
    ret
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (usize, usize), usize) {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'^' {
                return (grid, (i, j), 0);
            } else if grid[i][j] == b'>' {
                return (grid, (i, j), 1);
            } else if grid[i][j] == b'v' {
                return (grid, (i, j), 2);
            } else if grid[i][j] == b'<' {
                return (grid, (i, j), 3);
            }
        }
    }
    unreachable!()
}

fn visit(mut grid: Vec<Vec<u8>>, mut i: usize, mut j: usize, mut dir: usize) -> i32 {
    let mut visited = 0;
    loop {
        if grid[i][j] != b'x' {
            grid[i][j] = b'x';
            visited += 1;
        }

        let (ni, nj) = (
            i.wrapping_add_signed(DIRS[dir].0),
            j.wrapping_add_signed(DIRS[dir].1),
        );
        if ni >= grid.len() || nj >= grid[0].len() {
            break;
        } else if grid[ni][nj] == b'#' {
            dir = (dir + 1) % 4;
        } else {
            i = ni;
            j = nj;
        }
    }
    visited
}

fn is_loop(mut grid: Vec<Vec<u8>>, mut i: usize, mut j: usize, mut dir: usize) -> bool {
    loop {
        if grid[i][j] > 15 {
            grid[i][j] = 1 << dir;
        } else if grid[i][j] & (1 << dir) != 0 {
            return true;
        } else {
            grid[i][j] |= 1 << dir;
        }

        let (ni, nj) = (
            i.wrapping_add_signed(DIRS[dir].0),
            j.wrapping_add_signed(DIRS[dir].1),
        );
        if ni >= grid.len() || nj >= grid[0].len() {
            return false;
        } else if grid[ni][nj] == b'#' {
            dir = (dir + 1) % 4;
        } else {
            i = ni;
            j = nj;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
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
        let input = std::fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(part1(&input), 41);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(part2(&input), 6);
    }
}
