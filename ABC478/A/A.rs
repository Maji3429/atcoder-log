// https://atcoder.jp/contests/abc478/tasks/abc478_a
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let people_count: usize = input.next().unwrap().parse().unwrap();
    let grapes_count: usize = input.next().unwrap().parse().unwrap();

    let mut out = BufWriter::new(io::stdout().lock());
    
    let normal_amount: usize = grapes_count/people_count;
    let boanus_people: usize = grapes_count%people_count;
    let normal_people: usize = people_count-boanus_people;

    for _ in 0..boanus_people {
        writeln!(out, "{}", normal_amount+1).unwrap();
    }
    for _ in 0..normal_people {
        writeln!(out, "{}", normal_amount).unwrap();
    }
    
}
