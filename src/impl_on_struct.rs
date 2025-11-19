use core::fmt;
use std::fmt::{write, Display};

enum Color {    
    White,
    Brown, 
    Black
}

impl Display for Color {
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result {
        let name  = match self {
            Color::Black => "Black",
            Color::Brown => "Brown",
            Color::White => "White"
        };
        write!(f,"{}",name)
    }
}

struct Human {
    skin:Color,
    height:u32,
    weight:u32
}

impl Human {
    fn setskin(&mut self,color:Color) {
         self.skin = color;
    }
}

fn main(){
    let mut h = Human {
        skin:Color::Black,
        weight:32,
        height:177
    };

    h.setskin(Color::Brown);
    println!("{}",h.skin);
}

// trait Shape {
//     fn area(&self) -> u32;
// }

// fn area<T:Shape>(s:T) -> u32 {
//     return s.area();
// }

