struct ComputeArray;


impl ComputeArray {
    // 矩阵乘法运算
    fn multiply_matrix(matrix_1: Vec<Vec<i32>>,
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

    fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
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
}

#[test]
fn test() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    assert_eq!(ComputeArray::three_sum(nums), res);

    let matrix_1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let matrix_2 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let res = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(ComputeArray::multiply_matrix(matrix_1, matrix_2), res);
}