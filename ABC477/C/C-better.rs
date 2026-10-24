// https://atcoder.jp/contests/abc477/tasks/abc477_c
// 累積和を使う
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let q: usize = input.next().unwrap().parse().unwrap();
    // ASCII文字が前提となるため、バイト列として扱います
    let s = input.next().unwrap().as_bytes();
    let t = input.next().unwrap().as_bytes();

    // TがSに出現する位置を事前に調べ、累積和の配列を作成します
    let mut prefix_sum = vec![0; s.len() + 1];
    if s.len() >= t.len() {
        // windowsを用いてTと同じ長さの部分配列を順に取り出します
        for (i, window) in s.windows(t.len()).enumerate() {
            let match_count = if window == t { 1 } else { 0 };
            prefix_sum[i + 1] = prefix_sum[i] + match_count;
        }
        // Tが収まらない末尾の領域には直前の値を引き継ぎます
        for i in s.len() - t.len() + 1..s.len() {
            prefix_sum[i + 1] = prefix_sum[i];
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for _ in 0..q {
        let l: usize = input.next().unwrap().parse().unwrap();
        let r: usize = input.next().unwrap().parse().unwrap();

        // 競技プログラミングで一般的な1始まりのインデックスを0始まりに補正します
        let l_idx = l - 1;
        let r_idx = r;

        // 指定された区間の長さがTの長さ以上であるかを確認します
        if r_idx - l_idx >= t.len() {
            // 事前に計算した累積和の差分を利用し、区間内の出現回数を求めます
            let count = prefix_sum[r_idx - t.len() + 1] - prefix_sum[l_idx];
            if count > 0 {
                writeln!(out, "Yes").unwrap();
            } else {
                writeln!(out, "No").unwrap();
            }
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}
