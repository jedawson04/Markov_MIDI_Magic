pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
mod markov;
mod parsing; // module for parsing to and from midi files // module for trainig and predicting markov model

fn run() -> Result<()> {
    // calls important methods from modules
    let specific_file = "chopin_ballade_3_(c)lubetsky"; // specific file name
    let path = format!("./src/genres/chopin/{specific_file}.mid"); // keep this the same

    // let _parsed_midi =  assign this to a value when we get it to return one
    match parsing::from_midi(&path) {
        Err(err) => eprint!("{err}"), // propogate errors
        _ => println!("yay"),         // yay!
    };

    markov::train_model("parsed_midi"); // call markov model

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
