// use core::fmt;
// use std::{collections::btree_map::Values, fmt::{Display, Formatter, write}};

// use std::boxed;
// impl<T> Display for Node<T> {
//     fn fmt(&self, f:&mut Formatter) -> Result {
        
//     }
// }   
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::Not;

pub struct Node<T> {
    data:T,
    left:Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>
}  

impl<T:Ord + Display + Not + Copy> Node<T> {
    
    pub fn new(data:T) -> Self {
        Node {
            data,
            left:None,
            right:None
        }
    }

    fn insert(&mut self, value:T){
        if value < self.data {

            match self.left {
                Some(ref mut left_node ) => left_node.insert(value),  // if something available on left
                None => self.left = Some(Box::new(Node::new(value)))  // if Not Push here
            }

        } else if value > self.data {  // if value greater

                match self.right {  
                    Some(ref mut right_node)  => right_node.insert(value),   // Something here 
                    None => self.right = Some(Box::new(Node::new(value)))                   // Not then push in 
            }

        }
    }

    pub fn inorder(&self){
        if let Some(ref left) = self.left {
            left.inorder();
        } 
        print!("-> {}",self.data);

        if let Some(ref right) = self.right {
            right.inorder();
        }
    }

    pub fn postorder(&self) {
        if let Some(ref left) = self.left {
            left.postorder();
        }

        if let Some(ref right) = self.right {
            right.postorder();
        }

        print!("-> {}",self.data);
    }

    pub fn preorder(&self) {

        print!("-> {}",self.data);

        if let Some(ref left) = self.left {
            left.preorder();
        }

        if let Some(ref right) = self.right {
            right.preorder();
        }
    }

    pub fn level_order(&self) {
        let mut queue:VecDeque<&Node<T>> = VecDeque::new();

        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            print!("-> {}",node.data);

            if let Some(ref left) = node.left {
                queue.push_back(left);
            }

            if let Some(ref right) = node.right {
                queue.push_back(right);
            }
        }

    }
// find_min expects a reference to a Node and returns the min value
    fn find_min(node: &Node<T>) -> T {
        let mut current: &Node<T> = node;
        while let Some(ref left) = current.left {
            current = left.as_ref();
        }
        current.data
    }

    // delete takes ownership of the subtree (Option<Box<Node<T>>>)
    // and returns the possibly updated subtree.
    pub fn delete(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        match node {
            None => None,
            Some(mut boxed_node) => {
                if value < boxed_node.data {
                    boxed_node.left = Node::delete(boxed_node.left, value);
                    Some(boxed_node)
                } else if value > boxed_node.data {
                    boxed_node.right = Node::delete(boxed_node.right, value);
                    Some(boxed_node)
                } else {
                    // Node found — handle three cases

                    // 1) No children
                    if boxed_node.left.is_none() && boxed_node.right.is_none() {
                        None
                    }
                    // 2) Only left child
                    else if boxed_node.right.is_none() {
                        boxed_node.left.take()
                    }
                    // 3) Only right child
                    else if boxed_node.left.is_none() {
                        boxed_node.right.take()
                    }
                    // 4) Two children: find inorder successor (min in right subtree)
                    else {
                        // take the right child (we own it now)
                        let right_subtree = boxed_node.right.take().unwrap();

                        // find min in right subtree WITHOUT consuming it by borrowing
                        let min_val = Node::find_min(&right_subtree);

                        // replace current node's data with min
                        boxed_node.data = min_val;

                        // delete the min value from the right subtree and reattach
                        boxed_node.right = Node::delete(Some(right_subtree), min_val);

                        Some(boxed_node)
                    }
                }
            }
        }
    }
    

}

    



fn main(){
    let mut root = Node::new(10);
    root.insert(4);
    root.insert(8);
    root.insert(7);
    root.insert(2);
    root.insert(1);
    root.insert(9);

    println!("Inorder Traversal ...");

    root.inorder();
    println!();
    root.postorder();
    println!();

    root.preorder();
    println!();

    root.level_order();
    println!();
}   