use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A `Node` represents a single node in a tree structure.
/// Each node has:
/// - an `i32` value
/// - a weak reference to its parent (to avoid reference cycles)
/// - a vector of strong `Rc` pointers to its children
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Create a new leaf node with value `3`
    // It has no parent and no children
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // Initially no parent
        children: RefCell::new(vec![]),    // No children
    });

    // At this point, the leaf node has:
    // - 1 strong reference (owned by `leaf`)
    // - 0 weak references
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // Create a branch node with value `5`
    // The branch node has one child (the previously created `leaf` node)
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()), // Initially no parent
        children: RefCell::new(vec![Rc::clone(&leaf)]), // Add leaf as child
    });

    // Now set the leaf node's parent to a weak reference of the branch
    // This avoids a strong reference cycle between parent and child
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // Now the leaf node can access its parent (branch)
    // This will print Some(branch) showing the parent reference is set
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Finally, print reference counts for leaf node:
    // - 2 strong references: one in `leaf`, one in `branch.children`
    // - 0 or 1 weak references: one from leaf to branch (doesn't affect leaf's count)
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
