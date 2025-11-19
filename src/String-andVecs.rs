/*  Box (Heap) , String

*/


fn main(){
    // let s = String::from("osmam");

    let mut  s1 = " osman ".to_string();  // this is  a str not  String slice of String stored in Stack
                                          // we know the size :::  String ka size nhi 

    let p = s1.trim();
    let p = p.to_string();

    s1.push_str("goodbye");

    println!("p == '{}'", p);

    let fstr = "help me find home";
    let ffstr = string_stuff(fstr);
    println!("{}", ffstr);

}


fn string_stuff<'a>(s: &'a str) -> &'a str {
    let n = 0;

    for (n,x) in s.char_indices(){   // -> index with value
        if x == 'f' {
            return &s[n..];
        }
    }
    return s
}   

fn Choose_str(n:i32) -> &'static str {
    match n { 
        0 => "helo",
        1 => "goodbye",
        _ => "other"
    }
}