
struct User {
    name: Option<String>,
}

fn new() {
    let mut user = User { name: Some("Osman".to_string()) };

    let name = user.name.take(); // Move out the name
    println!("{:?}", name);      // Some("Osman")
    println!("{:?}", user.name); // None
}



fn main() {
    let mut opt = Some(42);

    let value = opt.take();   // take out the 42
    new();  
    println!("{}",value.unwrap());   // they have 42
    println!("{}",opt.unwrap());  // none
}