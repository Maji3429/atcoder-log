// https://atcoder.jp/contests/abc478/tasks/abc478_b
// 1 ms	2056 KiB
use std::io::{self, BufWriter, Read, Write};

fn main() {
    /* 三重ループの中で条件を満たしたときに最大値を直接更新するっていうやり方をさっさと覚えたほうが良い */
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let topping_kind_quantity: usize = input.next().unwrap().parse().unwrap();
    let total_price_cap: usize = input.next().unwrap().parse().unwrap();

    let happiness_value: Vec<usize> = input
        .take(topping_kind_quantity)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    dbg!(&happiness_value);

    let mut ans: usize = 0;
    /* ループの開始位置を前の変数＋1にすることで、余計な判定（if i != j など）なしで純粋な組み合わせにできます
     *  実際に、i<j<k が保証されることで、重複した組み合わせを排除できる */
    for i in 0..topping_kind_quantity {
        for j in (i + 1)..topping_kind_quantity {
            for k in (j + 1)..topping_kind_quantity {
                if i + j + k + 3 <= total_price_cap {
                    let happiness = happiness_value[i] + happiness_value[j] + happiness_value[k];
                    if happiness > ans {
                        ans = happiness;
                    }
                }
            }
        }
    }

    let mut out = BufWriter::new(io::stdout().lock());
    writeln!(out, "{}", ans).unwrap();
}
