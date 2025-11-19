use std::cell::RefCell;
use std::rc::Rc;


type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t:T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

pub struct SkipNode<T:PartialOrd> {
    right:Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>
}

fn main() {

}