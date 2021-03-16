mod types;
mod generator;
mod constants;

use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

/* examples (don't forget to use release build)
benchmark_5m.txt 5000000 1000 100 1000
benchmark_5k.txt 5000 100 50 100
*/

fn main() -> std::io::Result<()> {
    //read args
    let mut args = env::args();
    args.next();
    let file_path = args.next()
    .expect("Error: expected \"file_path\" argument");
    let query_num: usize = args.next()
    .expect("Error: expected \"query_num\" argument").parse()
    .expect("Error: failed to parse \"query_num\" argument");
    let word_num = args.next()
    .expect("Error: expected \"word_num\" argument").parse()
    .expect("Error: failed to parse \"word_num\" argument");
    let tag_num = args.next()
    .expect("Error: expected \"tag_num\" argument").parse()
    .expect("Error: failed to parse \"tag_num\" argument");
    let add_query_history_num = args.next()
    .expect("Error: expected \"add_query_history_num\" argument").parse()
    .expect("Error: failed to parse \"add_query_history_num\" argument");
    
    //create generator
    let mut rng = thread_rng();
    let rng = &mut rng;
    let mut generator = generator::Generator::new(rng, word_num, tag_num, add_query_history_num);

    //open file and output buffer
    let file = File::create(file_path)?;
    let mut writer = LineWriter::new(file);

    //write number of queries as first line
    writer.write(query_num.to_string().as_bytes())?;
    writer.write(&[b'\n'])?;

    //write queries out to specified file
    for _ in 0..query_num {
        let query = generator.get_query(rng); //only thing which will grow is array of "active" indices
        writer.write(&query.to_string().as_bytes())?;
        writer.write(&[b'\n'])?;
    }
    writer.flush()?;

    Ok(())
}
