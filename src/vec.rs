
fn new_int(){
    let mut  my_int:Vec<u32> = Vec::new();
    my_int.push(2);
    my_int.pop();
    my_int.push(56);
        my_int.push(43);
            my_int.push(56);
                my_int.push(5);
                    my_int.push(23);
                        my_int.push(34);

    println!("{}",my_int[0]);
    println!("{:?}",&(&my_int).as_slice()[0..3]);

    println!("{:?}",my_int.get(1))

}

fn vec_string(){
     let names:Vec<&str> = vec!["osman","rizwan","noname","noor"];

    for n in names.as_slice() {    //we added as_slice because ownership moved in iteration in n or clone 
        println!("String Ref {} ....",n);
    }

    println!("{:?}",names);
}

#[derive(Debug,Clone)]
struct Car {
    manufacturer:String,
    model:String
}

pub fn vec_car(){
    let  c =Car{manufacturer:"porsch".to_string(),model:"Panamera".to_string()};
    let car_list = vec![c.model;10];    
    println!("{:?}",car_list);

    let mut  car_list:Vec<Car> = vec![];
    let c1 = Car {manufacturer:"honda".to_string(),model:"hondaCity".to_string()};
    for _ in  1..=100u8 {    // 1 to 100 size u8 pushed to vector
        car_list.push(c1.clone());
    }

    for c in car_list.clone() {   // printing
        println!("{:?}",c);
    }

    println!("length: {}",car_list.len());
    println!("capacity: {}",car_list.capacity());

}

// ## 2
pub fn vec_list2(){
    let car1 = Car {manufacturer:"loona".to_string(),model:"noona".to_string()};

    let mut car_vec:Vec<Car>  = vec![];
    for _ in 1..=100u8 {
        car_vec.push(car1.clone());
    }

    let  car2 = Car {manufacturer:"hyundai".to_string(),model:"suzuki".to_string()};
    let mut  car_vec2: Vec<Car>  = vec![];
    for _ in 1..10u8 { 
        car_vec2.push(car2.clone());
    }

    car_vec.append(&mut car_vec2);   // moved the vec 1 values in vec 2

        // pushing In Specific index
        car_vec.insert(3,Car{manufacturer:"shine".to_string(),model:"dero".to_string()});

        for c in car_vec.clone() {
            println!("{:?}",c);
        }
        let  check = |c:&Car|  {if c.manufacturer == "shine"{ return true;} else {return false;}};
        car_vec.retain(check);

    println!("car_vec length: {}",car_vec.len());
        println!("car_vec2 length: {}",car_vec2.len());
            println!("car_vec capacity: {}",car_vec.capacity());
                println!("car_vec2 capacity: {}",car_vec2.capacity());
                 print!("{:?}",car_vec.get(0).unwrap());


}            


fn main(){
    new_int();
    vec_string();
    vec_car();
    vec_list2();
}