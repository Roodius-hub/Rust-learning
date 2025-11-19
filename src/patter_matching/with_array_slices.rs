



fn main() {


    let numbers = [1,2,3,4];
        match numbers {
            [1,2,3,4] => println!("Mathced fully array") , 
            [1,2,3,_] => println!("Not Fully Array"),
            _ => println!("Not found")
        }

        // [Destructuring arrays]
    let arr = [10,20,30];
    let [a,b,c] = arr;

        println!("a: {}, b: {}, c: {}", a,b,c);
        
    // [ Ignoring elements with _ or .. ]
    let arr2 = [3,4,5,6,6];
    match arr2 {
        [first, _, third, ..] => println!("Founded"), 
        [first, ..,end] => println!("first and end are present")
    }

    // [Matching start and end of slices:]
 
    let values: &[i32] = &[1, 2, 3, 4];

    match values {
        [first, .., last] => println!("First = {}, Last = {}", first, last),
        _ => println!("No match"),
    }

}   