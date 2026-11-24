// https://atcoder.jp/contests/abc485/tasks/abc485_c
// https://github.com/Maji3429/atcoder-log
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut std = String::new();
    io::stdin().read_to_string(&mut std).unwrap();
    let mut input = std.split_whitespace();
    let mut out = BufWriter::new(io::stdout().lock());
    let n: usize = input.next().unwrap().parse().unwrap();
    let mut colors: Vec<usize> = Vec::new();
    // colors.push(10);
    dbg!(&n);
    dbg!(&colors);
    for _ in 0..n {
        colors.push(input.next().unwrap().parse().unwrap());
    }
    for i in 0..n {
        dbg!(&i);
        let mut left = 1;
        dbg!(&left);
        dbg!(&colors[i]);
        loop{
            if i+1-left == 0 {left = usize::MAX; break;} 
            if colors[i]!=colors[i-left] {break;}
            left += 1;
        }
        let mut right = 1;
        loop {
            if i+right == colors.len() {right = usize::MAX; break;}
            if colors[i]!=colors[i+right] {break;}
            right += 1;
        }
        let ans = right.min(left);
        writeln!(out, "{}", ans).unwrap();
    }
}
