use std::io::Read;
fn main() {
    let mut buf = String::new();
    let mut inputs = {
        let _ = std::io::stdin().lock().read_to_string(&mut buf);
        buf.trim().lines()
    };

    let n = inputs.next().unwrap().trim().parse::<u64>().unwrap();
    let mut vec = vec![];
    vec.resize(n as usize, Vec::new());
    for _ in 1..n {
        let (ma, mb, mut p, mut q) = {
            let mut line = inputs.next().unwrap().trim().split(' ');
            (
                line.next().unwrap().parse::<u64>().unwrap(),
                line.next().unwrap().parse::<u64>().unwrap(),
                line.next().unwrap().parse::<u64>().unwrap(),
                line.next().unwrap().trim().parse::<u64>().unwrap(),
            )
        };

        let pq_gcd = gcd(p, q);
        p /= pq_gcd;
        q /= pq_gcd;
        vec[ma as usize].push((mb, p, q));
        vec[mb as usize].push((ma, q, p));
    }

    let ans = solve(&vec, n);

    ans.iter().for_each(|x| {
        print!("{} ", *x);
    });
    println!();
}

fn solve(vec: &Vec<Vec<(u64, u64, u64)>>, n: u64) -> Vec<u64> {
    let mut m0 = 1;
    let mut ratios_with_0 = vec![];
    ratios_with_0.resize(n as usize, (1, 1)); // (M0 : Mi)
    for i in 1..n {
        let (p, q) = dfs(vec, 0, i, 1, 1).unwrap();
        ratios_with_0[i as usize] = (p, q);
        m0 = m0 * p / gcd(p, m0);
    }

    let mut gcd_val = m0;
    let mut ans = vec![];
    ratios_with_0.iter().for_each(|(p, q)| {
        let temp = m0 * q / p;
        gcd_val = gcd(temp, gcd_val);
        ans.push(temp);
    });

    ans.iter_mut().for_each(|x| {
        *x /= gcd_val;
    });

    ans
}

fn dfs(
    vec: &Vec<Vec<(u64, u64, u64)>>,
    current: u64,
    target: u64,
    prev_p: u64,
    prev_q: u64,
) -> Option<(u64, u64)> {
    if target == current {
        return Some((1, 1));
    }
    let mut visit = Vec::new();
    visit.resize(vec.len(), false);
    visit[current as usize] = true;
    _dfs(vec, current, target, prev_p, prev_q, &mut visit)
}

fn _dfs(
    vec: &Vec<Vec<(u64, u64, u64)>>,
    current: u64,
    target: u64,
    prev_p: u64,
    prev_q: u64,
    visit: &mut Vec<bool>,
) -> Option<(u64, u64)> {
    if current == target {
        return Some((prev_p, prev_q));
    }

    for (m, p, q) in &vec[current as usize] {
        if visit[*m as usize] {
            continue;
        }

        let gcd_pq = gcd(prev_p * *p, prev_q * *q);
        visit[*m as usize] = true;
        if let Some(pq) = _dfs(
            vec,
            *m,
            target,
            prev_p * *p / gcd_pq,
            prev_q * *q / gcd_pq,
            visit,
        ) {
            return Some(pq);
        }
        visit[*m as usize] = false;
    }

    None
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod test {
    use crate::*;

    fn get_input_vec(vec: &Vec<Vec<u64>>) -> Vec<Vec<(u64, u64, u64)>> {
        let mut ret = vec![];
        ret.resize(vec.len() + 1, vec![]);
        for line in vec {
            ret[line[0] as usize].push((line[1], line[2], line[3]));
            ret[line[1] as usize].push((line[0], line[3], line[2]));
        }
        ret
    }

    #[test]
    fn test_gcd_1820_819_91() {
        assert_eq!(gcd(1820, 819), 91);
    }

    #[test]
    fn test_input1() {
        let vec: Vec<Vec<u64>> = vec![
            vec![4, 0, 1, 1],
            vec![4, 1, 3, 1],
            vec![4, 2, 5, 1],
            vec![4, 3, 7, 1],
        ];
        let vec = get_input_vec(&vec);
        let real_ans: Vec<u64> = vec![105, 35, 21, 15, 105];
        let ans = solve(&vec, 5);
        assert_eq!(real_ans, ans);
    }

    #[test]
    fn test_input2() {
        let vec: Vec<Vec<u64>> = vec![vec![0, 1, 6, 4]];
        let vec = get_input_vec(&vec);
        let real_ans: Vec<u64> = vec![3, 2];
        let ans = solve(&vec, 2);
        assert_eq!(real_ans, ans);
    }

    #[test]
    fn test_input3() {
        let vec: Vec<Vec<u64>> = vec![vec![0, 1, 9, 8], vec![1, 2, 9, 8]];
        let vec = get_input_vec(&vec);
        let real_ans: Vec<u64> = vec![81, 72, 64];
        let ans = solve(&vec, 3);
        assert_eq!(real_ans, ans);
    }

    #[test]
    fn test_input4() {
        let vec: Vec<Vec<u64>> = vec![vec![2, 3, 6, 8], vec![0, 1, 9, 3], vec![3, 0, 7, 5]];
        let vec = get_input_vec(&vec);
        let real_ans: Vec<u64> = vec![60, 20, 63, 84];
        let ans = solve(&vec, 4);
        assert_eq!(real_ans, ans);
    }
}
