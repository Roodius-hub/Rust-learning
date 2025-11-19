use std::cell::Cell;

/* Tuple Struct */

// #[derive(Debug)]
struct VehicleTuple(String,String,u16);

fn new_VehicleTuple() -> VehicleTuple {
    return VehicleTuple("Honda".to_string(),"honda city".to_string(),2015);
}

pub fn create_VehicleTuple(){
    let result = new_VehicleTuple();
    println!("Manufactures: {0}, model:{1}",result.0,result.1);
    // println!("{:?}",new_VehicleTuple());
}

/* How To mutate Struct */
/* Normal Struct */
#[derive(Debug,Clone)]
struct Person<'p> {
    name:Cell<&'p str>,
    age:u32
}

fn new_person() -> Person<'static> {
    let mut  p1  = Person {
        name:Cell::from("osman"),
        age:32
    };

    p1.name = Cell::from("newosman"); 

    return p1;
}

 
 
fn main(){
    create_VehicleTuple();

    let mut myperson = new_person();
    println!("{}",myperson.name.get());
    myperson.name = Cell::from("bfjsbdfb");
    println!("{}",myperson.name.get());
}


/*
    Cell use For Mutate Specific Property of  Struct 
    use std::cell::Cell;
    Struct Hello {
        Property:Cell<value type>,
        ------
        ----
    }

    fn {
        let newhello = Hello {
            Property : Cell::from("ksknksnkn"),
        }

        newhello.Property = Cell::from("sdkjkfbjsdgf")

        return newhello
    }

    main(){
        let value = fn()
        println("{}",value.Property.get())
    }
    

*/