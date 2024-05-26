#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

/// Add Two Numbers
/// https://leetcode.com/problems/add-two-numbers/description/
pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Lists to store carry values and digits for final sum
    let mut carry_vec: Vec<i32> = Vec::new();
    let mut result_vec: Vec<i32> = Vec::new();

    loop {
        // Get the carry value
        let carry: i32 = match carry_vec.pop() {
            Some(s) => s,
            None => 0
        };

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

        // Sum all numbers + carry value
        let mut sum: i32 = num1 + num2 + carry;
        
        // Incase sum value > 9, then calculate carry and resulting sum digit
        if sum > 9 {
            carry_vec.push(1);
            sum = sum - 10;
        }

        // Keep track of result digits
        result_vec.push(sum);

        if (carry_vec.len() == 0) & (l1 == None) & (l2 == None)  {
            break;
        } else {
            if l1 != None {
                l1 = l1.clone().unwrap().next;
            }
            
            if l2 != None {
                l2 = l2.clone().unwrap().next;
            }
        }
    }
    if result_vec.get(result_vec.len()-1).unwrap() == &0 {
        result_vec.pop();
    }
    create_linked_list(result_vec)
}

// Convert digits in result vector to Linked List
pub fn create_linked_list(result_vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut temp: Option<Box<ListNode>> = None;
    let mut cache: Option<Box<ListNode>> = None;
    for (idx, num) in result_vec.iter().rev().enumerate() {
        if idx == 0 {
            temp = Some(Box::new(
                ListNode {
                    val: *num,
                    next: None
                }
            ));
        } else {
            temp = Some(Box::new(
                ListNode {
                    val: *num,
                    next: cache
                }
            ))
        }
        cache = temp.clone();
    }
    temp
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
