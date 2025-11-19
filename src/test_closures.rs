use std::string;



struct Person {
    first_name:String,
    last_name:String
}

pub fn test_closures(){
    let  add = |a:i32,b:i32| a+b;   // closur and anyno function

    let ans = add(2,-3);
    println!("From Closure 1: {}",ans);

    let new = |x:i32,y:i32| {
        let xy = (x + y) * 2;
        return xy;
    };
   let ans2 =  new(2,4);
   
   let print = |v:i32| println!("multiline closure example in another closure: {}",(ans2 + v)); // closure printing another closure ans or value
    print(2);

    /* How can You Mut Object in Closure */

    let mut  p1 = Person {
        first_name:"osman".to_string(),
        last_name:"saifi".to_string()
    };

    let mut change_name = |new_first_name:&str| p1.first_name = new_first_name.to_string();
    change_name("osman");     // on mutating a struct value in closure struct instence need mut but aslo closure need to be mut
    println!("{}",p1.first_name);
    
    
 }



fn main(){
    test_closures();
}