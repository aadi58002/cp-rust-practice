#[allow(unused_imports)]
use std::io::Write;
use std::{io::{self, stdin}, collections::HashMap};

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
    #[allow(unused_variables, unused_mut)]
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let _ = sc.next::<usize>();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let array: Vec<usize> = input.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();

    // Try 1
    // let mut count: HashMap<usize, Vec<usize>> = HashMap::new();
    // for (index, ele) in array.iter().enumerate() {
    //     count
    //         .entry(*ele)
    //         .and_modify(|position| position.push(index))
    //         .or_insert(vec![index]);
    // }
    // let mut sort_count: Vec<(usize, Vec<usize>)> = count.into_iter().collect();
    // sort_count.sort_by(|val1, val2| val1.0.cmp(&val2.0));
    // let mut ans: Vec<(usize, usize)> = Vec::new();
    // for (ele, positions) in sort_count {
    //     if positions.len() == 1 {
    //         ans.push((ele, 0));
    //     } else {
    //         let diff = positions[1] - positions[0];
    //         let mut print_norm = true;
    //         for i in 1..(positions.len() - 1) {
    //             if positions[i + 1] - positions[i] != diff {
    //                 print_norm = false;
    //             }
    //         }
    //         if print_norm {
    //             ans.push((ele, diff));
    //         }
    //     }
    // }
    // ans.sort_by(|(val1,_),(val2,_)| val1.cmp(val2));
    // if ans.len() != 0 {
    //     println!("{}", ans.len());
    //     for ele in ans{
    //         println!("{} {}",ele.0,ele.1);
    //     }
    // } else {
    //     println!("0");
    // }

    // try 2
    // let mut ans: Vec<(usize, usize, usize, bool)> = Vec::new();
    // #[allow(unused_variables, unused_mut)]
    // for (index, ele) in array.iter().enumerate() {
    //     if let Some(position) = ans.iter().position(|num| num.0 == *ele) {
    //         if ans[position].3{
    //             if ans[position].1 == 0 {
    //                 ans[position].1 = index - (ans[position].2 as usize);
    //                 ans[position].2 = index;
    //             } else {
    //                 if index - (ans[position].2 as usize) != ans[position].1 {
    //                     ans[position].3 = false;
    //                 } else {
    //                     ans[position].2 = index;
    //                 }
    //             }
    //         }
    //     } else {
    //         ans.push((*ele, 0, index, true));
    //     }
    // }
    // ans.sort_by(|(val1, _, _, _), (val2, _, _, _)| val1.cmp(val2));
    // if ans.len() != 0 {
    //     let mut count = 0;
    //     for ele in ans.iter(){
    //         if ele.3 {
    //            count += 1;
    //         }
    //     }
    //     println!("{count}");
    //     for ele in ans.iter(){
    //         if ele.3 {
    //             println!("{} {}", ele.0, ele.1);
    //         }
    //     }
    // } else {
    //     println!("0");
    // }

    // try 3
    let mut ans: HashMap<usize, (usize, usize, bool)> = HashMap::new();
    let mut count = 0;
    for (index, ele) in array.iter().enumerate() {
        if let Some(position) = ans.get_mut(ele) {
            if position.2 {
                if position.0 == 0 {
                    position.0 = index - (position.1 as usize);
                    position.1 = index;
                } else {
                    if index - (position.1 as usize) != position.0 {
                        count -= 1;
                        position.2 = false;
                    } else {
                        position.1 = index;
                    }
                }
            }
        } else {
            ans.insert(*ele, (0, index, true));
            count += 1;
        }
    }
    let mut sorted_arr: Vec<(&usize,&(usize,usize,bool))> = ans.iter().collect();
    sorted_arr.sort_by(|val1,val2| val1.0.cmp(val2.0));
    writeln!(out,"{count}").ok();
    for ele in sorted_arr.iter() {
        if ele.1.2 {
            writeln!(out,"{} {}", ele.0, ele.1.0).ok();
        }
    }

    // let mut btm: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    // for (i, v) in array.iter().enumerate() {
    //     btm.entry(*v).or_insert(Vec::new()).push(i);
    // }
    // let ans: Vec<(&usize, &Vec<usize>)> = btm
    //     .iter()
    //     .filter(|(_, idxs)| {
    //         idxs.iter()
    //             .zip(idxs.iter().skip(1))
    //             .all(|(a, b)| b - a == idxs[1] - idxs[0])
    //     })
    //     .collect();
    // writeln!(out,"{}", ans.len()).ok();
    // for ele in ans {
    //     let length = ele.1.len();
    //     if length == 1 {
    //         writeln!(out,"{} 0", ele.0).ok();
    //     } else {
    //         writeln!(out,"{} {}", ele.0, ele.1[1] - ele.1[0]).ok();
    //     }
    // }
}
