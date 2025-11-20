// use std::borrow::BorrowMut;
// use std::cell::RefCell;
// use std::rc::Rc;

// type Rcc<T> = Rc<RefCell<T>>; /* RefCell  allowning runtime mutatbility
// Rc take care of  owners count or one owner at a time
//  */

// pub fn rcc<T>(t: T) -> Rcc<T> {
//     Rc::new(RefCell::new(t))
// }

// pub struct SkipNode<T: PartialOrd> {
//     right: Option<Rcc<SkipNode<T>>>,
//     down: Option<Rcc<SkipNode<T>>>,
//     data: Rcc<T>,
// }

// impl<T: PartialOrd> SkipNode<T> {
//     pub fn new(t: T) -> Self {
//         SkipNode {
//             right: None,
//             down: None,
//             data: rcc(t),
//         }
//     }

//     pub fn insert(&mut self, data: T) -> Option<Rcc<SkipNode<T>>> {
//         // bigger than right then go right
//         if let Some(ref mut rt) = self.right {
//             if data > *rt.borrow().data.borrow() {
//                 rt.borrow_mut().insert(data);
//                 return;
//             }
//         }

//         // if less then  right go  left
//         if let Some(ref dw) = self.down {
//            return match dw.borrow_mut().insert(data) {
//             Some(child) => match rand::random::<bool>(){
//                 true=> {
//                     let data = child.borrow().data.clone();
//                     let nn = SkipNode {

//                     }
//                 },
//                 false=>None,
//             }
//            }
//             return;
//         }

//         // should be before right , at bottom node
//         let mut nn = SkipNode::new(data);
//         nn.right = self.right.take();
//         self.right = Some(rcc(nn));
//     }
// }

// fn main() {
    
// }
