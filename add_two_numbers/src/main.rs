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

/// Add Two Numbers
/// https://leetcode.com/problems/add-two-numbers/description/
pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Lists to store carry values and digits for final sum
    let mut carry: i32 = 0;
    let mut result: ListNode = ListNode::new(0);
    let mut current: &mut ListNode = &mut result;

    loop {
        if (carry == 0) & (l1 == None) & (l2 == None)  {
            break;
        }

        // Initializing sum value
        let mut sum: i32 = carry;

        // Get the value from left number
        let num1: i32 = match l1.as_ref() {
            Some(s) => s.val,
            None => 0
        };

        // Get the value from right number
        let num2: i32 = match l2.as_ref() {
            Some(s) => s.val,
            None => 0
        };

        // Sum all numbers, and calculate carry value
        sum += num1 + num2;
        carry = sum / 10;

        // Keep track of result digits
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        if l1 != None {
            l1 = l1.clone().unwrap().next;
        }
        
        if l2 != None {
            l2 = l2.clone().unwrap().next;
        }
    }
    result.next
}

fn main() {
    let l1: Option<Box<ListNode>> = Some(Box::new(
        ListNode{
            val: 2,
            next: Some(Box::new(
                ListNode {
                    val: 4,
                    next: Some(Box::new(
                        ListNode {
                            val: 3, 
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    let l2: Option<Box<ListNode>> = Some(Box::new(
        ListNode{
            val: 5,
            next: Some(Box::new(
                ListNode {
                    val: 6,
                    next: Some(Box::new(
                        ListNode {
                            val: 4, 
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    println!("Test Case 1: {:?}", add_two_numbers(l1, l2));

    let l1: Option<Box<ListNode>> = Some(Box::new(
        ListNode{
            val: 9,
            next: Some(Box::new(
                ListNode {
                    val: 9,
                    next: Some(Box::new(
                        ListNode {
                            val: 9, 
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    let l2: Option<Box<ListNode>> = Some(Box::new(
        ListNode{
            val: 9,
            next: Some(Box::new(
                ListNode {
                    val: 9,
                    next: None
                }
            ))
        }
    ));

    println!("Test Case 2: {:?}", add_two_numbers(l1, l2));
}
