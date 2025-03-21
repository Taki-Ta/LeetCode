use crate::Solution;

/*
给你一个下标从 0 开始长度为 n 的整数数组 nums 和一个整数 k 。每一次操作中，你可以选择一个数并将它乘 2 。

你最多可以进行 k 次操作，请你返回 nums[0] | nums[1] | ... | nums[n - 1] 的最大值。

a | b 表示两个整数 a 和 b 的 按位或 运算。

 

示例 1：

输入：nums = [12,9], k = 1
输出：30
解释：如果我们对下标为 1 的元素进行操作，新的数组为 [12,18] 。此时得到最优答案为 12 和 18 的按位或运算的结果，也就是 30 。
示例 2：

输入：nums = [8,1,2], k = 2
输出：35
解释：如果我们对下标 0 处的元素进行操作，得到新数组 [32,1,2] 。此时得到最优答案为 32|1|2 = 35 。

8: 1 0 0 
1: 0 0 1
2: 0 1 0

left [0,8|1,8|1|2] = [0,8,9]
right [1|2,2|0,0] = [3,2,0]     [3,10,9]

[11, 3, 2, 0]
*/

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let len=nums.len();
        let mut left=vec![0;len+1];
        for i in (0..len).rev(){
            left[i]= left[i+1] | nums[i] as i64;
        }
        let mut res = 0;
        let mut pre = 0;
        for i in 0..len {
            res = res.max(pre | ((nums[i] as i64) << (k as i64)) | left[i + 1]);
            pre = pre | nums[i] as i64;
        }
        res
    }
}




#[test]
fn test(){
    assert_eq!(Solution::maximum_or(vec![8,1,2],2),35);
    assert_eq!(Solution::maximum_or(vec![12,9],1),30);
}