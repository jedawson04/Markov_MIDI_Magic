pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
mod markov; // module for trainig and predicting markov model
mod parsing; // module for parsing to and from midi files
use std::fs::read_dir;

fn run() -> Result<()> {
    let specified_genre = "Jazz"; // user selected
                                  // path to genre folder
    let directory_path = format!("./src/midi-files-by-genre/{specified_genre}/");
    // default filename
    let filename = &format!("./src/midi-files-by-genre/test/{specified_genre}_creation.mid");
    // user selected (?) parameters
    let (num_octaves, lowest_allowed_pitch, quantized_durations, melody_pitch_dif, markov_order) = (
        3,
        50,
        vec![0.0625, 0.125, 0.25, 0.5, 1.0, 2.0, 4.0, 60.0],
        36,
        3,
    );
    let genre_files = read_dir(directory_path)?;

    let mut training_sequence = Vec::new();
    let mut metricals = Vec::new();
    for midi_file in genre_files {
        // get the path
        let midi_path_buf = midi_file.unwrap().path();
        let midi_path = midi_path_buf.to_str().unwrap();

        // parse midi file to a note sequence
        let (note_sequence, metrical) = parsing::from_midi(
            midi_path,
            melody_pitch_dif,
            num_octaves,
            lowest_allowed_pitch,
        )
        .unwrap();

        // convert note sequence into a trainable string
        let hashed_sequence: Vec<u32> = parsing::tuples_to_nums(
            note_sequence,
            num_octaves,
            lowest_allowed_pitch,
            &quantized_durations,
        );
        // push sequence and metrical
        training_sequence.push(hashed_sequence);
        metricals.push(metrical);
    }

    let avg_metrical = (metricals.iter().min().unwrap() + metricals.iter().max().unwrap()) / 2;

    let chain = markov::train_model(&training_sequence, markov_order)?;

    let predicted_sequence = markov::predict_sequence(chain, training_sequence.len())?;

    let parsed_sequence = parsing::nums_to_tuples(
        predicted_sequence,
        num_octaves,
        lowest_allowed_pitch,
        &quantized_durations,
    );

    parsing::to_midi(parsed_sequence, filename, avg_metrical); // creates and save midi file as filename

    Ok(())
}

fn main() {
    // calls run and handles errors
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
