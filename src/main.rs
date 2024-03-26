pub mod algorithm;

// 声明模块algorithm模块 --> 找algorithm下的mod.rs
use crate::algorithm::sorts::{bubble, };
use crate::algorithm::array::ComputeArray;


fn main() {
    let mut nums = vec![9, 4, 2, 6, 7, 3, 8, 1, 5];
    bubble(&mut nums);
    println!("bubble: {:?}", nums);

    let nums = vec![-1, 0, 1, 2, -1, -4];
    println!("{:?}", ComputeArray::three_sum(nums));
}
