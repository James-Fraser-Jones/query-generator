mod types;
mod generator;
mod constants;

use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let rng = &mut rng;

    //read terminal args to initialize generator
    let mut generator = generator::Generator::new(rng, 20, 5, 30);

    for _ in 0..99 { //loop 0..number_of_queries
        let query = generator.get_query(rng); //only thing which will grow is array of "active" indices
        println!("{}", query); //write to (buffered) file
    }
}
