pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
mod markov; // module for trainig and predicting markov model
mod parsing; // module for parsing to and from midi files
use std::fs::{read_dir};

fn run() -> Result<()> {
    
    let specified_genre = "rnb"; // user selected specified genre
    let directory_path = format!("./src/midi-files-by-genre/{specified_genre}/"); // keep this the same
    let directory_files = read_dir(&directory_path)?;

    for file in directory_files { // for each midi file in the specified genre
        // get the path 
        let midi_path_buf = file.unwrap().path();
        let midi_path = midi_path_buf.to_str().unwrap();

        // parse midi file to a note sequence
        let note_sequence = parsing::from_midi(&midi_path).unwrap(); 
        println!("{:?}", note_sequence);

        // train markov model
        // markov::train_model(note_sequence); 
    }
    // calls important methods from modules

    Ok(())
}

fn main() {
    // calls run and handles possible errors
    if let Err(err) = run() {
        // this soesn't work?
        eprintln!("{err}");
        std::process::exit(1);
    }
}
