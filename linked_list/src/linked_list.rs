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

    pub fn link(&mut self, mut node: Box<ListNode<T>>) -> &mut Box<ListNode<T>> {
		let next = self.next.take();

		node.next = next;

		self.next = Some(node);

        self.next.as_mut().unwrap()
    }

	pub fn unlink(&mut self) -> Option<Box<ListNode<T>>> {
		let mut next = self.next.take();

		if let Some(node) = next.as_mut() {
            self.next = node.next.take();
        }

        next
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

    pub fn link(&mut self, node: Rc<RefCell<ListNode<T>>>) -> Rc<RefCell<ListNode<T>>> {
		let next = self.next.take();

		node.borrow_mut().next = next;

		self.next = Some(node);

        self.next.as_ref().unwrap().clone()
    }

    pub fn unlink(&mut self) -> Option<Rc<RefCell<ListNode<T>>>> {
		let mut next = self.next.take();

		if let Some(node) = next.as_mut() {
            self.next = node.borrow_mut().next.take();
        }

        next
	}
} */
