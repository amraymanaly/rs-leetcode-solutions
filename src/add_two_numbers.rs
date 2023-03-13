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

pub fn solve(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(list1) = l1 {
        if let Some(list2) = l2 {
            return Some(add_em_up(list1, list2));
        }
    }

    None
}

fn add_em_up(mut l1: Box<ListNode>, mut l2: Box<ListNode>) -> Box<ListNode> {
    let count = |list: &Box<ListNode>| -> usize {
        let mut pointer = Some(list);
        let mut size = 0;
        while let Some(next) = pointer {
            size += 1;
            pointer = if let Some(n) = &next.next {
                Some(n)
            } else {
                None
            }
        }
        size
    };

    let len1 = count(&l1);
    let len2 = count(&l2);

    let (mut bigger, mut smaller);
    let (big_len, small_len);

    if len1 > len2 {
        bigger = &mut l1;
        smaller = &l2;
        big_len = len1;
        small_len = len2;
    } else {
        bigger = &mut l2;
        smaller = &l1;
        big_len = len2;
        small_len = len1;
    }

    let mut result;
    let mut carry = false;
    let mut done = small_len == 0;

    for i in 1..=big_len {
        result = bigger.val + if done { 0 } else { smaller.val } + if carry { 1 } else { 0 };

        if result > 9 {
            result -= 10;
            carry = true;
        } else {
            carry = false;
        }

        bigger.val = result;

        if i == big_len {
            if carry {
                bigger.next = Some(ListNode::new(1).into());
            }
        } else {
            bigger = bigger.next.as_mut().unwrap();
        }

        if i == small_len {
            done = true;
        }

        if !done {
            smaller = smaller.next.as_ref().unwrap();
        }
    }

    if len1 > len2 {
        l1
    } else {
        l2
    }
}
