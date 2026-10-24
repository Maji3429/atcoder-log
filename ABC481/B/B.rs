// https://atcoder.jp/contests/abc481/tasks/abc481_b
use std::io::{self, Read};

fn main() {
  let mut std = String::new();
  io::stdin().read_to_string(&mut std).unwrap();
  let mut input = std.split_whitespace();
  
  let points_count: usize = input.next().unwrap().parse().unwrap();
  let k: usize = input.next().unwrap().parse().unwrap();
  
  let a_i: Vec<usize> = (0..points_count)
    .map(|_| input.next().unwrap().parse().unwrap())
    .collect();
  
  let mut over_count = 0;
  
  for i in 0..points_count-1 {
    let a = a_i[i];
    let b = a_i[i+1];
    
    if i==0  {
      if a==k {
        over_count += 1;
        continue;
      }
    }
    
    if (a<k && k<b)||(b<k && k<a)||(a<k && b==k)||(k<a && k==b) {
      over_count+=1;
    }
  }
  println!("{}", over_count);
}

// https://atcoder.jp/contests/abc481/submissions/80327081