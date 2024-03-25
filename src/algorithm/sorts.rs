// use super::xxx::xx  调用algorithm目录下其他rs文件内的方法，即在同一个mod下面
// <==> use crate::algorithm::xxx::xx

// 1.冒泡排序
// 比较相邻的元素。如果第一个比第二个大，就交换他们两个。(每轮得到一个最大的数)
pub fn bubble<T: Ord>(arr: &mut [T]) {  // Ord 表示可以进行比较的泛型
    if arr.is_empty() {
        return;
    }

    let n = arr.len();
    for i in 0..n-1 {
        for j in i+1..n {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

// 2.选择排序
// 遍历列表，定义第 $i$ 个位置为最小（大）元素，再遍历 $[i:]$，如果有小于（大于）第 $i$ 个位置的值时，则交换，直至排完。
pub fn selection<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in i+1..n {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

// 3.插入排序
// pub fn insert<T>(arr: &mut [T]) {

// }

// // 4.快速排序
// pub fn quick<T>(arr: &mut [T]) {
//     if arr.len() <= 1 {
//         return;
//     } else {
//         let mut pivot = arr[0].clone();
//         let mut left = Vec::new();
//         let mut right = Vec::new();
//         for num in arr[1..] {
//             if num <= pivot {
//                 left.push(num.clone());
//             } else {
//                 right.push(num.clone());
//             }
//         }
//         quick(&mut left);
//         quick(&mut right);
//         return arr.clone_from_slice(&[left, vec![pivot.clone()], right].concat());
//     }
// }

// // 5.归并排序
// pub fn merge<T>(arr: &mut [T]) {

// }

// // 6.堆排序
// pub fn heap<T>(arr: &mut [T]) {}