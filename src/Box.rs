struct Point {
    x: i32,
    y: i32,
}

fn box_struct() {
    let p = Box::new(Point { x: 1, y: 2 });
    println!("x = {}, y = {}", p.x, p.y);
}
//######################
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}




fn main(){
    let  a = Box::new(5);
    println!("{}",a);

    box_struct();

}