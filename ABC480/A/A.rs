// https://atcoder.jp/contests/abc480/tasks/abc480_a
use std::io::{self, Read};

fn verify(grid: Vec<&str>) -> &str {
    if grid[0]=="A" || grid[0]=="D"{
        if grid[3]=="A" || grid[3]=="D" {
            return "No";
        }
    }
    if grid[1]=="A" || grid[1]=="D" {
        if grid[2]=="A" || grid[2]=="D" {
            return "No";
        }
    }
    return "Yes";
}

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let nnn: Vec<&str> = (0..2)
        .map(|_| input.next().unwrap())
        .collect();
    
    let mut grid: Vec<&str> = vec!["A";4];
    grid[0] = &nnn[0][0..1];
    grid[1] = &nnn[0][1..2];
    grid[2] = &nnn[1][0..1];
    grid[3] = &nnn[1][1..2];

    let answer = verify(grid);
    println!("{}", &answer);

}