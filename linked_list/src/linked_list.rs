/* #[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<ListNode<T>>
} */

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }

    pub fn link(current: &mut Box<ListNode<T>>, mut node: Box<ListNode<T>>) {
		let next = current.next.take();

		node.next = next;

		current.next = Some(node);
    }

	pub fn unlink(prev: &mut Box<ListNode<T>>, node: &mut Box<ListNode<T>>) {
		let next = node.next.take();

		prev.next = next;
	}
}
