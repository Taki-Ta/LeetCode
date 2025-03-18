// use crate::solution::diagonal_prime;
mod solution;

use solution::Solution;



fn main() {
    let nums = vec![vec![1,2,3], vec![5,17,7], vec![9,11,10]];
    let result = Solution::diagonal_prime(nums);
    println!("{}", result);
}


