// https://atcoder.jp/contests/abc477/tasks/abc477_b
// Rustの数値型は値をそのまま複製できる性質を持つ
// usize, i32, bool など、メモリ上でのデータ容量が予め決まっている型に関しては、所有権の話は関係なく、自動で複製が行われる。
use std::io::{self, BufWriter, Read, Write};
fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let d: usize = input.next().unwrap().parse().unwrap();
    
    // let x: Vec<usize> = (0..n)
    //     .map(|_| input.next().unwrap().parse().unwrap())
    //     .collect();
    let x: Vec<usize> = input
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    let p: Vec<usize> = x
        .iter()       // 配列の各値に対して
        .enumerate()  // (idx, value)をタプルで取り出す
        .filter(|&(i, &xi)| {
            // 他の全ての要素に対して、自分自身であるか、または差がD以上であるかを確認
            x.iter().enumerate()
                // それぞれの要素に対して真偽値を返す
                // 条件を満たさない要素が1つでも見つかったらすぐにfalseを返して
                // 処理を打ち切る
                .all(|(j, &xj)| i==j || xi.abs_diff(xj) >= d)
        })
        .map(|(i, _)| i+1) // 出力用に、1-indexへ変換
        .collect();
    
    let mut out = BufWriter::new(io::stdout().lock());
    writeln!(out, "{}", p.len()).unwrap();
    for i in &p {
        write!(out, "{} ", i).unwrap();
    }
    wirteln!(out).unwrap();
}
