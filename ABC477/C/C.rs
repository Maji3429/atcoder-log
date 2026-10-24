// https://atcoder.jp/contests/abc477/tasks/abc477_c
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();
    let mut out = BufWriter::new(io::stdout());

    let q: usize = input.next().unwrap().parse().unwrap();
    let s: String = input.next().unwrap().parse().unwrap();
    let t: String = input.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let l: usize = input.next().unwrap().parse().unwrap();
        let r: usize = input.next().unwrap().parse().unwrap();

        // println!("{:?}", s.get(l-1..=r-1).unwrap());
        if s.get(l-1..=r-1).unwrap().contains(&t) {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    }    
}
