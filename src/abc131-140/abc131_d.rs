
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
use std::collections::BTreeMap;


fn main() {
  let mut n = read::<i64>();
  let mut map: BTreeMap<i64, i64> = BTreeMap::new();


  for i in 0..n {
    let a = read::<i64>();
    let b = read::<i64>();
    let current = map.entry(b).or_insert(0);
    *current += a;
  }

  let mut work = 0;
  let mut ans = "Yes";

  for (k, v) in map.iter() {
    work += *v;
    if k < &work {
      ans = "No"
    }
  }

  println!("{}", ans);
}
