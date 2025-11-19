// outside module
mod modules;   // public in lib file
use modules::multi::multi;   // module dir => mod.rs have multi mod file => multi funtion

// local mod in file
mod cal {
    pub fn sum(a:u32,b:u32) -> u32 {
        return a + b;
    }
}


fn main(){
    let ans = cal::sum(3, 3);
    println!("{}",ans);

    let new =  multi(3, 4);

    println!("{}",new);

}