use crate::Solution;

// 2829. k-avoiding 数组的最小总和
// 中等
// 相关标签
// 相关企业
// 提示
// 给你两个整数 n 和 k 。

// 对于一个由 不同 正整数组成的数组，如果其中不存在任何求和等于 k 的不同元素对，则称其为 k-avoiding 数组。

// 返回长度为 n 的 k-avoiding 数组的可能的最小总和。

// 示例 1：

// 输入：n = 5, k = 4
// 输出：18
// 解释：设若 k-avoiding 数组为 [1,2,4,5,6] ，其元素总和为 18 。
// 可以证明不存在总和小于 18 的 k-avoiding 数组。
// 示例 2：

// 输入：n = 2, k = 6
// 输出：3
// 解释：可以构造数组 [1,2] ，其元素总和为 3 。
// 可以证明不存在总和小于 3 的 k-avoiding 数组。 
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut result=0;
        let mut count=0;
        let mut current=0;
        while count<n{
            current+=1;
            if k<2*current && k>current{
                continue;
            }
            result+=current;
            count+=1;
        }
        result
    }
}


#[test]
fn test(){
    assert_eq!(Solution::minimum_sum(5, 4),18);
    assert_eq!(Solution::minimum_sum(2, 6),3);
    assert_eq!(Solution::minimum_sum(3, 6),6);
}