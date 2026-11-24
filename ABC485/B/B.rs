// https://atcoder.jp/contests/abc485/tasks/abc485_b
// https://github.com/Maji3429/atcoder-log
// ref: https://stackoverflow.com/questions/47829646/how-do-i-convert-a-string-to-a-list-of-chars
// ref: https://qiita.com/Yuki_Oshima/items/cc6c663a39b797b6bf24

use std::io::{self, Read};

fn main() {
  let mut std = String::new();
  io::stdin().read_to_string(&mut std).unwrap();
  let mut input = std.split_whitespace();
  
  let n: usize = input.next().unwrap().parse().unwrap();
  let m: usize = input.next().unwrap().parse().unwrap();
  // let mut square: Vec<Vec<String>> = Vec::new();
  // make sample of one line "o"
  let mut sample: String = "".to_string();
  for _ in 0..m {
      sample.push_str("x");
  }
  // assert_eq!("xxxx", sample);

  for j in 0..n {
      let target: &str = input.next().unwrap();
      if target==sample {
          println!("Faucet {}", j+1);
          std::process::exit(0);
      } else {
          let chars: Vec<char> = target.chars().collect();
          for i in 0..m {
              if chars[i]=='x' {
                  println!("Drain {}", i+1);
                  std::process::exit(0);
              }
          }
      }
  }
}
