pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
mod parsing; // crate for parsing to and from midi files
mod markov;  // crate for trainig and predicting markov model


fn run() -> Result<()> { // reads in data -- can rename this fn later, I'm just copying ideas from labs 
    let _ = parsing::example("chwapsth.mid", "output_filename");

    Ok(())
}

fn main() {
    // calls run and handles possible errors
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
