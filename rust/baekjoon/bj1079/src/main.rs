use std::io::Read;

struct MapiaGame {
    mapia: i32,
    crime_values: Vec<i32>,
    crime_changes: Vec<Vec<i32>>,
    alived_citizen: i32,
    mapia_dead: bool,
}

impl MapiaGame {
    fn is_night(&self) -> bool {
        let cnt = if self.mapia_dead {
            self.alived_citizen
        } else {
            self.alived_citizen + 1
        };
        cnt & 1 == 0
    }

    fn does_mapia_win(&self) -> bool {
        self.mapia_dead && self.alived_citizen == 0
    }

    fn find_criminal(&self, alives: &[bool]) -> i32 {
        let mut max_val = i32::MIN;
        let mut ret = -1;
        for (i, val) in self.crime_values.iter().enumerate() {
            if alives[i] && *val > max_val {
                max_val = *val;
                ret = i as i32;
            }
        }
        ret
    }

    fn kill_player(&mut self, n: i32, alives: &mut [bool]) {
        if n == self.mapia {
            self.mapia_dead = true;
        } else if self.alived_citizen > 0 {
            self.alived_citizen -= 1;
        }
        alives[n as usize] = false;
    }

    fn revive_player(&mut self, n: i32, alives: &mut [bool]) {
        if n == self.mapia {
            self.mapia_dead = false;
        } else {
            self.alived_citizen += 1;
        }
        alives[n as usize] = true;
    }

    fn increase_crime_values(&mut self, dead_num: i32, alives: &[bool]) {
        self.crime_values
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| *i as i32 != dead_num && alives[*i])
            .for_each(|(i, x)| {
                *x += self.crime_changes[dead_num as usize][i];
            });
    }

    fn recover_crime_values(&mut self, dead_num: i32, alives: &[bool]) {
        self.crime_values
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| *i as i32 != dead_num && alives[*i])
            .for_each(|(i, x)| {
                *x -= self.crime_changes[dead_num as usize][i];
            });
    }

    fn is_mapia(&self, num: i32) -> bool {
        self.mapia == num
    }

    fn total(&self) -> usize {
        self.crime_values.len()
    }
}

fn parse_input(input: &str) -> MapiaGame {
    let mut inputs = input.trim().lines();

    let total = inputs.next().unwrap().trim().parse::<i32>().unwrap();

    let mut crime_values = vec![];
    inputs.next().unwrap().trim().split(' ').for_each(|x| {
        crime_values.push(x.trim().parse::<i32>().unwrap());
    });

    let mut crime_changes = vec![];
    for _ in 0..total {
        crime_changes.push(vec![]);
        let row = crime_changes.last_mut().unwrap();
        inputs.next().unwrap().trim().split(' ').for_each(|x| {
            row.push(x.trim().parse::<i32>().unwrap());
        });
    }

    let mapia = inputs.next().unwrap().trim().parse::<i32>().unwrap();

    MapiaGame {
        mapia,
        crime_values,
        crime_changes,
        alived_citizen: total - 1,
        mapia_dead: false,
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut mapia_game = parse_input(&buf);

    let ans = play(&mut mapia_game);

    println!("{}", ans);
}

fn play(mg: &mut MapiaGame) -> i32 {
    let n = mg.total();
    let mut alives = vec![];
    alives.resize(n, true);
    _play(mg, &mut alives, 0)
}

fn _play(mg: &mut MapiaGame, alives: &mut [bool], night_count: i32) -> i32 {
    if mg.does_mapia_win() {
        return night_count;
    }

    let people_cnt = mg.total();
    let mut ret = night_count;

    if !mg.is_night() {
        // day
        let dead_num = mg.find_criminal(alives);

        if mg.is_mapia(dead_num) {
            return night_count;
        }

        mg.kill_player(dead_num, alives);

        let temp = _play(mg, alives, night_count);

        mg.revive_player(dead_num, alives);

        if temp > ret {
            ret = temp;
        }
    } else {
        // night
        for dead_num in 0..people_cnt {
            if mg.is_mapia(dead_num as i32) || !alives[dead_num] {
                continue;
            }

            mg.kill_player(dead_num as i32, alives);
            mg.increase_crime_values(dead_num as i32, alives);

            let temp = _play(mg, alives, night_count + 1);
            if temp as usize == alives.len() / 2 {
                return temp;
            } else if temp > ret {
                ret = temp;
            }

            mg.recover_crime_values(dead_num as i32, alives);
            mg.revive_player(dead_num as i32, alives);
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_input1() {
        let mut input = String::new();
        input.push_str("4\n");
        input.push_str("500 500 500 500\n");
        input.push_str("1 4 3 -2\n");
        input.push_str("-2 1 4 3\n");
        input.push_str("3 -2 1 4\n");
        input.push_str("4 3 -2 1\n");
        input.push_str("1\n");
        let mut mg = parse_input(input.as_str());
        let ans = play(&mut mg);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_input2() {
        let mut input = String::new();
        input.push_str("5\n");
        input.push_str("500 500 500 500 501\n");
        input.push_str("1 4 3 -2 5\n");
        input.push_str("-2 1 4 3 5\n");
        input.push_str("3 -2 1 4 5\n");
        input.push_str("4 3 -2 1 5\n");
        input.push_str("5 4 3 -2 1\n");
        input.push_str("1\n");
        let mut mg = parse_input(input.as_str());
        let ans = play(&mut mg);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_input3() {
        let mut input = String::new();
        input.push_str("6\n");
        input.push_str("500 500 500 500 500 500\n");
        input.push_str("-3 -3 -3 -3 -3 -3\n");
        input.push_str("2 2 2 2 2 2\n");
        input.push_str("-4 -4 -4 -4 -4 -4\n");
        input.push_str("6 6 6 6 6 6\n");
        input.push_str("7 7 7 7 7 7\n");
        input.push_str("-8 -8 -8 -8 -8 -8\n");
        input.push_str("0\n");
        let mut mg = parse_input(input.as_str());
        let ans = play(&mut mg);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test_input4() {
        let mut input = String::new();
        input.push_str("4\n");
        input.push_str("501 499 499 499\n");
        input.push_str("1 2 3 4\n");
        input.push_str("-26 -6 1 -7\n");
        input.push_str("5 19 1 19\n");
        input.push_str("-1 -20 -19 -13\n");
        input.push_str("0\n");
        let mut mg = parse_input(input.as_str());
        let ans = play(&mut mg);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_input5() {
        let mut input = String::new();
        input.push_str("1\n");
        input.push_str("800\n");
        input.push_str("5\n");
        input.push_str("0\n");
        let mut mg = parse_input(input.as_str());
        let ans = play(&mut mg);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_input6() {
        let mut input = String::new();
        input.push_str("4\n");
        input.push_str("500 500 500 500\n");
        input.push_str("1 1 1 1\n");
        input.push_str("1 1 -20 -20\n");
        input.push_str("2 1 20 20\n");
        input.push_str("3 1 20 20\n");
        input.push_str("0\n");
        let mut mg = parse_input(input.as_str());
        let ans = play(&mut mg);
        assert_eq!(ans, 2);
    }
}
