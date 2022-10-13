// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn add(mut self, val: i32) -> Self {
        self.next = Some(Box::new(ListNode::new(val)));
        self
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(head) = head {
        let mut head = head;
        let mut prev = None;
        while let Some(next) = (*head).next {
            (*head).next = prev;
            prev = Some(head);
            head = next;
        }
        (*head).next = prev;

        Some(head)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let head: Option<Box<ListNode>> =
            Some(Box::new(ListNode::new(1).add(2).add(3).add(4).add(5)));
        assert_eq!(
            reverse_list(head),
            Some(Box::new(ListNode::new(5).add(4).add(3).add(2).add(1)))
        );
    }

    #[test]
    fn example2() {
        let head = Some(Box::new(ListNode::new(1).add(2)));
        assert_eq!(reverse_list(head), Some(Box::new(ListNode::new(2).add(1))));
    }

    #[test]
    fn example3() {
        let head = None;
        assert_eq!(reverse_list(head), None);
    }
}
