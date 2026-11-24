// https://atcoder.jp/contests/abc485/tasks/abc485_d
// https://github.com/Maji3429/atcoder-log
use std::io::{self, Read};

fn solve(alp: VVec<usize>, memo: Vec<Vec<usize>>) {

}

fn main() {
    let mut std = String::new();
    io::stdin().read_to_string(&mut std).unwrap();
    let mut input = std.split_whitespace();
    let mut out = BufWriter::new(io::stdout().lock());
    let n: usize = input.next().unwrap().parse().unwrap();
    let alp: Vec<char> = input.next().unwrap().chars();
    let alp_num: Vec<usize> = vec![0;&alp.len()]);
    for i in &alp {
        let count_one = alp_num.iter().filter(|&&x| x==1).count();
        alp_num.push()
    }


    // A->B->C の発見を3回イテレートできる限り、それは1個と数えられる
    // 何番目のA、B、Cを選ぶかどうかの選択ロジックがキモ
    // もしくは、区切り方を全部出した後、条件に合うもの？→計算量爆発！
    //
    // 数え上げる際、重複がいっぱい発生
    // これは木にしてメモ化というやつでは？
    dbg!(&alp);
    let mut memo: Vec<Vec<usize>> = Vec::new();
    let mut ans: usize = 0;
    for i in &alp {
        ans += solve(alp, memo);
    }
    println!("{}", ans);
}
