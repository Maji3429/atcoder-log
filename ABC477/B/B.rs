// https://atcoder.jp/contests/abc477/tasks/abc477_b
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let d: usize = input.next().unwrap().parse().unwrap();
    let mut x: Vec<usize> = Vec::new();
    for _ in 0..n {
        let sx: usize = input.next().unwrap().parse().unwrap();
        x.push(sx);
    }

    let mut k: usize = 0;
    let mut p: Vec<usize> = Vec::new();
    for i in 0..x.len() {
        // 他の値に対してxとの差の絶対値をとり、その値がD以上であるかどうかを確認する
        let mut appr = 1;
        for j in 0..x.len() {
            print!("{}-{}={} | ", &x[i], &x[j], &x[i].abs_diff(x[j]));
            println!("{}", &i == &j);
            if &i != &j && &x[i].abs_diff(x[j]) < &d {
                appr = 0;
            }
        }
        println!("{}", appr);
        if appr == 1 {
            p.push(i);
            k += 1;
        }
    }

    let mut out = BufWriter::new(io::stdout());
    writeln!(out, "{}", k).unwrap();
    for i in &p {
        write!(out, "{} ", i + 1).unwrap();
    }
}


