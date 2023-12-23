use std::io::{self, Read};
fn main() {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    solve(buf.as_str());
}

fn solve(buf: &str) {
    let mut inputs = buf.lines();

    let mut vec = vec![];
    let tcnt = inputs.next().unwrap().trim().parse::<i64>().unwrap();

    for _ in 0..tcnt {
        vec.clear();

        let n = inputs.next().unwrap().trim().parse::<i64>().unwrap();
        for _ in 0..n {
            let (x, y) = {
                let mut iter = inputs.next().unwrap().trim().split(' ');
                (
                    iter.next().unwrap().trim().parse::<i64>().unwrap(),
                    iter.next().unwrap().trim().parse::<i64>().unwrap(),
                )
            };
            vec.push((x, y));
        }

        let ans = dfs(&vec, 0, 0, 0, 0);
        println!("{}", f64::sqrt(ans as f64));
    }
}

fn dfs(vec: &Vec<(i64, i64)>, depth: u8, x: i64, y: i64, pcnt: u8) -> i64 {
    let len = vec.len();

    if depth as usize == len {
        return x * x + y * y;
    }

    let mut ret = i64::MAX;
    let (tx, ty) = vec[depth as usize];

    if ((pcnt as usize) << 1) < len {
        let temp = dfs(vec, depth + 1, x + tx, y + ty, pcnt + 1);
        if temp < ret {
            ret = temp;
        }
    }

    if (((depth - pcnt) << 1) as usize) < len {
        let temp = dfs(vec, depth + 1, x - tx, y - ty, pcnt);
        if temp < ret {
            ret = temp;
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use crate::dfs;
    #[test]
    fn test_input1() {
        let vec = vec![(5, 5), (5, -5), (-5, 5), (-5, -5)];
        let ans = dfs(&vec, 0, 0, 0, 0);
        let ans = f64::sqrt(ans as f64);
        assert!(f64::abs(ans - 0.0) <= 0.000001);
    }

    #[test]
    fn test_input2() {
        let vec = vec![(-100000, -100000), (100000, 100000)];
        let ans = dfs(&vec, 0, 0, 0, 0);
        let ans = f64::sqrt(ans as f64);
        assert!(f64::abs(ans - 282842.712474619038_f64) <= 0.000001);
    }

    #[test]
    fn test_input3() {
        let vec = vec![
            (26, -76),
            (65, -83),
            (78, 38),
            (92, 22),
            (-60, -42),
            (-27, 85),
            (42, 46),
            (-86, 98),
            (92, -47),
            (-41, 38),
        ];
        let ans = dfs(&vec, 0, 0, 0, 0);
        let ans = f64::sqrt(ans as f64);
        assert!(f64::abs(ans - 13.341664064126334) <= 0.000001);
    }
}
