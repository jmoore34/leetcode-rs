struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(list2)) => Some(list2),
            (Some(list1), None) => Some(list1),
            (Some(list1), Some(list2)) => {
                if list1.val < list2.val {
                    Some(Box::new(ListNode {
                        val: list1.val,
                        next: Solution::merge_two_lists(list1.next, Some(list2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: list2.val,
                        next: Solution::merge_two_lists(Some(list1), list2.next),
                    }))
                }
            }
        }
    }
}

pub fn main() {}

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
