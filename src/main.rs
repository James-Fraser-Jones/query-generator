mod types;
mod generator;
mod trie;
mod constants;

use types::{Query, WordOrTag};

fn main() {
    let generated_queries = generator::generate(100, 20, 5);
}
