extern crate markov; // external markov crate (also added as a dependency)
use crate::Result;
use markov::Chain;
/// documentation for markov crate: https://crates.io/crates/markov

/// train model method to create a new markov chain and train it on the data parameter
pub fn train_model(genre_sequence: &Vec<Vec<u32>>) -> Result<Chain<u32>> {
    let mut chain = Chain::of_order(3); // create a new markov chain of order three

    for sequence in genre_sequence {
        // feed it the seqeunce from each file
        chain.feed(sequence);
    }
    Ok(chain)
}

pub fn predict_sequence(chain: Chain<u32>, iterations: usize) -> Result<Vec<u32>> {
    let mut predicted_sequence: Vec<u32> = Vec::new(); // create predicted sequence

    // predict!
    for line in chain.iter_for(iterations) {
        for elem in line {
            predicted_sequence.push(elem);
        }
    }
    Ok(predicted_sequence)
}
