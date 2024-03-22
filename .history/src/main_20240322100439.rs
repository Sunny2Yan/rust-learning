pub mod algorithm;

// 声明模块algorithm模块 --> 找algorithm下的mod.rs
use crate::algorithm::sorts::{bubble, selection};

fn main() {
    let mut nums = vec![9, 4, 2, 6, 7, 3, 8, 1, 5];
    // bubble(&mut nums);
    // println!("bubble: {:?}", nums);

    selection(&mut nums);
    println!("selection: {:?}", nums)
}
