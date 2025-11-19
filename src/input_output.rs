use std::io;

fn main(){
    let mut value:String = String::new();
    let mut age:f32;

    io::stdin().read_line(&mut value).unwrap();
    age = value.trim().parse::<f32>().unwrap();

    println!("{}",age);

}