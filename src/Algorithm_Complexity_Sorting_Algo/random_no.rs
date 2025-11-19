use std::sync::Mutex;
// use once_cell::sync::Lazy;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGene> =  Mutex::new(RandGene::new(34052));
}

pub fn rand(max:usize) -> usize {
    return RG.lock().unwrap().next(max);
}

pub struct RandGene {
    curr:usize,
    mul:usize,
    inc:usize,
    modulo:usize
}

impl RandGene {
    pub fn new(curr:usize) -> Self {
        RandGene {
            curr,
            mul:56394237,
            inc:34642349,
            modulo:2325454456
        }
    }

    pub fn next(&mut self,max:usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        return self.curr % max;
    }
}

#[warn(dead_code)]
fn main(){

    // let mut r = RandGene::new(12);

    // for _ in 0..100 {
    //         println!("--{}", r.next(100));
    // }

    // panic!();
}