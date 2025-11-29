use std::collections::HashSet;

// Fixed Sliding Window
pub fn max_sum_subarray(nums:&[i32], k:usize) -> i32 {
    if nums.len() < k {
        return 0;
    }
   
   let mut window_sum:i32 = nums[..k].iter().sum(); 
   let mut  max_sum = window_sum;
    
   for i in k..nums.len() {
       window_sum += nums[i] - nums[i - k]; 
       max_sum = max_sum.max(window_sum);
   }
   
    max_sum
}
//Variable Sliding Window
pub fn longest_unique_substring(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect(); 
    let mut set = HashSet::new();
    let (mut left, mut right) = (0,0);
    let mut max_len = 0;
  
    while right < chars.len() {
        if !set.contains(&chars[right]) {
            set.insert(chars[right]);
            max_len = max_len.max(right - left + 1);
            right += 1;
        } else {
            set.remove(&chars[left]);
            left +=  1;
        }
    }
    max_len 
}

fn main(){
    let arr = vec![2,1,3,4,5,5,7];
    let k = 3; 
    let ans = max_sum_subarray(&arr, k);
    println!("max sum of k elements: {}", ans);
        
    let s = "abcabcbb";
    println!("Longest unique SubString length: {}",longest_unique_substring(s));
    
}