/*
    Quick Sort (Basic Definition)

Quick Sort is a divide-and-conquer sorting algorithm that works as follows:

Pick a pivot element from the array.

Partition the array so that:

All elements less than the pivot go to the left.

All elements greater than the pivot go to the right.

Recursively apply the same process to the left and right subarrays.

Combine the results to get a sorted array.

*/

// use std::fmt::Debug;

mod random_no;



pub fn pivott(arr:&mut [u32]) -> usize {
    let mut pivot = random_no::rand(arr.len());
    arr.swap(pivot,0);

    for i in 1..arr.len(){
        if arr[i] < arr[pivot] {
            // move our pivot forward 1 , and put this element before it   [34,2,4,5,6,7,9,8,10,11,12,1];
            arr.swap(pivot+1,i);          // for this swap(arr[pivot], arr[i])   
            arr.swap(pivot,pivot+1);   // this lik in c++ swap(arr[pivot],arr[pivot+1] )  we can also do [p += 1]
            pivot += 1;
        } 
    }
    return pivot;
}

pub fn quick_sort(arr:&mut [u32]){
    if arr.len() < 1 {
        return;
    }

    let  p = pivott(arr);
    // println!("{:?}",p);

    let (a,b) = arr.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);  // mean not inlclude pivote so we doing &mut b[1...]   skip 0 th elelment
}


fn main(){
    let mut arr:[u32;12] = [34,2,4,5,6,7,9,8,10,11,12,1];

    quick_sort(&mut arr);

    for i in 0..arr.len(){
        println!(" {} ", arr[i]);
    }
    panic!();
    // assert_eq!(arr,[1,2,4,5,6,7,8,9,10,11,12,34]);

    /*
       assert => { check on run time condition true or not }
     */
}

// pub fn threaded_quick_sort(arr:&mut [u32]){
//     if arr.len() <= 1 {
//         return;
//     }   

//     let  p = pivott(arr);

//     let (a,b) = arr.split_at_mut(p);

//     // let handle = std::thread::spawn(move || {
//     //     threaded_quick_sort( a);
//     // });
//     threaded_quick_sort( b);

//     handle.join().ok();
// }   
