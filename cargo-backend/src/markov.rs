extern crate markov; // external markov crate (also added as a dependency)
use markov::Chain;
use crate::Result;
/// documentation for markov crate: https://crates.io/crates/markov

/// train model method to create a new markov chain and train it on the data parameter
pub fn train_model(genre_sequence: Vec<Vec<u32>>) -> Result<Chain<u32>>{
    
    let mut chain = Chain::new(); // create a new markov chain
    // examples straight from the docs

    // predicting 5 lines using chain iterators
    for sequence in genre_sequence { // for each file

        chain.feed(sequence);
    }
    Ok(chain)
}

pub fn predict_sequence(chain: Chain<u32>, iterations: usize) -> Result<Vec<u32>> {

    let mut predicted_sequence: Vec<u32> = Vec::new(); // create predicted sequence

    for line in chain.iter_for(iterations) { // predict
        println!("{:?}", line);
        for elem in line { 
            predicted_sequence.push(elem);
        }
    }
    Ok(predicted_sequence)
}
