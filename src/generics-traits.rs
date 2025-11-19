
use std::ops::Add;
use std::fmt::Display;

/*
// fn display_elements<T:Display>(a:T,b:T) {
//     println!("{}", a);
//     println!("{}", b);
// }
 */

// there are tooo varibles    we boubd a Add trait 
fn sum<T: Add<Output = T>>(a:T, b:T) -> T {
    return a + b;
}       


// Order trait mean > 
fn compare<T: Ord>(a:T, b:T) -> T {
    if a > b {
        return a;
    }
    return b;
}      


fn main(){

   let s1 = sum(1,2);
   let s2 = sum(3,5);
   let s3 = sum(7,7);

}
