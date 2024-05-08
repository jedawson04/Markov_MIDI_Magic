pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
mod markov; // module for trainig and predicting markov model
mod parsing; // module for parsing to and from midi files

fn run() -> Result<()> {
    // calls important methods from modules
    let specific_file = "51126a_ballade_op_47_no_3_a_flat_(nc)smythe"; // specific file name
    let specified_genre = "classical"; // for now we will hardcode this
    let path = format!("./src/midi-files-by-genre/{specified_genre}/{specific_file}.mid"); // keep this the same

    let note_sequence = parsing::from_midi(&path).unwrap(); // parse midi file to a note sequence
    println!("{:?}", note_sequence);
    // markov::train_model("parsed_midi"); // call markov model

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
