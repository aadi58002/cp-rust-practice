use std::io;
#[allow(unused_imports)]
use std::io::Write;

/**************************************************

START OF TEMPLATE CODE

 *************************************************/
#[allow(dead_code)]
enum InputSource {
    Stdin,
    FromFile(Vec<String>),
}

struct Scanner {
    buffer: Vec<String>,
    input_source: InputSource,
}

#[allow(dead_code)]
fn permute(nums: &mut Vec<usize>) -> Vec<Vec<usize>> {
    if nums.len() == 1 {
        return vec![nums.clone()];
    } else {
        let mut permutation = Vec::new();
        for i in 0..nums.len() {
            let mut remaining = nums[..i].to_vec();
            remaining.extend_from_slice(&nums[i + 1..]);
            let perms = permute(&mut remaining);
            for mut perm in perms {
                perm.insert(0, nums[i]);
                permutation.push(perm);
            }
        }
        return permutation;
    }
}

impl Scanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            buffer: vec![],
            input_source: InputSource::Stdin,
        }
    }
    #[allow(dead_code)]
    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

    fn parse_next_line(&mut self) -> bool {
        let mut input = String::new();
        match &mut self.input_source {
            InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            InputSource::FromFile(lines) => match lines.pop() {
                Some(line) => input = line,
                None => return false,
            },
        }

        self.buffer = input.split_whitespace().rev().map(String::from).collect();
        return true;
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }

            self.parse_next_line();
        }
    }

    #[allow(dead_code)]
    fn has_more_elements(&mut self) -> bool {
        loop {
            if !self.buffer.is_empty() {
                return true;
            }
            if !self.parse_next_line() {
                return false;
            }
        }
    }

    #[allow(dead_code)]
    fn string(&mut self) -> String {
        self.next::<String>()
    }
}

/**************************************************

END OF TEMPLATE CODE

 *************************************************/


fn main() {
    let stdout = io::stdout();
    #[allow(unused_variables, unused_mut)]
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let size = sc.next::<usize>();
    for _ in 0..size{
        let input = sc.next::<usize>();
        let health = sc.vec::<usize>(input);
        let mut count_1 = 0;
        let mut count_else = 0;
        for ele in health{
            if ele == 1{
                count_1 += 1;
            }else{
                count_else += 1;
            }
        }
        println!("{}",count_1/2 + count_1%2 +count_else);
    }
}
