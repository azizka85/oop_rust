pub mod linked_list;

// use std::{cell::RefCell, rc::Rc};

use linked_list::ListNode;

fn main() {
    let mut node_1 = Box::new(
        ListNode::new(1)
    );

    let node_2 = Box::new(
        ListNode::new(2)
    );

    let node_3 = Box::new(
        ListNode::new(3)
    );
    
    let next = node_1.link(node_2);

    next.link(node_3);

    let mut current = &node_1;

    println!("{}", current.val);

    while let Some(node) = current.next.as_ref() {
        println!("{}", node.val);

        current = node;
    }

    println!("--- Deleting node with val = 2");

    let mut current = &mut node_1;   

    while let Some(node) = current.unlink() {
        if node.val != 2 {
            current = current.link(node);
        }
    }

    let mut current = &node_1;

    println!("{}", current.val);

    while let Some(node) = current.next.as_ref() {
        println!("{}", node.val);

        current = node;
    }

    /* let node_1 = Rc::new(
        RefCell::new(
            ListNode::new(1)
        )
    );

    let node_2 = Rc::new(
        RefCell::new(
            ListNode::new(2)
        )
    );

    println!("Number of references to node_2 after creating = {}", Rc::strong_count(&node_2));

    let node_3 = Rc::new(
        RefCell::new(
            ListNode::new(3)
        )
    );

    let node_2 = node_1.borrow_mut().link(node_2);

    println!("Number of references to node_2 after linking to node_1 = {}", Rc::strong_count(&node_2));

    node_2.borrow_mut().link(node_3);

    println!("Number of references to node_2 after linking node_3 to this node = {}", Rc::strong_count(&node_2));

    let mut current = node_1.clone();

    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.as_ref() {
        println!("{}", node.borrow().val);

        current = node.clone();
    }

    println!("Number of references to node_2 after traversing = {}", Rc::strong_count(&node_2));

    println!("\n--- Deleting node with val = 2");

    let mut current = node_1.clone();

    while let Some(node) = current.clone().borrow().next.as_ref() {
        if node.borrow().val == 2 {
            break;
        } else {
            current = node.clone();
        }        
    } 

    current.borrow_mut().unlink();

    println!("Number of references to node_2 after unlinking of node_2 = {}", Rc::strong_count(&node_2));

    let mut current = node_1.clone();

    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.as_ref() {
        println!("{}", node.borrow().val);

        current = node.clone();
    }

    println!("Number of references to node_2 after traversing = {}", Rc::strong_count(&node_2)); */
}
