
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
  let a = read::<String>();
  let b = read::<String>();
  let c = read::<String>();
  let As = a.chars().nth(0).unwrap();
  let Al = a.chars().nth(a.len() - 1).unwrap();
  let Bs = b.chars().nth(0).unwrap();
  let Bl = b.chars().nth(b.len() - 1).unwrap();
  let Cs = c.chars().nth(0).unwrap();
  let Cl = c.chars().nth(c.len() - 1).unwrap();

  println!("{}", if Al == Bs && Bl == Cs { "YES" } else { "NO" });
}