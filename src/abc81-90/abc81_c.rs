
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

use std::collections::BinaryHeap;
use std::collections::HashMap;
fn main() {
  let n = read::<i64>();
  let k = read::<i64>();
  let mut bh: BinaryHeap<i64> = BinaryHeap::new();
  let mut dist: HashMap<i64, i64> = HashMap::new();
  let mut ans = 0;
  for _ in 0..n {
    let a = read::<i64>();
    let current = dist.entry(a).or_insert(0);
    *current += 1;
  }

  let dist_len: i64 = dist.len() as i64;
  for (k, v) in dist.iter() {
    bh.push(-v)
  }

  if 0 < dist_len - k {
    for _ in 0..dist_len - k {
      ans += -bh.pop().unwrap();
    }
  }

  println!("{}", ans);
}

