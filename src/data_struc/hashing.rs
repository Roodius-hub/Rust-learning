#[derive(Debug)]
struct HashTable {
    buckets: Vec<Vec<(String, i32)>>,
    size: usize,
    count:usize
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        let buckets = vec![Vec::new(); size];

        Self { buckets, size ,count:0}
    }

    fn hash(&self, key: &str) -> usize {
        let mut sum = 0; // this one

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
        
        self.count += 1;
        
        if self.load_factor() > 0.75 {
            self.resize();
        }
        
        
    }
    
    // laod factor method logic
    fn load_factor(&self) -> f64 {
        println!("load factor function Triggered !");
        return self.count as f64 / self.size as f64  // count value as float value / size value as float -> float value
    }
    
    // resize function
    fn resize(&mut self) {  // they  just resize if load factor > then 0.75
        let new_size  = self.size * 2; // we double the size value for  bucket
        let mut new_bucket = vec![Vec::new();new_size];
        println!("resize function Triggered !");
        // now  we apply loop on  bucket
        for bucket in &self.buckets {
            for (key,value) in bucket {
                let mut sum = 0usize;
                let mut power = 1usize;
                let p = 31usize;
                    
                for b in key.bytes() {
                    sum = (sum + (b as usize)* power) % new_size;
                    power = (power * p) % new_size;
                }
                new_bucket[sum].push((key.clone() ,*value));
            }
        }
        self.buckets = new_bucket;
        self.size = new_size;
    }
    
    fn get(&self, key: &str) -> Option<i32> {
        let index =  self.hash(&key);
        let bucket = &self.buckets[index];
            
        for pair in bucket {
            if pair.0 == key {
                return Some(pair.1)
            }
        }
        None
    }
    
    fn remove(&mut self, key: &str) -> bool {
        let index = self.hash(&key);
        let bucket = &mut  self.buckets[index];
            
        for i in 0..bucket.len() {
            if bucket[i].0 == key {
                bucket.swap_remove(i);
                return true
            }
        }
        
        false
    }
    
}

fn main() {
    let mut table = HashTable::new(2);
    table.insert("osman".to_string(),10);
    table.insert("noor".to_string(),7);
    table.insert("rizzz".to_string(),5);
    
    println!("{:?}", table.get("noor"));
    println!("Removed: {}", table.remove("rizzz"));
    println!("{:?}", table.get("rizzz"));
    
    println!("{:?}",table);
}
