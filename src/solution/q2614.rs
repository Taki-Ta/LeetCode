use crate::Solution;

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut vec= vec![];
        let mut max = 0;
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                if i == j || i + j == nums.len() - 1 {
                    vec.push(nums[i][j]);
                }
            }
        }
        for i in vec {
            if Solution::is_prime(i) && i > max {
                max = i;
            }
        }
        max
    }

    //判断是否是质数
    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=(n as f64).sqrt() as i32 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}