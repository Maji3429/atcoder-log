// https://atcoder.jp/contests/abc485/tasks/abc485_a
// https://github.com/Maji3429/atcoder-log

use std::io::{self, Read};

fn main() {
  let mut std = String::new();
  io::stdin().read_to_string(&mut std).unwrap();
  let mut input = std.split_whitespace();
  
  let a: isize = input.next().unwrap().parse().unwrap();
  let b: isize = input.next().unwrap().parse().unwrap();
  let is_positive: bool = (0 < &a*&b);
  if is_positive {
      println!("Yes");
  } else {
      println!("No");
  }
}
