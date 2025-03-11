// https://leetcode.com/problems/middle-of-the-linked-list/
// 876. Middle of the Linked List
// Given the head of a singly linked list, return the middle node of the linked list.
// If there are two middle nodes, return the second middle node.

// Example 1:
// Input: head = [1,2,3,4,5]
// Output: [3,4,5]
// Explanation: The middle node of the list is node 3.
//
// Example 2:
// Input: head = [1,2,3,4,5,6]
// Output: [4,5,6]
// Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
     }
   }
}

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}

// add tests but generate the list in a more compact form, use an auxiliary function if you want
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_node_1() {
        assert_eq!(Solution::middle_node(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None,
                        })),
                    })),
                })),
            })),
        }))), Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: None,
                })),
            })),
        })));
    }

    #[test]
    fn test_middle_node_2() {
        assert_eq!(Solution::middle_node(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: None,
                            })),
                        })),
                    })),
                })),
            })),
        }))), Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: None,
                })),
            })),
        })));
    }
}
