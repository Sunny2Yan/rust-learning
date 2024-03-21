// use super::xxx::xx  调用algorithm目录下其他rs文件内的方法，即在同一个mod下面
// <==> use crate::algorithm::xxx::xx

pub fn bubble() {
    println!("{}", "hello world")
}