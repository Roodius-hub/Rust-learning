struct HashTable {
    buckets: Vec<Vec<(String, i32)>>,
    size: usize,
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        let buckets = vec![Vec::new(); size];

        Self { buckets, size }
    }

    fn hash(&self, key: &str) -> usize {
        let mut sum = 0;

        for b in key.bytes() {
            sum += b as usize;
        }

        return sum % self.size; // more small value
    }

    fn insert(&mut self, key: String, value: i32) {
        let index = self.hash(&key); // convert key string into number 
        let bucket = &mut self.buckets[index];
        
        // check  if key value alredy  exist 
        for pair in bucket.iter_mut() {  
            if pair.0 == key {   // because of pair have two indexs
                pair.1 = value;
                return; 
            }
        }
        // othervise insert new key / value
        bucket.push((key,value));
    }
    
    // fn get(&self, key: &str) -> Option<i32> {}
    // fn remove(&mut self, key: &str) -> bool {}
    // fn resize(&mut self) {}
}

fn main() {
    let mut table = HashTable::new(5);
    table.insert("osman".to_string(),10);
    table.insert("noor".to_string(),10);
    table.insert("rizzz".to_string(),10);

    println!("inserted !");
    println!("created {} buckets", table.size);
}
