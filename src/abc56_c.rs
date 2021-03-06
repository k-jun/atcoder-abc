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

fn main() {
    let x: i64 = read();
    let mut sum: i64 = 0;
    let mut cnt: i64 = 0;
    while sum < x {
        cnt += 1;
        sum += cnt;
    }
    println!("{}", cnt);
}
