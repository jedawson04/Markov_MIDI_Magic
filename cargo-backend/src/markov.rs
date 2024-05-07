extern crate markov; // external markov crate (also added as a dependency)
use markov::Chain;

/// documentation for markov crate: https://crates.io/crates/markov

/// train model method to create a new markov chain and train it on the data parameter
pub fn _train_model(_note_sequence: Vec<String>) {
    let mut chain = Chain::new(); // create a new markov chain

    // examples straight from the docs

    // predicting 5 lines using chain iterators
    chain.feed_str("I like cats and I like dogs.");
    for line in chain.iter_for(5) {
        println!("{:?}", line);
    }
    // using a higher order chain --
    chain = Chain::of_order(2);
    chain.feed_str("I like cats and I like dogs.");
    for line in chain.iter_for(5) {
        println!("{:?}", line);
    }
}

pub fn _predict_sequence() {
    todo!();
}
