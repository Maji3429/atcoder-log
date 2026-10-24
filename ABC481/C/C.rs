// https://atcoder.jp/contests/abc481/tasks/abc481_c
use std::io::{self, Read};

fn main() {
  let mut std = String::new();
  io::stdin().read_to_string(&mut std).unwrap();
  let mut input = std.split_whitespace();
  
  let q: usize = input.next().unwrap().parse().unwrap();

  let query_q: Vec<&str> = (0..q)
    .map(|_| input.next().unwrap())
    .collect();
  
  let mut accounts: Vec<String> = Vec::new();
  accounts.push(" ".to_string());
  
  for s in query_q {
    // search weather account exists
    let found_id: Vec<String> = accounts.iter().filter(|name| s==*name);
    if let Some(s) = found_id[0] {
      println!("{}", found_id[0]);
    } else {
      accounts.push(s.to_string());
      println!("{}", accounts.len());
    }
  }
}