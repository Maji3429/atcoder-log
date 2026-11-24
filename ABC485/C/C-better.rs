// https://atcoder.jp/contests/abc485/tasks/abc485_c
// https://github.com/Maji3429/atcoder-log
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut std = String::new();
    io::stdin().read_to_string(&mut std).unwrap();
    let mut input = std.split_whitespace();
    let mut out = BufWriter::new(io::stdout().lock());
    let n: usize = input.next().unwrap().parse().unwrap();
    let mut colors: Vec<usize> = (0..n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();
    
    let mut l = vec![0usize; n];  // left用の、run を記録する配列
    let mut r = vec![0usize; n];  // right
    for i in 0..n {
        l[i] = if i > 0 && colors[i] == colors[i-1] { l[i-1] } else { i };  // iが左側末尾のindex=0に達していないかつ、入力配列のi番目がi-1番目と同じ色なら、left配列にもi-1番目をiに入れる。そうでなければ、i自体を入れる
    }
    for i in (0..n).rev() {  // .rev()で配列を反転して処理
        r[i] = if i+1 < n && colors[i] == colors[i+1] { r[i+1] } else { i };
    }

    for i in 0..n {
        let mut d = usize::MAX; // まず予めusize::MAXを用意しておいて、2段階で.min()を使う
        if l[i] != 0 { d = d.min(i-l[i]+1) };
        if r[i] != n-1 { d = d.min(r[i] +1 -i)};
        writeln!(out, "{}", d).unwrap();
    }
}
