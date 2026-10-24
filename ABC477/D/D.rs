// https://atcoder.jp/contests/abc477/tasks/abc477_d

// これはコンテスト期間外での取り組み
use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let tiles_count: usize = input.next().unwrap().parse().unwrap();
    let queries_count: usize = input.next().unwrap().parse().unwrap();

    let mut tiles: Vec<(bool, char)> = vec![(false, 'a');tiles_count];

    for _ in 0..queries_count {
        let mode: usize = input.next().unwrap().parse().unwrap();
        match mode%2==1 {
            true => {
                let obj: usize = input.next().unwrap().parse().unwrap();
                tiles[obj-1].0 = match tiles[obj-1].0 {
                    true => false,
                    false=> true
                };
            }
            false => {
                let color: char = input.next().unwrap().parse().unwrap();
                for (b, c) in &mut tiles {
                    if *b==false {
                        *c = color
                    }
                }
            }
        };
    }

    let mut out = BufWriter::new(io::stdout().lock());
    for (_, c) in &tiles {
        write!(out, "{}", c).unwrap();
    }
}


