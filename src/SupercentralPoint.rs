#[allow(unused_imports)]
use std::io::Write;
use std::io;

/**************************************************

START OF TEMPLATE CODE

 *************************************************/
#[allow(dead_code)]
#[allow(unused_macros)]
macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        eprint!("[{}:{}] {} = {:?}",
                file!(), line!(), stringify!($first_val), &$first_val);
        ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
        eprintln!();
    };
    ($first_val:expr) => {
        eprintln!("[{}:{}] {} = {:?}",
                  file!(), line!(), stringify!($first_val), &$first_val);
    };
}

#[allow(dead_code)]
enum InputSource {
    Stdin,
    FromFile(Vec<String>),
}

struct Scanner {
    buffer: Vec<String>,
    input_source: InputSource,
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
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let mut x_point: Vec<i32> = Vec::new();
    let mut y_point: Vec<i32> = Vec::new();
    let n = sc.next::<usize>();
    for _ in 0..n {
        x_point.push(sc.next::<i32>());
        y_point.push(sc.next::<i32>());
    }
    let mut count = 0;
    for i in 0..n {
        let (mut lx, mut gx, mut ly, mut gy) = (false, false, false, false);
        let x = x_point[i];
        let y= y_point[i];
        for j in 0..n{
            if x_point[j] == x{
                if y_point[j] > y{gy=true}
                if y_point[j] < y{ly=true}
            }
            if y_point[j] == y{
                if x_point[j] > x{gx=true}
                if x_point[j] < x{lx=true}
            }
        }
        if gx && gy && lx && ly{
            count += 1;
        }
    }
    writeln!(out,"{count}").ok();
}
