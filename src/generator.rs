use crate::types::*;
use rand::prelude::*;

pub fn generate(query_num: u64, word_num: u64, tag_num: u64) -> Vec<Query> {
    let mut rng = thread_rng();
    let result = match rng.gen_range(1..=4) {
        1..=2 => "Add", //50%
        3 => "Done", //25%
        4 => "Search", //25%
        _ => "Oh no!", //0%
    };
    println!("{}", result);
    vec![]
}

fn generate_word_pool(){

}

fn generate_add(){

}

fn generate_done(){

}

fn generate_search(){

}

fn guaranteed_search(){

}

fn random_search(){
    
}