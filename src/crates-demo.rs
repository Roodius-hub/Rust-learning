use dotenv::dotenv;
use std::env;


fn main(){
    dotenv().ok();
    let var = env::var("NAME");

    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => println!("Error while reading variables")
    }

}