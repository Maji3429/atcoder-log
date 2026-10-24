// https://atcoder.jp/contests/abc477/tasks/abc477_a
use std::io::{self, Read, Write, BufWriter};
// use std::io;

fn main() {
    let colors: Vec<char> = vec!['B', 'Y', 'R', 'B'];
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let input_color: char = input.next().unwrap().parse().unwrap();

    let color_idx: usize = colors
        .iter()
        .position(|clr| clr == &input_color)
        .unwrap();

    let mut out = BufWriter::new(io::stdout());
    writeln!(out, "{}", colors[color_idx+1]).unwrap();
    
}

