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
    let n: usize = read();
    let x: Vec<i64> = read_n(n);
    let mut avg: f64 = 0.0;

    let mut ans = 0;
    for i in 0..x.len() {
        let xi = x[i];
        avg += xi as f64;
    }

    avg = avg / x.len() as f64;
    let point: i64 = avg.round() as i64;

    for i in 0..x.len() {
        let xi = x[i];
        ans += (point - xi).pow(2)
    }
    println!("{}", ans);
}