pub mod linked_list;

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
}
