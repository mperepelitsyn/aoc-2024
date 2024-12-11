type Compactor = fn(&mut [i32], Vec<(i32, usize)>, Vec<(i32, usize)>);

fn solve(input: &str, compactor: Compactor) -> i64 {
    let (mut disk, files, spaces) = parse_input(input);
    compactor(&mut disk, files, spaces);
    disk.into_iter()
        .enumerate()
        .filter(|t| t.1 >= 0)
        .fold(0, |acc, (i, id)| acc + i as i64 * id as i64)
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<i32>, Vec<(i32, usize)>, Vec<(i32, usize)>) {
    let mut disk = Vec::new();
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    let mut is_space = false;
    let mut id = 0;
    for len in input.trim().bytes().map(|b| (b - b'0') as i32) {
        if is_space {
            if len > 0 {
                spaces.push((len, disk.len()));
                disk.extend(std::iter::repeat(-1).take(len as usize));
            }
        } else {
            if len > 0 {
                files.push((len, disk.len()));
                disk.extend(std::iter::repeat(id).take(len as usize));
            }
            id += 1;
        }
        is_space = !is_space;
    }
    (disk, files, spaces)
}

fn compact_partial(disk: &mut [i32], files: Vec<(i32, usize)>, spaces: Vec<(i32, usize)>) {
    let mut spaces = spaces.into_iter().peekable();
    let mut files = files.into_iter().rev().peekable();
    while spaces.peek().is_some() && files.peek().is_some() {
        let (f_len, f_start) = files.peek_mut().unwrap();
        let (s_len, s_start) = spaces.peek_mut().unwrap();
        if f_start < s_start {
            break;
        }

        let len = (*s_len).min(*f_len);
        move_blocks(
            disk,
            (s_len, s_start),
            (len, *f_start + (*f_len - len) as usize),
        );

        *f_len -= len;
        if *f_len == 0 {
            files.next();
        }
        if *s_len == 0 {
            spaces.next();
        }
    }
}

fn compact_full(disk: &mut [i32], files: Vec<(i32, usize)>, mut spaces: Vec<(i32, usize)>) {
    for (f_len, f_start) in files.into_iter().rev() {
        if let Some((s_len, s_start)) = spaces
            .iter_mut()
            .take_while(|s| s.1 < f_start)
            .filter(|s| s.0 >= f_len)
            .take(1)
            .next()
        {
            move_blocks(disk, (s_len, s_start), (f_len, f_start));
        }
    }
}

fn move_blocks(
    disk: &mut [i32],
    (s_len, s_start): (&mut i32, &mut usize),
    (f_len, f_start): (i32, usize),
) {
    let id = disk[f_start];
    disk.iter_mut()
        .skip(*s_start)
        .take(f_len as usize)
        .for_each(|b| *b = id);
    disk.iter_mut()
        .skip(f_start)
        .take(f_len as usize)
        .for_each(|b| *b = -1);
    *s_len -= f_len;
    *s_start += f_len as usize;
}

fn main() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", solve(&input, compact_partial));
    println!("Part 2: {}", solve(&input, compact_full));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(solve(&input, compact_partial), 1928);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(solve(&input, compact_full), 2858);
    }
}
