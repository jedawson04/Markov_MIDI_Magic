pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
use std::fs;
extern crate markov; // external markov crate (also added as a dependency)
use markov::Chain;

enum _Duration { // not sure if this is how we'd like to store stuff, but I think we want an enum for Duration
    Half, 
    Quarter, 
    Eighth, 
    Sixteenth,
} 
// I think having a separate crate for parsing/processing data would be a good idea.
// Especially if we want to parse MIDI data into notes/duration before we feed it into the markov chain

fn run() -> Result<()> { // reads in data -- can rename this fn later, I'm just copying ideas from labs 

    let training_data: Vec<String> = Vec::new(); // Assuming we'll have some vec of paths to data
    for data in training_data { 
        let _load_data = fs::read_to_string(&data)?.trim(); // load data 
        // do something with the data here
    } 
    todo!(); 
}
/// train model method to create a new markov chain and train it on the data parameter
/// documentation for markov crate: https://crates.io/crates/markov 
fn _train_model(data: String) { 
    let mut chain = Chain::new(); // create a new markov chain
    chain.feed_str(&data); // use feed_str to feed the chain a str

}
fn main() {
    // calls run and handles possible errors
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
