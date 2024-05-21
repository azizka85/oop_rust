/* #[derive(Debug)]
pub struct ListNode<T> {
    val: T,
    prev: &ListNode<T>,
    next: &ListNode<T>
} */

/* #[derive(Debug)] 
pub struct ListNode<'a, T> {
    pub val: T,
    pub prev: &'a ListNode<'a, T>,
    pub next: &'a ListNode<'a, T>
}

impl<'a, T> ListNode<'a, T> {
    pub fn new(val: T) -> Self {
        Self { val, prev: ???, next: ??? }
    }
} */

/* #[derive(Debug)]
pub struct ListNode<'a, T> {
    pub val: T,
    pub prev: &'a Option<ListNode<'a, T>>,
    pub next: &'a Option<ListNode<'a, T>>
}

static NONE: Option<ListNode<???>> = None;

impl<'a, T> ListNode<'a, T> {
    pub fn new(val: T) -> Self {
        Self { val, prev: &NONE, next: &NONE }
    }
} */

/* #[derive(Debug)]
pub struct ListNode<'a, T> {
    pub val: T,
    pub prev: Option<&'a ListNode<'a, T>>,
    pub next: Option<&'a ListNode<'a, T>>
}

impl<'a, T> ListNode<'a, T> {
    pub fn new(val: T) -> Self {
        let mut item = Self { val, prev: None, next: None };

        item.prev = Some(&item);
        item.next = Some(&item);

        item
    }    
} */

/* use std::rc::Rc;

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub prev: Option<Rc<ListNode<T>>>,
    pub next: Option<Rc<ListNode<T>>>
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Rc<Self> {
        let item = Self {val, prev: None, next: None};
        let item_ptr = Rc::new(item);
        
        item_ptr.prev = Some(item_ptr.clone());
        item_ptr.next = Some(item_ptr.clone());

        item_ptr
    }
} */

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub prev: Option<Rc<RefCell<ListNode<T>>>>,
    pub next: Option<Rc<RefCell<ListNode<T>>>>
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Rc<RefCell<Self>> {
        let item = Self {val, prev: None, next: None};

        let item_ptr = Rc::new(
            RefCell::new(item)
        );
        
        item_ptr.borrow_mut().prev = Some(item_ptr.clone());
        item_ptr.borrow_mut().next = Some(item_ptr.clone());

        item_ptr
    }

    pub fn link(current: &mut Rc<RefCell<Self>>, node: &mut Rc<RefCell<Self>>) {
        let next = current.borrow_mut().next.take();

        current.borrow_mut().next = Some(node.clone());

        if let Some(ptr) = next.as_ref() {
			if Rc::ptr_eq(current, ptr) {
				current.borrow_mut().prev = Some(node.clone());
			}
        }
        
        node.borrow_mut().prev = Some(current.clone());        
        node.borrow_mut().next = next;
    }

	pub fn unlink(node: &mut Rc<RefCell<Self>>) {
		let next = node.borrow_mut().next.take().unwrap();
		let prev = node.borrow_mut().prev.take().unwrap();

		next.borrow_mut().prev = Some(prev.clone());
		prev.borrow_mut().next = Some(next.clone());

		if !Rc::ptr_eq(&next, &prev) {
			node.borrow_mut().next = Some(node.clone());
			node.borrow_mut().prev = Some(node.clone());
		}		
	}
}

/* use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub prev: Option<Weak<RefCell<ListNode<T>>>>,
    pub next: Option<Weak<RefCell<ListNode<T>>>>
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Rc<RefCell<Self>> {
        let item = Self {val, prev: None, next: None};

        let item_ptr = Rc::new(
            RefCell::new(item)
        );
        
        item_ptr.borrow_mut().prev = Some(Rc::downgrade(&item_ptr));
        item_ptr.borrow_mut().next = Some(Rc::downgrade(&item_ptr));

        item_ptr
    }

    pub fn link(current: &mut Rc<RefCell<Self>>, node: &mut Rc<RefCell<Self>>) {
        let next = current.borrow_mut().next.take();

        current.borrow_mut().next = Some(Rc::downgrade(node));

        let current_ptr = Rc::downgrade(current);

        if let Some(ptr) = next.as_ref() {
			if Weak::ptr_eq(&current_ptr, ptr) {
				current.borrow_mut().prev = Some(Rc::downgrade(node));
			}
        }
        
        node.borrow_mut().prev = Some(current_ptr);        
        node.borrow_mut().next = next;
    }

	pub fn unlink(node: &mut Rc<RefCell<Self>>) {
		let next = node.borrow_mut().next.take().unwrap();
		let prev = node.borrow_mut().prev.take().unwrap();

        let next = next.upgrade();
        let prev = prev.upgrade();

        if next.is_some() && prev.is_some() {
            let next = next.unwrap();
            let prev = prev.unwrap();

            next.borrow_mut().prev = Some(Rc::downgrade(&prev));
            prev.borrow_mut().next = Some(Rc::downgrade(&next));

            if !Rc::ptr_eq(&next, &prev) {
                node.borrow_mut().next = Some(Rc::downgrade(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&node));
            }
        }				
	}
}  */
