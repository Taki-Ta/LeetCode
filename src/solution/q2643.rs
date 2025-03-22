use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    #[allow(unused)]
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pos=0;
        let mut max_num=0;
        let mut this_num=0;
        for (index,i) in mat.iter().enumerate(){
            this_num=i.iter().sum();
            if max_num<this_num{
                pos=index as i32;
                max_num=this_num;
            }
        }
        vec![pos,max_num]
    }
}

#[test]
fn test(){
    assert_eq!(Solution::row_and_maximum_ones(vec![vec![0,1],vec![1,0]]),vec![0,1]);
    assert_eq!(Solution::row_and_maximum_ones(vec![vec![0,0,0],vec![0,1,1]]),vec![1,2]);
    assert_eq!(Solution::row_and_maximum_ones(vec![vec![0,0],vec![1,1],vec![0,0]]),vec![1,2]);
}