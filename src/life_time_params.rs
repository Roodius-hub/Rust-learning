// [life time annotation]

fn main(){
    let str1 = "lifeTime annotation";
    let str2  = "Multiple lifetime annotaion";
    let Gstr = get_greater_str(str1,str2);
    println!("Greater Str is:: {}",Gstr);
}

// example life-time
fn get_greater_str<'a>(str1:&'a str,str2:&'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}

// multiple lifetime
fn main2(){
    let str1 = String::from("lifeTime annotation");
    let Gstr;
    {
        let str2  = "Multiple lifetime annotaion";
        Gstr = get_greater_str2(&str1,&str2);
    }
    
    println!("Greater Str is:: {}",Gstr);
}


fn get_greater_str2<'a, 'b>(str1:&'a str, str2:&'b str) -> &'a str {    /* here we telling to compiler lifetime b is different from a */
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str1;
    }
}
