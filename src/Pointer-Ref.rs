/*
    self, &self &mut self,mut self
*/

//## [self] => I take full ownership of the object
#[derive(Debug, Clone)]

struct User {
    name: String,
}

impl User {
    fn consume(self) {
        println!("Goodbye, {}", self.name);
    }
}

fn main() {
    let u = User { name: String::from("Osman") };
    u.consume(); // u is MOVED here
    // u can't be used again
}

//######################################################
// &self :: > I borrow it to read only

impl User {
    fn show(&self) {
        println!("User: {}", self.name);
    }
}

fn main1() {
    let u = User { name: String::from("Osman") };
    u.show(); // just looks
    println!("{}", u.name); // still usable
}


//###################################################
// &mut self :: > I borrow it to edit
impl User {
    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
}

fn main() {
    let mut u = User { name: String::from("Osman") };
    u.rename(String::from("OsDev"));
    println!("{}", u.name); // now "OsDev"
}

//###############################################
// mut  self :: > I take it, change it, and then return or drop it
impl User {
    fn into_upper(mut self) -> Self {
        self.name = self.name.to_uppercase();
        self
    }
}




