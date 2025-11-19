
use std::{cmp::PartialOrd, fmt::Debug};
/*
    [PartialOrd] => is  trait   like Display or custom trai 
    trait PartialOrd {
        ........
        .....
        ..
    } 
*/

pub fn bubble_sort<T:PartialOrd + Debug>(v:&mut [T]) -> bool{
    for _ in 0..v.len(){
        // let mut sorted = false; 
        println!("{:?}",v); 
        for i in  0..v.len()-1 {
            if v[i] > v[i+1]  {
                v.swap(i,i+1);
                // sorted = true;    // reapeating then all sorted
            }
        }
    }
    return true;
}





fn main(){
    
    let mut v = vec![4,6,1,8,11,13,3];
     let ans:bool =  bubble_sort(&mut v);
    println!("{}",ans);
    debug_assert_eq!(v,vec![1,3,4,6,8,11,13]);
}   