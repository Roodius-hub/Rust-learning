
/*   [ Declerative Macros ]   
----------------------------------
*/
//############################
/*macro_rules! say_hello {
    () => {
        println!("Hello, world!");  // also println! macro called
    };
}

fn main() {
    say_hello!();  // Expands to: println!("Hello, world!");
}
*/
/*
//############################
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name));
        }
    };
}

create_function!(hello);  // This will create a function called "hello"

fn main() {
    hello();  // Prints "Hello from hello"
}
 */

//################################
/*2nd >  [ Procedural Macros ]  
    ------------------------
*/


/*
    when we use Macro so under the hood  they done some metaprogramming for example ::
    
    #[derive(Debug)]    ===> what they do ? 
    struct User {
        username:String,
        password:String,
        age:u32
    }    

    they impl trait Like that ===> 
    trait Debug For User {
        -----------
        ------
        ---
    }

    :: Complier Dont know they will display your Struct:: then You use Display macro ::
     they what do under the hood :: 
     they do metaprogramming ::
     trait Display for User {
        ---- printing logic or  whatever---
        ----------------
        --------
        ----
     }


*/


// either do this or either do this  => [use::fmt::Display]
// trait Display {
//     fn print() -> String;
// }


#[derive(Debug)]   // => in built one /*   */ 
/*or #[derive(Clone,Copy)] */
struct User {
    username:String,    // value are string they go on heap they not use copy macro use a Clone
    password:String,    // if value bool or number who present on to the stack they will copy and print
    age:u32             // over all need to print  a struct use use::fmt::Display not other one 
}


fn main(){
    let u = User {
        username:String::from("osman"),
        password:String::from("paswihr"),
        age:32
    };

    println!("{:?}", u);  // debug macro who impl a Debug Trait under the hood 
}



