#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node1 = l1.unwrap();
    let mut node2 = l2.unwrap();
    let mut result = Box::new(ListNode::new(0));
    let mut reference = &mut result;
    while node1.val != -1 || node2.val != -1 {
        if node1.val != -1 {
            reference.val += node1.val;
            if let Some(n) = node1.next { node1 = n;}
            else { node1.val = -1; }
        }
        if node2.val != -1{
            reference.val += node2.val;
            if let Some(n) = node2.next { node2 = n;}
            else { node2.val = -1; }
        }
        if reference.val>9 {
            reference.val -= 10;
            reference.next = Some(Box::new(ListNode::new(1)));
        } else if node1.val != -1 || node2.val != -1 {reference.next = Some(Box::new(ListNode::new(0)));}
        if let Some(..) = &reference.next { reference = reference.next.as_mut().unwrap();}
    }
    Some(result)
}