

fn main(){
    let mut num1 = 10;
    let mut num2 =  23;

    println!("{},{}",num1,num2);
    swap_call_by_value(num1, num2);
    swap_call_by_ref(&mut num1, &mut num2);
}

// call by value
fn swap_call_by_value(mut num1:i32,mut num2:i32){
    let temp:i32;
    num1 = num2;
    temp = num1;
    num2 = temp;

    println!("{},{}",num1, num2);
}

// call by mut ref
fn swap_call_by_ref(num1:&mut i32,num2:&mut i32){

    *num1 += *num2;

    *num2 += *num1; 

    println!("{},{}",num1, num2);
}