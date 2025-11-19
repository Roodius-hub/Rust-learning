

fn squar(a:i32) -> Option<i32> {
    return Some(a*a);
}

fn main(){
    let number = Some(5);
    let doubleNumber = number.map(|x| x*2);   // map
    println!("changed number: {}", doubleNumber.unwrap());   


    // -----
    let value = Some(4) 
    .and_then(squar)  
    .and_then(squar);  //   ((4*4)*4) => 64

    println!("Answer: {}", value.unwrap());

    //Provides a default value computed by a closure if None.

    let result = None;

    let answer = result.unwrap_or_else(|| {
        println!("No value found");
        404
    });

    println!("result: {}" , answer);

    /*
        For Result<T, E>
        These methods work similarly but also handle errors.
     */

    let res:Result<i32,&str> = Ok(2);
    let squared = res.map(|value| value* value);
    println!("sqaure: {}", squared.unwrap());


    /*
            map_err()
            Transforms only the Err value.
     */

    let result_err:Result<i32,&str> = Err("poops");
    let handled_err = result_err.map_err(|e| e.to_uppercase());
    println!("Error: {}", handled_err.unwrap());


    // -------------------
    let rr: Result<i32, String> = divide(10, 2)
        .and_then(|v| divide(v, 1))
        .and_then(|v| divide(v, 0));

    println!("{:?}", rr);


    // -----------------
    let res: Result<i32, &str> = Err("failed");
    let value = res.unwrap_or_else(|e| {
        println!("Error: {}", e);
        0
    });
    println!("Value = {}", value);


    let a = get_value(Some(34));
    println!("{:?}", a);

    // binding range 
    let numm = 3;
    match numm {
        n @ 1..5 => println!("{}", n),
         n @ 6..=10 => println!("Medium number: {}", n),
         _ => println!("Other number"),
    }

}

fn get_value(opt: Option<i32>) -> Result<i32, String> {
    let val = opt.ok_or("Missing value")?;
    Ok(val * 2)
}


fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}


