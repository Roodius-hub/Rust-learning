/*
    trait => type / interface
    exmaple => [std::fmt::Display]
    exmaple of TS
    [
        interface Shape {
            area() => number
        }

        class Circle Implement Shape {
            area(){
                this.w * this.h
            }
        } 
    ]

*/
trait Shape {   //   [such a funcion need who return f32 valule]  its like a type decleration of rs function like [d.ts]
    fn area(&self) -> f32;  // Shape of your final Thing
}
/*
    [every funtion need to be in trait who implemented for struct]
*/
struct Rect {
    w:f32,
    h:f32
}

impl Shape for Rect { // shape impl for Rect
    fn area(&self) -> f32 {    // this function need to be in trait
        return self.w * self.h
    }
}

struct Circle {
    r:f32
}

impl Shape for  Circle {   // shape impl for Circle
    fn area(&self) -> f32 { //  this function need to be in trait
        return self.r * self.r
    }
}

// new shape
struct Cylender {
    length:f32,
    radius:f32,
    width:f32
}

impl Shape for Cylender {
    fn area(&self) -> f32 {
        return self.width * self.radius + self.length /2.0; // this function need to be in trait
    }
}


fn Circle_area<T:Shape>(s:T){
    println!("circle area:  {}", s.area());
 }

fn Rect_area(r:impl Shape) {
         println!("rectangle area: {}",r.area());
}

fn Cylender_area<T:Shape>(cy:T){
    println!("cylender area: {}",cy.area());
}

fn main(){
    let r =  Rect {
        w:10.4,
        h:23.3
    };

    let c = Circle {
        r:21.2
    };

    let cy = Cylender {
        length:23.4,
        radius:34.2,
        width:21.4
    };

    Circle_area(c);

     Rect_area(r);

    Cylender_area(cy);
}   
