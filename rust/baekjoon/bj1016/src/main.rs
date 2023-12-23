fn main() {
    // 1. input from user
    let (min_n, max_n) = {
        let mut buf = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buf).unwrap();
        let inputs = buf.trim().split(' ');
        let mut iter = inputs.map(|x| x.parse::<u64>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };
    let cnt = solve(min_n, max_n);
    println!("{}", cnt);
}

fn solve(min_n: u64, max_n: u64) -> usize {
    // create array to store primes
    let sqrt_max = f64::sqrt(max_n as f64) as u64;
    let mut nums = {
        let mut vec = vec![true; (sqrt_max + 1) as usize];
        vec[0] = false;
        vec[1] = false;
        vec
    };

    // Sieve of Eratosthenes
    for i in 2..=sqrt_max {
        if !nums[i as usize] {
            continue;
        }
        let mut j = i + i;
        while j <= sqrt_max {
            nums[j as usize] = false;
            j += i;
        }
    }

    // extract only primes and multiply itself
    let mut primes = Vec::new();
    for i in 2..=sqrt_max {
        if nums[i as usize] {
            primes.push(i * i);
        }
    }

    // create for solution
    let mut nums = vec![true; (max_n - min_n + 1) as usize];

    // get solution
    for p in primes {
        let mut min_n_sqrt = (min_n / p) * p;
        if min_n_sqrt != min_n {
            min_n_sqrt += p;
        }
        while min_n_sqrt <= max_n {
            nums[min_n_sqrt as usize - min_n as usize] = false;
            min_n_sqrt += p;
        }
    }

    // count
    nums.iter().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn test_input1() {
        let cnt = solve(1, 10);
        assert_eq!(cnt, 7);
    }

    #[test]
    fn test_input2() {
        let cnt = solve(15, 15);
        assert_eq!(cnt, 1);
    }

    #[test]
    fn test_input3() {
        let cnt = solve(1, 1000);
        assert_eq!(cnt, 608);
    }
}
