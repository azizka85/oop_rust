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
    
    ListNode::link(&mut node_1, node_2);
    ListNode::link(node_1.next.as_mut().unwrap(), node_3);

    let mut current = &node_1;

    println!("{}", current.val);

    while let Some(node) = current.next.as_ref() {
        println!("{}", node.val);

        current = node;
    }

    println!("--- Deleting node with val = 2");

    /* let mut current = &mut node_1;   
    let mut next_node = None; 

    println!("{}", current.val);

    while let Some(node) = current.next.as_mut() {
        if node.val == 2 {
            next_node = node.next.take();

            break;
        } else {
            println!("{}", node.val);

            current = node;
        }        
    }

    ListNode::unlink(current, next_node.as_mut().unwrap()); */

    let mut node = node_1.next.take();

    ListNode::unlink(&mut node_1, node.as_mut().unwrap());

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

    ListNode::link(node_1.clone(), node_2.clone());

    println!("Number of references to node_2 after linking to node_1 = {}", Rc::strong_count(&node_2));

    ListNode::link(node_2.clone(), node_3.clone());

    println!("Number of references to node_2 after linking node_3 to this node = {}", Rc::strong_count(&node_2));

    let mut current = node_1.clone();

    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.clone() {
        println!("{}", node.borrow().val);

        current = node.clone();
    }

    println!("Number of references to node_2 after traversing = {}", Rc::strong_count(&node_2));

    println!("\n--- Deleting node with val = 2");

    let mut current = node_1.clone();
    let mut target_node = node_1.clone();

    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.clone() {
        if node.borrow().val == 2 {
            target_node = node;

            break;
        } else {
            println!("{}", node.borrow().val);

            current = node.clone();
        }
    }

    println!("Number of references to node_2 after finding to delete = {}", Rc::strong_count(&node_2));

    ListNode::unlink(current.clone(), target_node.clone());

    target_node = node_1.clone();

    println!("Number of references to node_2 after unlinking from node_1 = {}", Rc::strong_count(&node_2));

    while let Some(node) = current.clone().borrow().next.clone() {
        println!("{}", node.borrow().val);

        current = node.clone();
    } */
}
