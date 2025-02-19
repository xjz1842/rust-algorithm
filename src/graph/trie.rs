
struct Node {
    node: [Option<Box<Node>>; 26],
    end: bool,
}

impl Node {
    fn new () -> Self{
        Node {
            node:[const {None};26],
            end:false
        }
    }
}

struct Trie {
   root: Node,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
           root : Node::new()
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;

        for ch in word.chars() { 
            let c = (ch as u8 - b'a') as usize;
            if  cur.node[c].is_none(){
                cur.node[c] = Some(Box::new(Node::new()));
            }
            cur = cur.node[c].as_mut().unwrap();
        }
        cur.end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut cur = &self.root;

        for ch in word.chars() { 
            let c = (ch as u8 - b'a') as usize;
            if  cur.node[c].is_none(){
                return false;
            }
            cur = cur.node[c].as_ref().unwrap();
        }
        cur.end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;

        for ch in prefix.chars() { 
            let c = (ch as u8 - b'a') as usize;
            if  cur.node[c].is_none(){
                return false;
            }
            cur = cur.node[c].as_ref().unwrap();
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[test]
fn trie_test() {
   let mut obj = Trie::new();
   let word: String= "str".to_string();
   obj.insert(word);
   let word: String= "str".to_string();
   let ret_2: bool = obj.search(word);
   let prefix = "st".to_string();
   let ret_3: bool = obj.starts_with(prefix);
   println!("{} {}",ret_2,ret_3);
}