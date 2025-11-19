use core::panic;
use std::{fmt::{self, Display, Formatter}}; 


#[derive(Debug)]
pub struct Node<T> {
    data:T,
    next:Option<Box<Node<T>>>      // next node
}

pub struct LinkedList<T> {
    head : Option<Box<Node<T>>>,     // linked contain every node
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    next:Option<&'a Node<T>> // we return a reference to Node
}

impl<'a, T> Iterator for Iter<'a, T> {   // they difing a behavior
    type Item = &'a Node<T>; 

    fn next(&mut self) -> Option<Self::Item> {
        // map over the next node
        self.next.map(|node|{
            self.next = node.next.as_ref().map(|next_node| &**next_node);
            return node
        })
    }
  
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut iter = self.iter();

        if let Some(first) = iter.next() {
            write!(f, "{}", first.data)?;
            for node in iter {
                write!(f, ", {}", node.data)?;
            }
        }

        write!(f, "]")
    }
}

impl<T:Display> Display for Node<T> {
    fn fmt(&self,f:&mut Formatter<'_>) -> fmt::Result {
        write!(f,"[")?;
        write!(f, "{}",self.data)?;
        write!(f,"]")
    }
}

impl<T> LinkedList<T>{

    // linked list created
    pub fn new() -> Self {
        return LinkedList {
            head:None
        }
    }

    // push front
    pub fn push_front(&mut self, value:T){
        let newnode = Box::new(Node {
            data:value,
            next:self.head.take(),
        });

        self.head = Some(newnode);
    }

        // push back
    pub fn push_back(&mut self, value: T) {
        let mut current = &mut self.head;

        // traverse until we find None (end of list)
        while let Some(node) = current {
            current = &mut node.next;
        }

        // now current is pointing to the final None
        *current = Some(Box::new(Node { data: value, next: None }));
    }

    // pop_front
    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take();

        old_head.map(|node| {
            let node = *node;  //  derefenece value into node 
            self.head = node.next;   // self.head assign to node.next
            return node.data        // return node.data   now bich wala gaya
        })

    }

    //pop_back
    pub fn pop_back(&mut self) -> Option<T> {
        let mut head = &mut self.head;
        while head.is_some() {
            if head.as_mut().unwrap().next.is_none() {
                break;
            }
            head = &mut head.as_mut().unwrap().next;    // last node reached here
        }
        return head.take().map(|node| node.data);    // value goes to the head now last node None
    }

    // isEmpty
    pub fn isempty(&self) -> bool { 
        return self.head.is_none()
    }

    // top
    pub fn top(&mut self) -> Option<&Node<T>> {
         let front = self.head.as_mut().map(|node| &**node).clone();
         return front;
    }

    // for print
    pub fn iter(&self) ->Iter<'_,T> {
        Iter {
            next:self.head.as_ref().map(|node|&**node)
        }
    }

    // insert in middle
    pub fn insert_in_middle(&mut self, value:T, index:i32){
        /*
             Final Summary:

            Expression	Meaning
            node.next	         --     the current link (e.g., Some(Box::new(B)))
            node.next.take()	 --     remove that link (set to None) and return it
            next: node.next.take()	 -- make the new node’s next pointer point to the old next
            node.next = Some(newnode) - attach the new node into the chain
         */
         if index == 0 {
            let new_node = Box::new(Node {
                data: value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            return;
        }

        let mut temp = &mut self.head;
        let mut i = 0;

        while let Some(node) = temp.as_mut() {
            if i + 1 == index {
                let newnode = Box::new(Node {data:value,next:node.next.take()});
                node.next = Some(newnode);
                return;
            }
            temp = &mut node.next;
            i += 1;
        }
        panic!("index out of bound")
    }    

    // delete list
    pub fn delete(&mut self){
        let mut curr =  self.head.take();

        while let Some(mut boxed_node) = curr {
            curr = boxed_node.next.take();    // replacing with none
        }

    }

    //recursive search
    pub fn search(&mut self,key:T) -> Option<i32> {
        let temp = self.head;
        

    }

}



fn main(){
    let mut list = LinkedList::<i32>::new();

    list.push_back(3);
    list.push_back(6);
    list.push_back(7);
    list.push_back(5);
    list.push_back(4);
    list.push_back(8);
    list.push_back(1);
    println!("{}",list);

    list.pop_back();
    println!("{}", list);

    list.pop_front();
    println!("{}",list);

    let front = list.top();
    println!("{}",front.unwrap());

    list.insert_in_middle(9,2);
    println!("{}",list);

    list.delete();
    println!("{}",list);
}   