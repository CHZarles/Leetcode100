struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut lhs = head;
        let mut rhs = None;

        while let Some(mut node) = lhs {
            lhs = node.next.take();
            node.next = rhs;
            rhs = Some(node);
        }

        rhs
    }
}

fn main() {
    // test node
    // 创建两个节点
    let mut node1 = ListNode::new(1);
    let node2 = ListNode::new(2);

    // 将第一个节点的 next 指向第二个节点
    node1.next = Some(Box::new(node2));

    // 打印第一个节点
    println!("{:?}", node1);

    let res = Solution::reverse_list(Some(Box::new(node1)));

    println!("{:?}", res.unwrap());
}
