/*
    [let Some pattern matching]
*/


fn main(){
    let v = vec![1,2,2,3,4];

    while let Some(ref v) = Some(&v) {
        println!("{:?}",v);
        break;
    }

}
