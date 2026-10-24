// https://atcoder.jp/contests/abc477/tasks/abc477_a
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_color = input.trim().chars().next().unwrap();

    // 分岐が少ないのであれば、match文が適している
    let next_color = match input_color {
        'B' => 'Y',
        'Y' => 'R',
        'R' => 'B',
        _ => unreachable!(),
    };

    println!("{}", next_color);
}
