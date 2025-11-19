use std::os::linux::raw;


fn option(){
    let maybe_number = Some(10);

    match maybe_number {
        Some(n) => println!("The number is {}", n),
        None => println!("No number found"),
    }
}

fn option1(){
    let a:Option<i32> = Some(34);    // ## Replacement of NULL
    println!("a is: {}",a.unwrap());
    match a  {
        Some(x) => println!("{}",true),
        None => println!("Nothing")
    }
}   

fn option2() -> Option<String> {
    let mut opt1:Option<String> = None;
    opt1 = Some("Hi there".to_string());
    println!("{}", opt1.clone().unwrap());
    return opt1;
}


enum CharaterType {
    Up,
    Down,
    Left,
    Right
}

impl ToString for CharaterType {
    fn to_string(&self) -> String {
        match  self {
           CharaterType::Down => "Down".to_string(),
           CharaterType::Up  => "Up".to_string(),
           CharaterType::Right => "Right".to_string(),
           CharaterType::Left => "Left".to_string(),  
        }.to_string()
    }
}

pub fn game() -> Option<CharaterType> {
    let mut chartype:Option<CharaterType> = None;
    chartype  = Some(CharaterType::Up);
    return chartype;
}

pub fn main() {
    option();
    option1();
   let ans =  option2();
   println!("{}", ans.unwrap());
   let newans = game();
   println!("{}",newans.unwrap().to_string());
}



