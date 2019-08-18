
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
use std::collections::HashMap;

fn main() {
  let n = read::<i64>();
  let t = read::<i64>();
  let mut t_vec = vec![];


  for _ in 0..n {
    t_vec.push(read::<i64>())
  }

  let mut total = t;
  let mut nokori = 0;

  for i in 1..t_vec.len() {
    let current = t_vec[i];
    let prev = t_vec[i - 1];
    total += t;
    if current - prev < t {
      nokori += t - (current - prev).abs();
    }

  }

  println!("{}", total - nokori);
}