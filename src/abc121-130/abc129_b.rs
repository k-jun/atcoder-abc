// mod custom_lib;
use std::cmp;

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

pub fn read_n<T: FromStr>(n: u32) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: u32, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

fn main() {
  let n = read::<u32>();
  let w = read_n::<i32>(n);

  let mut sum: i32 = w.iter().sum();

  let mut possible_ans = 0;
  let mut answer = 100000000;
  for item in w {
    possible_ans += item;
    sum -= item;
    answer = cmp::min(answer, (sum - possible_ans).abs());
  }
  println!("{}", answer);
}