use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    #[allow(unused)]
    pub fn score_of_string(s: String) -> i32 {
        let bytes=s.as_bytes();
        let mut result=0;
        let mut bef:u8=0;
        let mut cur:u8=0;
        for i in 1..bytes.len(){
            bef=bytes[i-1] as u8;
            cur=bytes[i] as u8;
            result+= (if bef <cur{ cur-bef}else{bef-cur}) as i32
        }
        result
    }
}

#[test]
fn test(){
    assert_eq!(Solution::score_of_string("hello".to_string()),13);
    assert_eq!(Solution::score_of_string("zaz".to_string()),50);
}