use std::vec;
use crate::Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // let len=nums.len();
        // let mut l_vec=vec![1;len];
        // let mut res=vec![1;len];
        // let mut r_vec=vec![1;len];
        // //左乘积
        // for i in 1..len{
        //     l_vec[i]=l_vec[i-1]*nums[i-1];
        // }
        // //右乘积
        // for j in (0..len-1).rev() {
        //     r_vec[j] *= r_vec[j+1]*nums[j+1];
        // }
        // for i in 0..len{
        //     res[i]=l_vec[i]*r_vec[i]
        // }
        // res

        //右乘积可以使用累积数表示
        let len=nums.len();
        let mut r=1;
        let mut res=vec![1;len];
        //左乘积
        for i in 1..len{
            res[i]=res[i-1]*nums[i-1];
        }
        //右乘积
        for j in (0..len).rev() {
            res[j]*=r;
            r *= nums[j];
        }
        res

    }
}

/*
[1,2,3,4]
左乘积 [1,1,2,6]    右乘积 [24,12,4,1]
3  1
2  4
1  12
0  24
[24,12,8,6]

[-1,1,0,-3,3]
pre [-1*1,1*0,0*-3,-3*3]   [-1,0,0,-9]
[0,0,9,0,0]
*/


#[test]
fn test() {
    assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
    assert_eq!(Solution::product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
}
