use std::cell::RefCell;
use std::rc::{Rc};

fn main(){
    let data = RefCell::new(5);
    
    // borroe mutabily
    *data.borrow_mut() += 5;
    println!("{:?}", data);
    println!("{}", data.borrow());


    let mut a = Rc::new(7);
    let b = Rc::clone(&a);

    println!("Total a ref count: {}",Rc::strong_count(&a));   // refernce count
    println!("{}, {}", a,b);

    let c = Rc::clone(&a);
    println!("{}", c);

    // mutliple mut refe also
    let mut  d = Rc::clone(&mut a);
    d = Rc::<i32>::new(3);
    println!("{}", d);
}