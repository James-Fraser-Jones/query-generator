use crate::types::*;
use crate::constants::*;
use rand::prelude::*;
use pool::*;

pub fn generate(query_num: usize, word_num: usize, tag_num: usize) -> Vec<Query> {
    let mut rng = thread_rng();
    let pool = Pool::new(&mut rng, word_num, tag_num);

    let mut queries = vec![];
    for _ in 0..query_num { //TODO: track IDs and ensure Dones only target non-done IDs
        let query = match rng.gen_range::<u8, _>(1..=4) {
            1..=2 => generate_add(&mut rng, &pool),     //50%
            3 => generate_done(&mut rng),               //25%
            4 => generate_search(&mut rng, &pool),      //25%
            _ => Default::default(),                    //0%
        };
        queries.push(query);
    }

    for query in &queries {
        println!("{}", query);
    }

    queries
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
                word_pool: Self::generate_pool(rng, word_num),
                tag_pool: Self::generate_pool(rng, tag_num),
            }
        }

        pub fn get_word(&self, rng: &mut ThreadRng) -> &str {
            &self.word_pool[rng.gen_range(0..self.word_num)]
        }

        pub fn get_tag(&self, rng: &mut ThreadRng) -> &str {
            &self.tag_pool[rng.gen_range(0..self.tag_num)]
        }

        fn generate_pool(rng: &mut ThreadRng, num: usize) -> Vec<String> {
            let mut pool = vec![];
            for _ in 0..num {
                pool.push(Self::generate_drop(rng))
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

fn generate_done(rng: &mut ThreadRng) -> Query {
    Query::Done(rng.gen_range::<u64, _>(0..=999))
}

fn generate_search(rng: &mut ThreadRng, pool: &Pool) -> Query {
    match rng.gen_range::<u8, _>(1..=4) {
        1..=3 => guaranteed_search(rng, pool),  //75%
        4 => random_search(rng, pool),          //25%
        _ => Default::default(),                //0%
    }
}

fn guaranteed_search(rng: &mut ThreadRng, pool: &Pool) -> Query {
    Query::Search(vec![])
}

fn random_search(rng: &mut ThreadRng, pool: &Pool) -> Query {
    Query::Search(vec![])
}