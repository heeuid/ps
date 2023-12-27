use std::io::Read;

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    let mut board = parse_input(buf.as_str());
    let ans = dfs(&mut board);
    println!("{}", ans);
}

fn parse_input(input: &str) -> Vec<Vec<Option<i8>>> {
    let mut inputs = input.trim().lines();
    let rows = inputs
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .next()
        .unwrap()
        .trim()
        .parse::<i8>()
        .unwrap();
    let mut board = vec![];
    for r in 0..rows {
        board.push(vec![]);
        inputs.next().unwrap().trim().chars().for_each(|nstr| {
            board[r as usize].push(match nstr {
                'H' => None,
                nch => Some(nch as i8 - '0' as i8),
            });
        });
    }

    board
}

fn dfs(board: &mut [Vec<Option<i8>>]) -> i32 {
    let (mut visited, mut dp) = {
        let rows = board.len();
        let columns = board[0].len();
        let mut v = vec![];
        let mut d = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            row.resize(columns, None);
            d.push(row);
            let mut row = vec![];
            row.resize(columns, false);
            v.push(row);
        }
        (v, d)
    };
    _dfs(0, 0, 0, board, &mut visited, &mut dp)
}

fn _dfs(
    y: isize,
    x: isize,
    depth: i32,
    board: &[Vec<Option<i8>>],
    visited: &mut [Vec<bool>],
    dp: &mut [Vec<Option<i32>>],
) -> i32 {
    dp[y as usize][x as usize] = Some(depth);
    visited[y as usize][x as usize] = true;

    let rows = board.len();
    let cols = board[0].len();

    let mut ret = depth + 1;

    for d in 0..4 {
        let (ny, nx) = nextyx(y as usize, x as usize, d, board);
        if ny < 0 || nx < 0 || ny as usize >= rows || nx as usize >= cols {
            continue;
        }
        if board[ny as usize][nx as usize].is_none() {
            continue;
        }
        if visited[ny as usize][nx as usize] {
            return -1;
        }

        let mem_val = dp[ny as usize][nx as usize];
        if mem_val.is_some_and(|n| n < 0) {
            return -1;
        } else if !mem_val.is_some_and(|n| n > depth) {
            let temp = _dfs(ny, nx, depth + 1, board, visited, dp);
            if temp < 0 {
                return -1;
            }
            if temp > ret {
                ret = temp;
            }
        }
    }

    visited[y as usize][x as usize] = false;

    ret
}

fn nextyx(y: usize, x: usize, d: u8, board: &[Vec<Option<i8>>]) -> (isize, isize) {
    let num = board[y][x].unwrap();
    match d {
        0 => (y as isize + num as isize, x as isize),
        1 => (y as isize - num as isize, x as isize),
        2 => (y as isize, x as isize + num as isize),
        _ => (y as isize, x as isize - num as isize),
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_parse_input() {
        let mut input = String::new();
        input.push_str("3 7\n");
        input.push_str("H942178\n");
        input.push_str("123H567\n");
        input.push_str("91235H2\n");
        let board = parse_input(input.as_str());
        assert_eq!(
            board,
            vec![
                vec![None, Some(9), Some(4), Some(2), Some(1), Some(7), Some(8)],
                vec![Some(1), Some(2), Some(3), None, Some(5), Some(6), Some(7)],
                vec![Some(9), Some(1), Some(2), Some(3), Some(5), None, Some(2)],
            ]
        );
    }

    #[test]
    fn test_input1() {
        let mut input = String::new();
        input.push_str("3 7\n");
        input.push_str("3942178\n");
        input.push_str("1234567\n");
        input.push_str("9123532\n");
        let mut board = parse_input(input.as_str());
        let ans = dfs(&mut board);
        assert_eq!(ans, 5);
    }

    #[test]
    fn test_input2() {
        let mut input = String::new();
        input.push_str("1 10\n");
        input.push_str("2H3HH4HHH5\n");
        let mut board = parse_input(input.as_str());
        let ans = dfs(&mut board);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_input3() {
        let mut input = String::new();
        input.push_str("4 4\n");
        input.push_str("3994\n");
        input.push_str("9999\n");
        input.push_str("9999\n");
        input.push_str("2924\n");
        let mut board = parse_input(input.as_str());
        let ans = dfs(&mut board);
        assert_eq!(ans, -1);
    }

    #[test]
    fn test_input4() {
        let mut input = String::new();
        input.push_str("4 6\n");
        input.push_str("123456\n");
        input.push_str("234567\n");
        input.push_str("345678\n");
        input.push_str("456789\n");
        let mut board = parse_input(input.as_str());
        let ans = dfs(&mut board);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_input5() {
        let mut input = String::new();
        input.push_str("1 1\n");
        input.push_str("9\n");
        let mut board = parse_input(input.as_str());
        let ans = dfs(&mut board);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test_input6() {
        let mut input = String::new();
        input.push_str("3 7\n");
        input.push_str("2H9HH11\n");
        input.push_str("HHHHH11\n");
        input.push_str("9HHHH11\n");
        let mut board = parse_input(input.as_str());
        let ans = dfs(&mut board);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_input7() {
        let mut input = String::new();
        input.push_str("6 7\n");
        input.push_str("12HHHHH\n");
        input.push_str("2214HHH\n");
        input.push_str("H1HHHHH\n");
        input.push_str("H4H9H2H\n");
        input.push_str("HHHHHHH\n");
        input.push_str("HHH2HHH\n");
        let mut board = parse_input(input.as_str());
        let ans = dfs(&mut board);
        assert_eq!(ans, 7);
    }
}
