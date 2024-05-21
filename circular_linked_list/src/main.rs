pub mod linked_list;

use std::rc::{Rc, Weak};

use linked_list::ListNode;

fn main() {
    let mut node_1 = ListNode::new(1);
    let mut node_2 = ListNode::new(2);
    let mut node_3 = ListNode::new(3);

    println!(
        "Number of references to node_2 after creating: strong = {}, weak = {}", 
        Rc::strong_count(&node_2),
        Rc::weak_count(&node_2)
    );

    ListNode::link(&mut node_1, &mut node_2);

    println!(
        "Number of references to node_2 after linking to node_1: strong = {}, weak = {}", 
        Rc::strong_count(&node_2),
        Rc::weak_count(&node_2)
    );

    ListNode::link(&mut node_2, &mut node_3);

    println!(
        "Number of references to node_2 after linking node_3 to this node: strong = {}, weak = {}", 
        Rc::strong_count(&node_2),
        Rc::weak_count(&node_2)
    );

    let mut current = node_3.clone();
    
    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.as_ref() {
        println!("{}", node.borrow().val);

        if Rc::ptr_eq(node, &node_3) {
            break;
        }

        current = node.clone();
    }

    println!(
        "Number of references to node_2 after traversing: strong = {}, weak = {}", 
        Rc::strong_count(&node_2),
        Rc::weak_count(&node_2)
    );

    ListNode::unlink(&mut node_2);    

    println!("node with val = {} unlinked", node_2.borrow().val);

    println!(
        "Number of references to node_2 after unlinking: strong = {}, weak = {}", 
        Rc::strong_count(&node_2),
        Rc::weak_count(&node_2)
    );

    let mut current = node_3.clone();
    
    println!("{}", current.borrow().val);

    while let Some(node) = current.clone().borrow().next.as_ref() {
        println!("{}", node.borrow().val);

        if Rc::ptr_eq(node, &node_3) {
            break;
        }

        current = node.clone();
    }

    println!(
        "Number of references to node_2 after traversing: strong = {}, weak = {}", 
        Rc::strong_count(&node_2),
        Rc::weak_count(&node_2)
    );

    /* let mut node_1 = ListNode::new(1);    
    let mut node_3 = ListNode::new(3);

    {
        let mut node_2 = ListNode::new(2);

        println!(
            "Number of references to node_2 after creating: strong = {}, weak = {}", 
            Rc::strong_count(&node_2),
            Rc::weak_count(&node_2)
        );

        ListNode::link(&mut node_1, &mut node_2);

        println!(
            "Number of references to node_2 after linking to node_1: strong = {}, weak = {}", 
            Rc::strong_count(&node_2),
            Rc::weak_count(&node_2)
        );

        ListNode::link(&mut node_2, &mut node_3);

        println!(
            "Number of references to node_2 after linking node_3 to this node: strong = {}, weak = {}", 
            Rc::strong_count(&node_2),
            Rc::weak_count(&node_2)
        );

        let ptr = Rc::downgrade(&node_3);
        let mut current = ptr.clone();
        
        while let Some(node) = current.upgrade() {
            println!("{}", node.borrow().val);        

            current = node.borrow().next.as_ref().unwrap().clone();

            if Weak::ptr_eq(&current, &ptr) {
                break;
            }
        }

        println!(
            "Number of references to node_2 after traversing: strong = {}, weak = {}", 
            Rc::strong_count(&node_2),
            Rc::weak_count(&node_2)
        );
    }    

    let ptr = Rc::downgrade(&node_3);
    let mut current = ptr.clone();
    
    while let Some(node) = current.upgrade() {
        println!("{}", node.borrow().val);        

        current = node.borrow().next.as_ref().unwrap().clone();

        if Weak::ptr_eq(&current, &ptr) {
            break;
        }
    } */
}
