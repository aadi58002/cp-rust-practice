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
fn next_permutation(nums: &mut Vec<usize>) -> bool {
    if nums.len() < 2 {
        return false;
    }
    let mut i = nums.len() - 1;
    while i > 0 && nums[i - 1] >= nums[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = nums.len() - 1;
    while j >= i && nums[j] <= nums[i - 1] {
        j -= 1;
    }
    nums.swap(j, i - 1);
    nums[i..].reverse();
    true
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
    for _ in 0..size {
        let input = sc.next::<usize>();
        let mut steps = sc.vec::<usize>(input);
        steps.sort();
        if steps.len() >= 3 {
            println!(
                "{}",
                (steps[steps.len() - 1].min(steps[steps.len() - 2]) - 1).min(steps.len() - 2)
            );
        } else {
            println!("0");
        }
    }
}
