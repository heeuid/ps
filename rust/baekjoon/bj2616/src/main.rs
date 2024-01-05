use std::io::Read;

struct Problem {
    train: Vec<u8>,
    sub_train_room_cnt: u16,
    acc: Vec<u32>,
    dp: Vec<Vec<u32>>, // dp[i][j]: maximum passengers (0~j th room and (i+1)th subtrains)
}

impl Problem {
    fn parse_input(buf: &str) -> Problem {
        let mut iter = buf.trim().lines();
        let _ = iter.next();
        let mut train = vec![];
        let mut acc = vec![];
        acc.push(0);
        iter.next().unwrap().split(' ').for_each(|x| {
            let num = x.parse::<u8>().unwrap();
            train.push(num);
            let prev = acc.last().unwrap();
            acc.push(num as u32 + prev);
           
        });
        let sub_train_room_cnt = iter.next().unwrap().parse::<u16>().unwrap();
        let dp = vec![
            vec![0; train.len() + 1],
            vec![0; train.len() + 1],
            vec![0; train.len() + 1],
        ];
        Problem {
            train,
            sub_train_room_cnt,
            acc,
            dp,
        }
    }

    fn get_range_sum(&self, s: usize, len: usize) -> u32 {
        self.acc[s + len - 1] - self.acc[s - 1]
    }

    fn solve_dp_one_subtrain(&mut self) {
        let length = self.sub_train_room_cnt;
        self.dp[0][length as usize] = self.get_range_sum(1, length as usize);
        for i in (length as usize + 1)..=self.train.len() {
            self.dp[0][i] = u32::max(
                self.dp[0][i - 1],
                self.get_range_sum(i - length as usize + 1, length as usize),
            );
        }
    }

    fn solve_dp_two_subtrains(&mut self) {
        let length = self.sub_train_room_cnt;
        self.dp[1][(length + length) as usize] = self.get_range_sum(1, (length + length) as usize);
        for i in ((length + length) as usize + 1)..=self.train.len() {
            self.dp[1][i] = u32::max(
                self.dp[1][i - 1],
                self.dp[0][i - length as usize] + self.get_range_sum(i - length as usize + 1, length as usize)
            );
        }
    }

    fn solve_dp_three_subtrains(&mut self) {
        let length = self.sub_train_room_cnt;
        self.dp[2][(length + length + length) as usize] = self.get_range_sum(1, (length + length + length) as usize);
        for i in ((length + length + length) as usize + 1)..=self.train.len() {
            self.dp[2][i] = u32::max(
                self.dp[2][i - 1],
                self.dp[1][i - length as usize] + self.get_range_sum(i - length as usize + 1, length as usize)
            );
        }
    }

    fn solve(&mut self) -> u32 {
        self.solve_dp_one_subtrain();
        self.solve_dp_two_subtrains();
        self.solve_dp_three_subtrains();
        *self.dp[2].last().unwrap()
    }
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    let mut problem = Problem::parse_input(buf.as_str());
    let ans = problem.solve();
    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_parse_input() {
        let mut buf = String::new();
        buf.push_str("7\n");
        buf.push_str("35 40 50 10 30 45 60\n");
        buf.push_str("2\n");
        let problem = Problem::parse_input(buf.as_str());
        assert_eq!(problem.train, vec![35, 40, 50, 10, 30, 45, 60]);
        assert_eq!(problem.sub_train_room_cnt, 2);
        assert_eq!(problem.acc, vec![35, 75, 125, 135, 165, 210, 270]);
    }

    #[test]
    fn test_dp_one() {
        let mut buf = String::new();
        buf.push_str("7\n");
        buf.push_str("35 40 50 10 30 45 60\n");
        buf.push_str("2\n");
        let mut problem = Problem::parse_input(buf.as_str());
        problem.solve_dp_one_subtrain();
        assert_eq!(problem.dp[0], vec![0, 0, 75, 90, 90, 90, 90, 105]);
    }

    #[test]
    fn test_dp_two() {
        let mut buf = String::new();
        buf.push_str("7\n");
        buf.push_str("35 40 50 10 30 45 60\n");
        buf.push_str("2\n");
        let mut problem = Problem::parse_input(buf.as_str());
        problem.solve_dp_one_subtrain();
        problem.solve_dp_two_subtrains();
        assert_eq!(problem.dp[1], vec![0, 0, 0, 0, 135, 135, 165, 195]);
    }

    #[test]
    fn test_dp_three() {
        let mut buf = String::new();
        buf.push_str("7\n");
        buf.push_str("35 40 50 10 30 45 60\n");
        buf.push_str("2\n");
        let mut problem = Problem::parse_input(buf.as_str());
        problem.solve_dp_one_subtrain();
        problem.solve_dp_two_subtrains();
        problem.solve_dp_three_subtrains();
        assert_eq!(problem.dp[2], vec![0, 0, 0, 0, 0, 0, 210, 240]);
    }

    #[test]
    fn test_input1() {
        let mut buf = String::new();
        buf.push_str("7\n");
        buf.push_str("35 40 50 10 30 45 60\n");
        buf.push_str("2\n");
        let mut problem = Problem::parse_input(buf.as_str());
        let ans = problem.solve();
        assert_eq!(ans, 240);
    }

    #[test]
    fn test_input2() {
        let mut buf = String::new();
        buf.push_str("9\n");
        buf.push_str("26 6 25 18 18 18 4 44 68\n");
        buf.push_str("1\n");
        let mut problem = Problem::parse_input(buf.as_str());
        let ans = problem.solve();
        assert_eq!(ans, 138);
    }
}
