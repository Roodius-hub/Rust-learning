use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};



#[derive(Debug)]
struct DNode<T> {
    data:T,
    next:Option<Rc<RefCell<DNode<T>>>>,  // next node point count for Rc
    prev:Option<Weak<RefCell<DNode<T>>>> // for prev Weak
}
#[derive(Debug)]
struct DbList<T> {
    head:Option<Rc<RefCell<DNode<T>>>>,   // head -- rc
    tail:Option<Weak<RefCell<DNode<T>>>>  // tail -- weak
}

impl<T> DbList<T> {

    pub fn new() -> Self {   
        DbList {
            head:None,
            tail:None
        }
    }

    // push_front
    pub fn push_front(&mut self, value:T){
        match self.head.take() {
            Some(r) => {

                // creating new obj
                 let newnode = Rc::new(RefCell::new(DNode
                    {
                        data:value,
                        next:Some(r.clone()),  // r node now next
                        prev:None
                }));
                //tell the first object this is now in front    
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&newnode));   // prev now newnode creating Weak pointer 
                self.head = Some(newnode);  // finanlly pushed  on front
            }
            None => {
                // if head empty
                let newnode = Rc::new(RefCell::new(DNode{
                    data:value,
                    next:None,
                    prev:None
                }));
                self.tail = Some(Rc::downgrade(&newnode));
                self.head = Some(newnode);
            } 
        }
    }

    //push_back
    pub fn push_back(&mut self , value:T) {
        match self.tail.take() {
            Some(r) => {
                let newnode = Rc::new(RefCell::new(
                    DNode{
                        data:value,
                        prev: Some(r.clone()),
                        next:None    
                }));
                let st = Weak::upgrade(&r).unwrap();
                let mut m  = st.borrow_mut();
                self.tail = Some(Rc::downgrade(&newnode));
                m.next = Some(newnode);
            }
            None => {
                let newnode = Rc::new(RefCell::new(
                    DNode {
                        data:value,
                        prev: None,
                        next:None
                    }
                ));
                self.tail = Some(Rc::downgrade(&newnode));
                self.head = Some(newnode);
            }
        }
    }

}

fn main(){
    let mut list = DbList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);

    println!("{:?}", list);
}