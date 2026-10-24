// https://atcoder.jp/contests/ahc071/tasks/ahc071_a

// WA... https://atcoder.jp/contests/ahc071/submissions/79246986
// AHC071 当時のルールで許可されている GitHub Copilot の Tab 補完を使用している
use std::io::{self, Read, Write, BufWriter};

fn main() {
  let mut inp = String::new();
  io::stdin().read_to_string(&mut inp).unwrap();
  let mut input = inp.split_whitespace();

  let w: usize = input.next().unwrap().parse().unwrap();
  let h: usize = input.next().unwrap().parse().unwrap();

  // Kは穴の空いた総数
  let k: usize = input.next().unwrap().parse().unwrap();

  // 値段表
  let _c1: usize = input.next().unwrap().parse().unwrap();
  let _c3: usize = input.next().unwrap().parse().unwrap();
  let _c5: usize = input.next().unwrap().parse().unwrap();
  let _c7: usize = input.next().unwrap().parse().unwrap();
  let _c9: usize = input.next().unwrap().parse().unwrap();

  // 穴の座標
  let mut holes = Vec::new();
  for _ in 0..k {
      holes.push((input.next().unwrap().parse().unwrap(), input.next().unwrap().parse().unwrap()));
  }

  // 出力に使うための配列(x, y, l) *lはブロックの長さ
  let mut blocks = Vec::new();
  
  for y in 0..h {
      for x in 0..w {
          if holes.iter().any(|(hx, hy): &(usize, usize)| *hx == x && *hy == y) {
              blocks.push((x, y, 1));
              // 今埋めたブロックの下も埋める
              // 重複を気にする
              if y + 1 < h && !blocks.iter().any(|(bx, by, _)| *bx == x && *by == y + 1) {
                  blocks.push((x, y + 1, 1));
              }
              
          }
      }
  }
  let m = blocks.len();

  let mut out = BufWriter::new(io::stdout());
  writeln!(out, "{}", m).unwrap();
  for (x, y, l) in blocks {
      writeln!(out, "{} {} {}", x, y, l).unwrap();
  }
}