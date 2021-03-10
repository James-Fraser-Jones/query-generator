use std::collections::HashMap;

//=========================================DATABASE===============================================
pub struct Database<'a> { 
    records: Vec<(usize, String)>,
    index: Trie<'a>,
    next_id: usize,
}

impl<'a> Database<'a> {
    fn new() -> Database<'a> {
        Database {
            records: vec![],
            index: Trie {
                children: HashMap::new(), //TODO: replace inefficient hashing algorithm
                parent: None,
                level: 0,
                position: 1,
                first_occour: HashMap::new(),
                last_occour: HashMap::new(),
                next: None,
                ids: vec![],
            },
            next_id: 0,
        }
    }

    fn search(&self, subsequence: &str) -> Vec<(usize, &str)> {
        //query trie directly, then do lookup into results using retrieved indices instead code below

        let mut results = vec![];
        for (index, sequence) in &self.records {
            if match_subsequence(sequence, subsequence) {
                results.push((*index, &sequence[..]));
            }
        }
        results
    }

    fn insert(&mut self, new: &str) {
        //insert into trie

        self.records.push((self.next_id, new.to_owned()));
        self.next_id = self.next_id + 1;
    }

    fn delete(&mut self, index: usize) {
        //delete from trie

        self.records.remove(index);
    }
}

struct Trie<'a> {
    children: HashMap<char, Trie<'a>>,
    parent: Option<&'a Trie<'a>>,

    level: usize,
    position: usize,

    first_occour: HashMap<(char, usize), Trie<'a>>,
    last_occour: HashMap<(char, usize), Trie<'a>>,
    next: Option<&'a Trie<'a>>,

    ids: Vec<usize>,
}

//=========================================SIMPLE DATABASE===============================================
pub struct SimpleDatabase { 
    records: HashMap<usize, String>,
    index: SimpleTrie,
    next_id: usize,
}
impl SimpleDatabase {
    fn new() -> SimpleDatabase {
        SimpleDatabase {
            records: HashMap::new(),
            index: SimpleTrie::new(),
            next_id: 0,
        }
    }
    fn search(&self, subsequence: &str) -> Vec<(usize, &str)> {
        let mut results = vec![];
        
        results
    }
    fn insert(&mut self, new: &str) {
        //update trie
        let mut trie = &mut self.index;
        trie.ids.push(self.next_id);
        for b in new.chars() {
            trie = trie.children.entry(b).or_insert(SimpleTrie::new());
            trie.ids.push(self.next_id);
        }
        //update records
        self.records.insert(self.next_id, new.to_owned());
        //increment id counter
        self.next_id = self.next_id + 1;
    }
    fn delete(&mut self, index: usize) {
        //remove from records
        let record = self.records.remove(&index).unwrap();
        //remove from trie
        let mut trie = &mut self.index;
        trie.ids.remove(trie.ids.iter().position(|x| *x == index).unwrap());
        for b in record.chars() {
            trie = trie.children.entry(b).or_insert(SimpleTrie::new()); //super ugly but I don't know what else to do
            trie.ids.remove(trie.ids.iter().position(|x| *x == index).unwrap());
        }
    }
}
struct SimpleTrie {
    children: HashMap<char, SimpleTrie>,
    ids: Vec<usize>,
}
impl SimpleTrie {
    fn new() -> SimpleTrie {
        SimpleTrie {
            children: HashMap::new(),
            ids: vec![],
        }
    }
}

struct BasicTrie {
    //...
    children: HashMap<char, BasicTrie>,
}
impl BasicTrie {
    //...
    fn insert(&mut self, new: &str) {
        let mut trie = self;
        for b in new.chars() {
            trie = trie.children.entry(b).or_insert(BasicTrie {children: HashMap::new()});
        }
    }
}

//=========================================NO DATABASE===============================================

pub struct NoDatabase { 
    records: Vec<(usize, String)>,
    next_id: usize,
}
impl NoDatabase {
    fn new() -> NoDatabase {
        NoDatabase {
            records: vec![],
            next_id: 0,
        }
    }
    fn search(&self, subsequence: &str) -> Vec<(usize, &str)> {
        let mut results = vec![];
        for (index, sequence) in &self.records {
            if match_subsequence(sequence, subsequence) {
                results.push((*index, &sequence[..]));
            }
        }
        results
    }
    fn insert(&mut self, new: &str) {
        self.records.push((self.next_id, new.to_owned()));
        self.next_id = self.next_id + 1;
    }
    fn delete(&mut self, index: usize) {
        self.records.remove(index);
    }
}

fn match_subsequence(sequence: &str, subsequence: &str) -> bool {
    let l = subsequence.len();
    if l == 0 { //prevent unsafe memory access if subsequence ended up being empty slice 
        return true //empty string is technically a subsequence of every string
    }
    let sub = subsequence.as_bytes();
    let mut i = 0;
    for b in sequence.as_bytes() {
        unsafe { //safe because termination is guaranteed before i gets too large
            if b == sub.get_unchecked(i) {
                i = i + 1;
                if i == l {
                    return true
                }
            }
        }
    }
    false
}