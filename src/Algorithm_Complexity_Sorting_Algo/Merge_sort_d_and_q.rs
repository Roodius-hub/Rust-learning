
pub fn merge( left:&mut [u32],right:&mut [u32]){
    let mut temp:Vec<u32> = Vec::with_capacity(left.len() + right.len());

    let  (mut i,mut j) = (0,0);

    // element comaprion
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            temp.push(left[i]);
            i += 1;
        } else {
            temp.push(right[j]);
            j += 1;
        }   
    }

    // temp.extend_from_slice(&left[i..]);
    // temp.extend_from_slice(&right[j..]);

    // push remaining elements of left
    while i < left.len() {
        temp.push(left[i]);
        i += 1;
    }

    // push remaining elements of right
    while j < right.len() {
        temp.push(right[j]);
        j += 1;
    }

    for (idx,&val) in temp.iter().enumerate() {
        if idx < left.len() {
            left[idx] = val;
        } else {
            right[idx - left.len()] = val;
        }
    }

}


pub fn merge_sort(arr:&mut [u32]){
    if arr.len() <= 1 {
        return;
    } else {
        let mid = arr.len() / 2;
        let (left,right) = arr.split_at_mut(mid);
        merge_sort(left);
        merge_sort(right);
        merge(left, right);
    }
}



fn main(){
    let mut arr:[u32;10] = [1,3,2,5,4,7,6,9,8,10]; 
    merge_sort(&mut arr);

    for val in arr.iter() {
        print!(" << {} ",val);
    }

}
