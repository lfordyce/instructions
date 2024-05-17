#[derive(Debug)]
pub struct ListNode {
    value: u8,
    next: Option<Box<ListNode>>,
}

pub fn reverse_list(mut right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut left = None;
    while let Some(ref mut current) = right {
        let next_right = current.next.take();
        current.next = left;
        left = right;
        right = next_right;
    }
    left
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let mut list = None;
        for value in 0..5 {
            list = Some(Box::new(ListNode { value, next: list }));
        }
        println!("list: {:?}", list);
        println!("reverse: {:?}", reverse_list(list));
    }
}
