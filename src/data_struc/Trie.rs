use std::collections::HashMap;


#[derive(Debug,Default)]
struct TrieNode {
    children: HashMap<char,TrieNode>,
    is_word:bool
}

#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
    size: usize // number of words stored
}

impl Trie {
    // create new empty Trie 
    pub fn new() -> Self {
        Trie {
            root : TrieNode::default(),
            size: 0,
        }
    }
    
    // insert 
    pub fn insert(&mut self, word:&str) -> bool {
        if  word.is_empty() {
            if !self.root.is_word {
                self.root.is_word = true;
                self.size  += 1;
                return true;
            } else {
                return false;
            }
        }
        
        let mut node = &mut self.root;
        for ch in  word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        
        if  node.is_word {
            false
        } else {
            node.is_word = true;
            self.size += 1;
            return true;
        }
        
    }
        
    // check duplicate or containe exact word 
    pub fn contains(&self, word:&str) -> bool {
        if word.is_empty() {
            return self.root.is_word;
        }
        
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        return node.is_word
    }
    
    // Check whether any word in the trie starts with the given prefix.
        
    pub fn start_with(&self, prefix:&str) -> bool {
        if prefix.is_empty() {
            return true;  // every trie starts with empty prefix
        }
        
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false
            }
        }
        
        return true;
    }
        
    /// Delete a word. Returns true if the word was found and removed, false otherwise.
    
    pub  fn delete(&mut self, word:&str) -> bool {
        if word.is_empty() {
            if self.root.is_word {
                self.root.is_word = false;
                self.size -= 1;
                return true;
            } else {
                return false;
            }
        }
            
    // recursive helper returns whether we should remove this child from its parent.
    fn delete_rec(node: &mut TrieNode, mut chars: impl Iterator<Item = char>) -> bool {
        if let Some(ch) = chars.next() {
            if let Some(child_node) = node.children.get_mut(&ch) {
                let should_remove_child = delete_rec(child_node, chars);
                
                if should_remove_child {
                    node.children.remove(&ch);
                }
                
                return node.children.is_empty() && !node.is_word;
            } else {
                return false;
            }
        } else {
            if node.is_word {
                node.is_word = false;
                return node.children.is_empty();
            } else {
                return false;
            }
        }
    }
        // we need to know whether the word existed beforehead
        if !self.contains(word) {
            return false;
        }
        
        delete_rec(&mut self.root, word.chars());
        self.size += 1;
        return true;
    }
    
    // is empty 
    pub fn is_empty(&self) -> bool {
        return self.size == 0   
    }
}

fn main() {
    
}