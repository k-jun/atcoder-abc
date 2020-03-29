use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
    read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
    match n {
        0 => a,
        _ => {
            a.push(read::<T>());
            read_n_logic(n - 1, a)
        }
    }
}

use std::cmp::{max, min};

fn main() {
    let n: i64 = read();
    let mut a: Vec<i64> = vec![];
    let mut b: Vec<i64> = vec![];

    for _ in 0..n {
        let item_a: i64 = read();
        let item_b: i64 = read();
        a.push(item_a);
        b.push(item_b);
    }
    let mut ans: i64 = std::i64::MAX;

    let mut ab: Vec<i64> = [&a[..], &b[..]].concat();
    for i in &ab {
        for j in &ab {
            let mut maybe_ans = 0;
            for k in 0..n {
                let k_usize = k as usize;
                maybe_ans += (i - a[k_usize]).abs()
                    + (a[k_usize] - b[k_usize]).abs()
                    + (b[k_usize] - j).abs();
            }
            ans = min(ans, maybe_ans);
        }
    }
    println!("{}", ans);
}

