//  ### in main file all basics 

/* Loops
    #1 loop    ** 
                loop {
                    n += 1;
                if n > 10 {
                            break;
                        }
                println!("Hello {}", n);
                        }
                println!("all done");
                    }**
    #2 while  ## normal all other lang
    #3 iterative loop $$ custome loop
*/

pub struct Stepper {
    curr: i32,
    step: i32,
    max:i32
}

impl Iterator for Stepper  {
    type Item = i32;
    fn next(&mut self)-> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        return Some(res);
    }
}

fn main(){

    let mut st = Stepper {curr:2,
                          step:3,
                          max:15};
    loop {
        match st.next(){
            Some(v) => println!("loop {}",v),
            None => break, 
        }
    }    

    let mut st1 = Stepper {
        curr:3,
        step:4,
        max:24
    };

    while let Some(n) = st1.next() {
        println!("while: {}!",n);
    }

    let mut it = Stepper {
        curr:1,
        step:10,
        max:50
    };
    
    for i in it {
        println!("{}", i);
    }

}