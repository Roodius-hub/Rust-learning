/* Mutability & variables & Copy & Cloning

*/

#[derive(Debug, Clone)]

pub struct Person {
    name:String,
    age:i32
} 


#[derive(Debug,Copy,Clone)]
pub struct Point {
    x:i32,
    y:i32,
}

impl Point {
    pub  fn new(x:i32,y:i32) -> Self {
        Point{
            x,y
        }
    }
}

fn Copyconept(){

    let p = Person {
        name:"Matt".to_string(),
        age:32
    };

    let mut p2 = p.clone();
    p2.name.push_str("osman");

    println!("p = {:?}, p2 = {:?}", p,p2);

}

fn main(){
    let mut  a = 1;

    a += 2;
    println!("{}", a); // not changable

    Copyconept();

    let mut  pnt = Point::new(3,4);
    let pn2 = pnt;

    pnt.x += 3;

    println!("p = {:?}, p2 = {:?}", pnt,pn2);
}
