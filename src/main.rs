extern crate regex;

use regex::Regex;
use std::env;

fn main() {
    //command args
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("{}", "Atleast two args required. First- number of input numbers, followed by list of input nos");
        std::process::exit(1);
    }

    //Arg1: input count
    let _: u64 = match args.get(1).unwrap().parse() {
        Ok(x) => x,
        Err(_x) => {
            eprintln!("First arg is not a positive number or exceeds bounds");
            std::process::exit(1);
        }
    };

    //Remove the next line
    if let Some(last) = args.last_mut() {
        *last = last.replace('\n', "");
    }

    //remove first 2 args to get only inputs
    args.remove(0);
    args.remove(0);

    //loop through the inputs and print the result
    for n in args.iter_mut() {
        check_and_exit(n);
        print!("{} ", find_next_pal(n));
    }
}

///Is positive integer
fn is_pos_int(num: &str) -> bool {
    let re = Regex::new(r"^[1-9]\d*$").unwrap();
    re.is_match(num)
}

///Check if given str is a positive int
/// if not exit the prog
fn check_and_exit(num: &str) {
    if !is_pos_int(num) {
        eprintln!("{} is not a positive integer", num);
        std::process::exit(1);
    }
}

///Find the smallest palindrom greater than the given number(str)
fn find_next_pal(num: &str) -> String {
    let len = num.len();

    let half_count = len / 2;

    if len == 1 {
        return num.to_string();
    }

    let (half1, m, half2) = split(num, len);

    let rev = half1.chars().rev().collect::<String>();

    if less_than(&rev, half2) {
        return match m {
            Some(x) => format!("{}{}{}", half1, x, rev),
            None => format!("{}{}", half1, rev),
        };
    }

    match m {
        Some(x) => {
            let first = format!("{}{}", half1, x);
            let first = add_one(&first);
            format!(
                "{}{}",
                first,
                first[..half_count].chars().rev().collect::<String>()
            )
        }
        None => {
            let first = add_one(half1);
            format!(
                "{}{}",
                first,
                first[..half_count].chars().rev().collect::<String>()
            )
        }
    }
}

fn split(num: &str, len: usize) -> (&str, Option<char>, &str) {
    let odd = len % 2 != 0;
    let half: usize = num.len() / 2;
    let half1 = &num[..half];
    let half2 = if odd { &num[half + 1..] } else { &num[half..] };

    let middle: Option<char> = if odd { num.chars().nth(half) } else { None };

    (half1, middle, half2)
}

///Check if str num2 is less than str num1
fn less_than(num1: &str, num2: &str) -> bool {
    if num1 == num2 {
        return false;
    }

    for (ns1, ns2) in num1.chars().zip(num2.chars()) {
        if ns2 > ns1 {
            return false;
        } else if ns2 < ns1 {
            return true;
        }
    }
    true
}

//add 1 to a num string
fn add_one(num: &str) -> String {
    let mut num_arr: Vec<u8> = num
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let mut carry: u8 = 0;
    let len = num_arr.len();
    for (j, n) in num_arr.iter_mut().rev().enumerate() {
        let i = len - j - 1;
        let sum = if j == 0 { *n + 1 + carry } else { *n + carry };
        if i == 0 || sum < 10 {
            *n = sum;
            return num_arr.iter().map(|c| c.to_string()).collect();
        } else {
            let ssum = sum.to_string();
            let cur_num = ssum.chars().last().unwrap().to_string().parse().unwrap();

            *n = cur_num;
            carry = 1;
        }
    }
    String::new()
}
