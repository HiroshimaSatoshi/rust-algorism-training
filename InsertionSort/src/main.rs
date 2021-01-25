use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::*;
macro_rules! trace {
    ($e1: expr, $e2: expr) => {
        let mut line: String = "".to_string();
        for i in 0..$e2 {
            if i > 0 {
                println!(" ");
            }
            line = line + &$e1[i].to_string();
        }

        println!("{}", line);
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);

    let mut sec_line = "".to_string();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if index != 0 {
            sec_line = line;
            continue;
        }
    }

    let mut vect: Vec<i32> = sec_line
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for i in 1..vect.len() {
        let v = vect[i];
        let mut j = (i as isize) - 1 as isize;
        while j >= 0 && vect[j as usize] > v {
            vect[(j + 1) as usize] = vect[j as usize];
            j = j - 1;
            vect[(j + 1) as usize] = v;
        }
        trace!(vect, i);
    }
}
