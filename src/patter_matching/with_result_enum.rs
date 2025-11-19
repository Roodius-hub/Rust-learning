
// enum Result<T,E> {
//     Ok(T),
//     Err(E)
// }


fn divide(a:f64, b:f64) -> Result<f64 ,String> {
    if b == 0.0 {
        Err("cannot divide by zero".to_string())
    } else {
        Ok(a/b)
    }
}

fn main(){

    let result = divide(10.0, 7.0);
    match result {
        Ok(res) => println!("{}", res),
        Err(msg) => println!("{}", msg)        
    }


    let answer:Result<i32,&str> = Ok(45);
    if let Ok(res) = answer {
        println!("Success! {}", res);
    }


    let res: Result<i32,&str> = Err("oops");

    res.map_err(|e| println!("Handled Error: {}",e));

}
