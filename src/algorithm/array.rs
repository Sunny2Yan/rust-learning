pub struct FindArray;
pub struct ComputeArray;


impl FindArray {
    // 查找有序列表中目标数字重复次数
    fn find_number(nums: Vec<i32>, target: i32) -> usize{
        fn __find(nums: &Vec<i32>, tar: i32) -> usize {
            let mut left = 0;
            let mut right = nums.len() - 1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid] <= tar {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            left
        }
        __find(&nums, target) - __find(&nums, target - 1)
    }
}

impl ComputeArray {
    // 矩阵乘法运算
    pub fn multiply_matrix(matrix_1: Vec<Vec<i32>>,
        matrix_2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut res = vec![vec![0; matrix_1.len()]; matrix_2[0].len()];

            for i in 0..matrix_1.len() {
                for j in 0..matrix_2[0].len() {
                    for k in 0..matrix_1[0].len() {
                        res[i][j] += matrix_1[i][k] * matrix_2[k][j];
                    }
                }
            }
            res
        }

    // 区间合并
    pub fn merge_arr(intervals: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|x| x[0]);
        let mut res: Vec<Vec<i32>> = vec![];

        for interval in intervals {
            if res.is_empty() || res[res.len() - 1][1] < interval[0] {
                res.push(interval.clone());
            } else {
                let last = res.len() - 1;  // 可变引用res与不可变引用res.len()不能同时存在
                res[last][1] = std::cmp::max(res[last][1], interval[1])
            }
        }
        res
    }

    // 三数之和
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];

        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let s = nums[i] + nums[left] + nums[right];
                if s == 0 {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left + 1] == nums[left] {
                        left += 1;
                    }
                    while left < right && nums[right - 1] == nums[right] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if s < 0{
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        res
    }

    // 生成杨辉三角
    fn generate(num_rows: usize) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        for i in 0..num_rows {
            let mut temp: Vec<i32> = vec![];
            for j in 0..i+1 {
                if j==0 || j==i {
                    temp.push(1);
                } else {
                    temp.push(res[i-1][j-1] + res[i-1][j]);
                }
            }
            res.push(temp);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![-1, 0, 1, 2, 3, 4, 4, 4, 5, 6];
    let res = 3;
    assert_eq!(FindArray::find_number(nums, 4), res);

    let matrix_1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let matrix_2 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let res = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(ComputeArray::multiply_matrix(matrix_1, matrix_2), res);

    let mut intervals = vec![vec![1,3], vec![2,6], vec![8,10], vec![15,18]];
    let res = vec![vec![1,6], vec![8,10], vec![15,18]];
    assert_eq!(ComputeArray::merge_arr(&mut intervals), res);

    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    assert_eq!(ComputeArray::three_sum(nums), res);

    let num_rows = 4;
    let res = vec![vec![1], vec![1,1], vec![1,2,1], vec![1,3,3,1]];
    assert_eq!(ComputeArray::generate(num_rows), res);
}