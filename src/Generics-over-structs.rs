use std::ops::Multi;    

#[derive(Copy,Clone)]
struct Rect<T> {
    w:T,
    h:T
}

impl<T:std::ops::Mul<Output = T> + Copy> Rect<T> {
    
    fn area(&self) -> T {
        return self.w * self.h
    }
}

fn main(){
    let r = Rect {
        w:10,
        h:20
    };

    println!("{}", r.area());
}

fn sum<T>(a:T,b:T) -> T {
    return a+b;
}