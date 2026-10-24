// https://atcoder.jp/contests/abc481/tasks/abc481_a
use std::io::{self, Read};

fn main() {
  let mut std = String::new();
  io::stdin().read_to_string(&mut std).unwrap();
  let mut input = std.split_whitespace();
  
  let a: usize = input.next().unwrap().parse().unwrap();
  let b: usize = input.next().unwrap().parse().unwrap();
  let length = (a+b).to_string().len();
  println!("{}", length);
}

// https://atcoder.jp/contests/abc481/submissions/80309098