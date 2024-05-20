pub mod linked_list;

use std::rc::Rc;

use linked_list::ListNode;

fn main() {
    let node_1 = ListNode::new(1);
    let node_2 = ListNode::new(2);
    let node_3 = ListNode::new(3);

    ListNode::link(node_1, node_2.clone());
    ListNode::link(node_2.clone(), node_3.clone());

    let mut current = node_3.clone();
    
    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.as_ref() {
        println!("{}", node.borrow().val);

        if Rc::ptr_eq(node, &node_3) {
            break;
        }

        current = node.clone();
    }

    ListNode::unlink(node_2.clone());

    println!("node with val = {} unlinked", node_2.borrow().val);

    let mut current = node_3.clone();
    
    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.as_ref() {
        println!("{}", node.borrow().val);

        if Rc::ptr_eq(node, &node_3) {
            break;
        }

        current = node.clone();
    }
}
