// extern crate markov; // external markov crate (also added as a dependency)
// use markov::Chain;
// use crate::Result;
// /// documentation for markov crate: https://crates.io/crates/markov

// /// train model method to create a new markov chain and train it on the data parameter
// pub fn train_model(note_sequence: Vec<String>, chain: Chain<String>) -> Result<Chain<String>>{
//     let mut chain = Chain::new(); // create a new markov chain

//     // examples straight from the docs

//     // predicting 5 lines using chain iterators
//     for tuple in note_sequence {
//         println!("{:?}", tuple);
//         chain.feed_str(&tuple);
//     }

//     // for line in chain.iter_for(5) {
//     // }
//     // // using a higher order chain --
//     // chain = Chain::of_order(2);
//     // chain.feed_str("I like cats and I like dogs.");
//     // for line in chain.iter_for(5) {
//     //     println!("{:?}", line);
//     // }
//     Ok(chain)
// }

// pub fn predict_sequence() {
//     todo!();
// }
