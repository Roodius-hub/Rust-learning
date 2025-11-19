



fn main(){
    let num = -5;
    
    match num {
        n if n > 0 => println!("Positive"),
        n if n < 0 => println!("negative"),
        _ => println!("zero")        
    }

    let num2 = 0;

    match num2 {
        1..=i32::MAX => println!("Positive number"),
        i32::MIN..=-1 => println!("Negative number"),
        0 => println!("Zero"),
    }

    let age  = 18;

    match age {
        n if n % 2 == 0 => println!("Able to vote"),
        n if n % 2 != 0 => println!("under age"),
        _ => println!("unknown number")
    }

    let number = 3;

    match number {
        1 | 2 => println!("one or two not three"),
        3 | 4 |5 => println!("three,Four,five"),
        _ => println!("Something else")
    }

    let dir = Dir::Up;
    
    match dir {
        Dir::Up | Dir::Down => println!("Vertical Directions"),
        Dir::Left | Dir::Right => println!("Horizontal Direction"),
    }

    let score = 98;

    match score {
        90..=100 => println!("grade A"),
        80..=89 => println!("Grade B"),
        70..=79 => println!("Grade C"),
        _ => println!("Grade: D and F")
    }

    let ch = 'g';

    match ch {
        'a'..='z' => println!("Lowercase Character"),
        'A'..='Z' => println!("uppercase character"),
        _ => println!("not a character")
    }

    // Option inside another option

    let value = Some(Some(10));

    let data = value.unwrap().unwrap();
    println!("{}", data);

    match value {
        Some(Some(num)) => println!("Double Some with value: {}", num),
        Some(None) => println!("Outer Some inner None"),
        _ => println!("completley Empty")        
    }

    // nested enum
    let shape = Some(Shape::Rectangle { widht: 34, height: 45 });

    match shape {
        Some(Shape::Rectangle { widht, height }) => println!("Rectangle area w * h = {}",widht*height),
        Some(Shape::Cicle(r)) => println!("Cicle with Radius: {}",r),
        None => println!("no shape here")
    }

     // [destructure structs using pattern matching]

    let p = Point {
        x:0,
        y:39
    };

    match p {
        Point{x:0,y} => println!("on Y axis at {}", y),
        Point{x,y:0} => println!("on X axis at {}", x),
        Point{x,y} => println!("Point is at ({},{})", x,y)
    }

    // [destructure tuples]

    let point = (10,34);
    let (x,y) = point;
    println!("{},{}", x ,y);

    // in match 
    let pair = (0,-5);

    match pair {
        (0,y) => println!("x is zero and y is: {}", y),
        (x,0) => println!("y is zero x is: {}", x),
        (x,y) => println!("x and y is {}, {}" ,x ,y)
    }

    // [if let and while let]
    // if let Used when you want to match a single pattern and ignore all others. 

    let number_some = Some(45);

    if let Some(value) = number_some {
        println!("Got a  number: {}", value)
    } else {
        println!("no value found")
    }

    /* while let
        Used in loops to keep matching as long as the pattern fits */

    let mut numbers = vec![3,4,55,6];

    while let Some(value) = numbers.pop() {
        println!("{}", value);        
    }

    /*
    In Rust, Option<T> is an enum that represents an optional value:
    Some(T) → contains a value of type T
    None → represents the absence of a value
     */

    let num = Some(5);

    match num {
        Some(value)  => println!("Value is: {}",value),
        None => println!("No value Found")       
    }

}

struct Point {
    x:i32,
    y:i32
}

enum Shape {
    Rectangle {widht:u32,height:u32},
    Cicle(u32)   
}

enum Dir {
    Up,
    Down,
    Left,
    Right
}
