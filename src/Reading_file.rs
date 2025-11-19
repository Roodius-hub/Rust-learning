use std::fs::File;
use std::io::{prelude, Read};
use std::path::{self, Path};


fn main(){
    let path = Path::new("src/hello.txt");

    let mut file_data:String  = String::new();

    let mut readed_file = File::open(&path).expect("file not found");
    readed_file.read_to_string(&mut file_data).expect("Can not Read file");
    println!("file detail:\n path: {},\n Content:{}", path.display(), file_data);
}