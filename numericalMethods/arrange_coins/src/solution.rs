use std::f64;

pub struct Solution {}

impl Solution {
  pub fn arrange_coins(n: i32) -> i32 {
      if n < 0 { return -1; }
      if n < 1 { return 0; }
      let float_n = n as f64;
      let rows = ((1.0 + 8.0 * float_n).sqrt() - 1.0) / 2.0;
      return rows.floor() as i32;
  }
}