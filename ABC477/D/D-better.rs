// https://atcoder.jp/contests/abc477/tasks/abc477_d
// 問題点: mode2は計算量が果てしない
// 解決策: 「まだ色が塗られていない」かつ「タイルに覆われていない」マスの集合 S を管理して、後ろからクエリを見る。
// 数学の「集合」の概念を扱うため、以下をインポート：
use std::collections::HashSet;
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let tiles_count: usize = input.next().unwrap().parse().unwrap();
    let queries_count: usize = input.next().unwrap().parse().unwrap();

    // let tiles: Vec<(bool, char)> = vec![(false, 'a');tiles_count];

    let queries: Vec<(usize, &str)> = (0..queries_count)
        .map(|_| {
            (input.next().unwrap().parse().unwrap(), input.next().unwrap())
        })
        .collect();
    
    let mut ans: Vec<char> = vec!['a'; tiles_count];

    // 一番最後のtrue/false状態を求めておく
    let mut is_covered = vec![false; tiles_count];
    for &(mode, val) in queries.iter() {
        if mode == 1 {
            let tgt = val.parse::<usize>().unwrap()-1;
            is_covered[tgt] = !is_covered[tgt];
        }
    }

    let mut s: HashSet<usize> = HashSet::new();
    for (idx, &b) in is_covered.iter().enumerate() {
        if !b {
            s.insert(idx);
        }
    }
    
    for (mode, val) in queries.into_iter().rev() {
        if mode == 2 {
            let color = val.parse().unwrap();
            for idx in s.drain() { // drainをイテレートすると、使ったそばから削除してメモリ削減できる
                ans[idx] = color;
            }
        } else if mode == 1 {
            let target = val.parse::<usize>().unwrap() -1;
            is_covered[target] = !is_covered[target];
            if ans[target] == 'a' {
                match is_covered[target] {
                    true => s.insert(target),
                    false=> s.remove(&target)
                };
            }
        }
    }
    

    let mut out = BufWriter::new(io::stdout().lock());
    for c in &ans {
        write!(out, "{}", c).unwrap();
    }
}
