mod types;
mod generator;
mod constants;

use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;

/* examples (don't forget to use release build)
benchmark_5m.txt 5000000 1000 100 1000
benchmark_5k.txt 5000 100 50 100
*/

fn main() -> io::Result<()> {
    //read args
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_owned();
    let wrap_err = |e| io::Error::new(io::ErrorKind::InvalidInput, e);
    let query_num: usize = args[2].parse().map_err(wrap_err)?;
    let word_num: usize = args[3].parse().map_err(wrap_err)?;
    let tag_num: usize = args[4].parse().map_err(wrap_err)?;
    let add_query_history_num: usize = args[5].parse().map_err(wrap_err)?;
    
    //create generator
    let mut rng = thread_rng();
    let rng = &mut rng;
    let mut generator = generator::Generator::new(rng, word_num, tag_num, add_query_history_num);

    //open file and output buffer
    let file = File::create(file_path)?;
    let mut writer = io::BufWriter::new(file);

    //write number of queries as first line
    writeln!(writer, "{}", query_num)?;

    println!("Ready to generate queries");
    //write queries out to specified file
    for _ in 0..query_num {
        let query = generator.get_query(rng); //only thing which will grow is array of "active" indices
        writeln!(writer, "{}", query)?;
    }
    writer.flush()?;
    println!("Done - generated {} queries", query_num);

    Ok(())
}
