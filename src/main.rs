mod types;
mod generator;

fn main() {
    // let generated_queries = vec![
    //     Query::Add(
    //         vec![
    //             "hello".to_string(),
    //             "world".to_string(),
    //         ], 
    //         vec![
    //             "these".to_string(),
    //             "are".to_string(),
    //             "the".to_string(),
    //             "tags".to_string(),
    //         ]
    //     ),
    //     Query::Done(4),
    //     Query::Search(vec![
    //         WordOrTag::Word("hello".to_string()), 
    //         WordOrTag::Tag("world".to_string()),
    //         WordOrTag::Word("bello".to_string()),
    //         WordOrTag::Tag("burld".to_string())
    //     ])
    // ];

    let generated_queries = generator::generate(100, 20, 5);

    for query in generated_queries {
        println!("{}", query);
    }

    // println!("{}", constants::SEGMENTS[11]);
    // println!("{}", constants::LETTERS[5]);
}
