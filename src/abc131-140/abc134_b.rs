
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

use std::cmp::{max, min};

fn main() {
  let n = read::<i32>();
  let d = read::<i32>();

  let mut ans = n / (2 * d + 1);
  let amari = (n % (2 * d + 1));
  if amari != 0 {
    ans += 1;
  }
  println!("{}", ans);
}
