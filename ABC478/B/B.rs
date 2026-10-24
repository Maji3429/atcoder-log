// https://atcoder.jp/contests/abc478/tasks/abc478_b

// TLE, WAになっている...
use std::io::{self, BufWriter, Read, Write};
// use std::io::{self, Read};

fn combinations(x: usize) -> Vec<(usize, usize, usize)> {
    let mut ans: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..x {
        for j in 0..x {
            for k in 0..x {
                let a = i;
                let b = { if i!=j {j} else {continue;}};
                let c = { if i!=j && i!=k && j!=k {k} else {continue;}};
                ans.push((a,b,c));
            }
        }
    }
    return ans;
}


// Rustの数値型は値をそのまま複製できる性質を持つ
// usize, i32, bool など、メモリ上でのデータ容量が予め決まっている型に関しては、所有権の話は関係なく、自動で複製が行われる。
fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let topping_kind_quantity: usize = input.next().unwrap().parse().unwrap();
    let _total_price_cap: usize = input.next().unwrap().parse().unwrap();
    
    // let x: Vec<usize> = (0..n)
    //     .map(|_| input.next().unwrap().parse().unwrap())
    //     .collect();
    let _happiness_value: Vec<usize> = input
        .take(topping_kind_quantity)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    
    // let mut prices: Vec<usize> = Vec::new();
    let mut comb: Vec<(usize, usize, usize)> = combinations(topping_kind_quantity);
    for x in 0..comb.len() {
        if comb[x].0+comb[x].1+comb[x].2 > _total_price_cap {
            comb.remove(x);
        }
    }

    let mut menu: Vec<usize> = Vec::new(); // (cost, happiness)
    for i in 0..comb.len() {
        let a = comb[i].0;
        let b = comb[i].1;
        let c = comb[i].2;
        let happiness = _happiness_value[a] + _happiness_value[b] + _happiness_value[c];
        menu.push(happiness);
    }

    let max_happiness: &usize = menu.iter().max().unwrap();

    
    
    let mut out = BufWriter::new(io::stdout().lock());
    write!(out, "{} ", max_happiness).unwrap();
}