use std::fs::File;
use std::io::{Read, Write, prelude};
use std::path::{Path};


fn main(){
    let path = Path::new("src/poop.txt");

    // created
    let mut new_file = File::create(&path).expect("not created !");
    
    //writed
    new_file.write_all(b"hi there i am osman").expect("not writed");

    // opening file
    let mut read_file  = File::open(&path).expect("not opend");

    // reading
    let mut content:String = String::new();
    read_file.read_to_string(&mut content).expect("not readed");

    println!("  {}",content);
}


