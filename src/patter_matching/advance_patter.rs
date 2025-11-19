enum Message {
    Hello(i32),
}

fn main(){ 
    let num = 7;

    // adding ranges
    match num {
        n @ 1..=5 => println!("Small number: {}", n),
        n @ 6..=10 => println!("Medium number: {}", n),
        _ => println!("Other number"),
    }


    // enum exmaple
    let msg = Message::Hello(435);

    match msg {
        Message::Hello(id @ 1..=10) => println!("ID in range: {}", id),
        Message::Hello(id) => println!("Not in range: {}", id)
    }

    // with array  and 
    let numbers = [1,3,35,5,5];

    for &n in &numbers {
        match numbers {
            n if n % 2 == 0 => println!("Evens: {}", n),
            _ => println!("odd: {}", n)
        }
   }

}