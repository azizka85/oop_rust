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

/* use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Rc<RefCell<ListNode<T>>>>
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }

    pub fn link(current: Rc<RefCell<ListNode<T>>>, node: Rc<RefCell<ListNode<T>>>) {
		let next = current.borrow_mut().next.take();

		node.borrow_mut().next = next;

		current.borrow_mut().next = Some(node);
    }

    pub fn unlink(prev: Rc<RefCell<ListNode<T>>>, node: Rc<RefCell<ListNode<T>>>) {
		let next = node.borrow_mut().next.take();

		prev.borrow_mut().next = next;
	}
} */
