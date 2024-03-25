pub mod algorithm;

// 声明模块algorithm模块 --> 找algorithm下的mod.rs
use crate::algorithm::sorts::{bubble, };

fn main() {
    let mut nums = vec![9, 4, 2, 6, 7, 3, 8, 1, 5];
    bubble(&mut nums);
    println!("bubble: {:?}", nums);

    let x: i32 = 5;
    let y = x as f64;
    println!("y: {}, type: {}", y, y);
}
