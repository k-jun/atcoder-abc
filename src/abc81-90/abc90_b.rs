
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
  let a = read::<i64>();
  let b = read::<i64>();
  let mut cnt = 0;

  for i in a..b + 1 {
    let i_str = i.to_string();
    let i_vec: Vec<char> = i_str.chars().collect();
    let mut i_vec_rev = i_vec.clone();
    i_vec_rev.reverse();

    if i_vec == i_vec_rev {
      cnt += 1;
    }
  }

  println!("{}", cnt);
}