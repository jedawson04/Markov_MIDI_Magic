extern crate markov; // external markov crate (also added as a dependency)
use markov::Chain;

/// train model method to create a new markov chain and train it on the data parameter
/// documentation for markov crate: https://crates.io/crates/markov
 
pub fn train_model(data: String) { 
    let mut chain = Chain::new(); // create a new markov chain
    chain.feed_str(&data); // use feed_str to feed the chain a str

}

pub fn predict_sequence() { 
    todo!();
}