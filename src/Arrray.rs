 // [accept as mutable]
/*  
pub fn change_array(mut gfg_array:[i32;4]){   // mutable liya idhar change but orignal nhi hoga
  for i in 0..4 {
     gfg_array[i] = 0;
  }
  println!("Changed array {:?}",gfg_array);
}


fn main() {
  let gfg_array = [1,2,3,4];
  change_array(gfg_array);

  print!("Original array {:?}",gfg_array);
} */


// passed by reference

fn main() {
  let mut gfg_array = [1,2,3,4,5];      
  change_array(&mut gfg_array);   // passed as mutable    now => 0,0,0,0,0,0
  print!("Original array {:?}",gfg_array);
}

fn change_array(gfg_array:&mut [i32;5]){     // mutable reference
  for i in 0..5 {
     gfg_array[i] = 0;     // now they will change orignal    0,0,0,0,0,
  }
  println!("Changed array {:?}",gfg_array);
}




