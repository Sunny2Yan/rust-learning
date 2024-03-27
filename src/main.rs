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

    let a = [3, 4, 6];
    println!("{:?}", a.iter().max());

    let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子", 
        81, "wayslog", width=4, height=178);
    print!("{}", s);
}