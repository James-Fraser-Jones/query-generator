use crate::types::*;
use crate::constants::*;
use pool::*;

use std::cmp;
use std::collections::VecDeque;
use rand::prelude::*;

pub struct Generator {
    next_id: u64,
    active_ids: Vec<u64>,
    pool: Pool,
    add_query_history_num: usize,
    add_query_history: VecDeque<Query>,
}
impl Generator {
    pub fn new(rng: &mut ThreadRng, word_num: usize, tag_num: usize, add_query_history_num: usize) -> Self {
        Generator {
            next_id: 0,
            active_ids: vec![],
            pool: Pool::new(rng, word_num, tag_num),
            add_query_history_num,
            add_query_history: VecDeque::with_capacity(add_query_history_num),
        }
    }

    pub fn get_query(&mut self, rng: &mut ThreadRng) -> Query {
        loop {
            match rng.gen_range::<u8, _>(1..=4) {
                1..=2 => {                          //50%
                    let query = generate_add(rng, &self.pool);
                    self.active_ids.push(self.next_id);
                    self.next_id = self.next_id + 1;
                    if self.add_query_history.len() == self.add_query_history_num { //maintain maximum size of queries by removing oldest element
                        self.add_query_history.pop_front();
                    }
                    self.add_query_history.push_back(query.clone());
                    return query;
                },
                3 => {                              //25%
                    if self.active_ids.len() > 0 { //cannot done if there are no active tasks
                        let index = rng.gen_range(0..self.active_ids.len());
                        let query = generate_done(self.active_ids[index]);
                        self.active_ids.remove(index);
                        return query;
                    }
                },
                4 => {                              //25%
                    if self.add_query_history.len() > 0 {
                        let add_query = self.add_query_history.get(rng.gen_range(0..self.add_query_history.len())).unwrap();
                        let query = generate_search(rng, add_query);
                        return query;
                    }
                },
                _ => return Default::default(),     //0%
            };
        }
    }
}

fn generate_add(rng: &mut ThreadRng, pool: &Pool) -> Query {
    let mut words = vec![pool.get_word(rng).to_owned()];
    let mut tags = vec![];
    let extra_words = match rng.gen_range::<u8, _>(1..=8) {
        1..=4 => 0,                 //50%
        5..=6 => 1,                 //25%
        7 => 2,                     //12.5%
        8 => 3,                     //12.5%
        _ => Default::default(),    //0%
    };
    for _ in 0..extra_words {
        words.push(pool.get_word(rng).to_owned());
    }
    let extra_tags = match rng.gen_range::<u8, _>(1..=8) {
        1..=4 => 0,                 //50%
        5..=6 => 1,                 //25%
        7 => 2,                     //12.5%
        8 => 3,                     //12.5%
        _ => Default::default(),    //0%
    };
    for _ in 0..extra_tags {
        tags.push(pool.get_tag(rng).to_owned());
    }
    Query::Add(words, tags)
}

fn generate_done(n: u64) -> Query {
    Query::Done(n)
}

fn generate_search(rng: &mut ThreadRng, add_query: &Query) -> Query {
    match rng.gen_range::<u8, _>(1..=4) {
        1..=3 => guaranteed_search(rng, add_query),     //75%
        4 => random_search(rng),                        //25%
        _ => Default::default(),                        //0%
    }
}

//search not technically guaranteed since it's possible to have recent "add" query already done when search for it happens
//this becomes less likely later on when recent additions make up smaller number of available items to mark as "done"
fn guaranteed_search(rng: &mut ThreadRng, add_query: &Query) -> Query { 
    if let Query::Add(words, tags) = add_query {
        let matched_words = 1 + match rng.gen_range::<u8, _>(1..=8) {
            1..=4 => 0,                 //50%
            5..=6 => 1,                 //25%
            7 => 2,                     //12.5%
            8 => 3,                     //12.5%
            _ => Default::default(),    //0%
        };
        let matched_words = cmp::min(matched_words, words.len());
        let matched_tags = match rng.gen_range::<u8, _>(1..=8) {
            1..=4 => 0,                 //50%
            5..=6 => 1,                 //25%
            7 => 2,                     //12.5%
            8 => 3,                     //12.5%
            _ => Default::default(),    //0%
        };
        let matched_tags = cmp::min(matched_tags, tags.len());
        let mut search_params = vec![];
        for i in 0..matched_words {
            if rng.gen_range(0..4) == 0 {
                search_params.push(WordOrTag::Word(words[i].to_owned()));
            }
            else {
                search_params.push(WordOrTag::Word(random_subsequence(rng, &words[i])));
            }
        }
        for i in 0..matched_tags {
            if rng.gen_range(0..4) == 0 {
                search_params.push(WordOrTag::Tag(tags[i].to_owned()));
            }
            else {
                search_params.push(WordOrTag::Tag(random_subsequence(rng, &tags[i])));
            }
        }
        search_params.shuffle(rng);
        return Query::Search(search_params)
    }
    Default::default() //unreachable
}

fn random_subsequence(rng: &mut ThreadRng, s: &str) -> String {
    let mut stri = s.to_owned();
    for _ in 0..(rng.gen_range(0..stri.len())) {
        stri.remove(rng.gen_range(0..stri.len()));
    }
    stri
}

fn random_search(rng: &mut ThreadRng) -> Query {
    let generated_words = 1 + match rng.gen_range::<u8, _>(1..=8) {
        1..=4 => 0,                 //50%
        5..=6 => 1,                 //25%
        7 => 2,                     //12.5%
        8 => 3,                     //12.5%
        _ => Default::default(),    //0%
    };
    let generated_tags = match rng.gen_range::<u8, _>(1..=8) {
        1..=4 => 0,                 //50%
        5..=6 => 1,                 //25%
        7 => 2,                     //12.5%
        8 => 3,                     //12.5%
        _ => Default::default(),    //0%
    };
    let mut search_params = vec![];
    for _ in 0..generated_words {
        search_params.push(WordOrTag::Word(random_word(rng)));
    }
    for _ in 0..generated_tags {
        search_params.push(WordOrTag::Tag(random_word(rng)));
    }
    search_params.shuffle(rng);
    return Query::Search(search_params)
}

fn random_word(rng: &mut ThreadRng) -> String {
    let max_length = 7;
    let mut stri = String::new();
    for _ in 0..rng.gen_range::<u8, _>(1..=max_length) {
        stri.push(CHARS[rng.gen_range(0..CHARS.len())]);
    }
    stri
}

mod pool {
    use super::*;

    pub struct Pool {
        word_num: usize,
        tag_num: usize,
        word_pool: Vec<String>,
        tag_pool: Vec<String>,
    }

    impl Pool {
        pub fn new(rng: &mut ThreadRng, word_num: usize, tag_num: usize) -> Pool {
            Pool {
                word_num,
                tag_num,
                word_pool: generate_pool(rng, word_num),
                tag_pool: generate_pool(rng, tag_num),
            }
        }

        pub fn get_word(&self, rng: &mut ThreadRng) -> &str {
            &self.word_pool[rng.gen_range(0..self.word_num)]
        }

        pub fn get_tag(&self, rng: &mut ThreadRng) -> &str {
            &self.tag_pool[rng.gen_range(0..self.tag_num)]
        }
    }

    fn generate_pool(rng: &mut ThreadRng, num: usize) -> Vec<String> {
        let mut pool = vec![];
        for _ in 0..num {
            pool.push(generate_drop(rng))
        }
        pool
    }
    
    fn generate_drop(rng: &mut ThreadRng) -> String {
        let mut drop = SEGMENTS[rng.gen_range(0..=999)].to_owned();
        let extra_segments = match rng.gen_range::<u8, _>(1..=8) {
            1..=4 => 0,                 //50%
            5..=6 => 1,                 //25%
            7 => 2,                     //12.5%
            8 => 3,                     //12.5%
            _ => Default::default(),    //0%
        };
        for _ in 0..extra_segments {
            drop.push('-');
            let extra_hyphens = match rng.gen_range::<u8, _>(1..=20) {
                1..=18 => 0,                //90%
                19 => 1,                    //5%
                20 => 2,                    //5%
                _ => Default::default(),    //0%
            };
            for _ in 0..extra_hyphens {
                drop.push('-');
            }
            drop.push_str(SEGMENTS[rng.gen_range(0..=999)]);
        }
        drop
    }
}