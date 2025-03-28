use crate::Solution;
// 2. 两数相加
// 中等

// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。

// 请你将两个数相加，并以相同形式返回一个表示和的链表。

// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

// 示例 1：

// 输入：l1 = [2,4,3], l2 = [5,6,4]
// 输出：[7,0,8]
// 解释：342 + 465 = 807.
// 示例 2：

// 输入：l1 = [0], l2 = [0]
// 输出：[0]
// 示例 3：

// 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// 输出：[8,9,9,9,0,0,0,1]

/*
9,9,9,9,9,9,9
      9,9,9,9
*/

// 提示：

// 每个链表中的节点数在范围 [1, 100] 内
// 0 <= Node.val <= 9
// 题目数据保证列表表示的数字不含前导零

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head=ListNode::new(0);
        let mut current_node=&mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some()|| carry != 0 {
            let val = l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val)
                + carry;
            carry = val / 10;

            current_node.next=Some(Box::new(ListNode::new(val % 10)));
            current_node = current_node.next.as_mut()?;

            l1=l1.and_then(|node|{node.next});
            l2=l2.and_then(|node|{node.next});
        }
        head.next
    }
}

#[test]
fn test() {
    let l1_1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(3))),
        })),
    }));

    let l2_1 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));

    assert_eq!(
        Solution::add_two_numbers(l1_1, l2_1),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }))
    );

    let l1_2 = Some(Box::new(ListNode { val: 0, next: None }));

    let l2_2 = Some(Box::new(ListNode { val: 0, next: None }));

    assert_eq!(
        Solution::add_two_numbers(l1_2, l2_2),
        Some(Box::new(ListNode { val: 0, next: None }))
    );
}
