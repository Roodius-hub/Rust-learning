

fn main() {
    let x = 10;
    println!("{}",x);

    {
        let x = 45.5;  // scoped shadow var
        println!("{}",x);
    }

    let x = "string";   // shadow var
    println!("{}",x);

}