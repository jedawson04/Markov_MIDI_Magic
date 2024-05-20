pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
mod markov; // module for trainig and predicting markov model
mod parsing; // module for parsing to and from midi files
use std::fs::read_dir;

fn run() -> Result<()> {
    // user stuff...
    let user_specification = vec!["scarlatti/petzgold", "jazz"]; // user selected genre(s)
                                                       // -- TRAINING ON MULTIPLE GENRES MESSES UP THE METRICAL a bit.. lol. but it sounds cool? - maybe we normalize somehow.
    // user selected parameters --
    let (
        num_octaves,
        lowest_allowed_pitch,
        mut current_duration,
        longest_duration,
        melody_pitch_dif,
        markov_order,
        longest_parsed_duration,
    ) = (
        3,      // # allowed octaves
        50,     // lowest allowed pitch -- this has a lot of impact on the melodic contour of the output
        0.0625, // shortest/current duration - this is a 64th note, we can calculate beats by taking 4.0/64
        4.0,    // longest duration - this is a whole note, 4.0/1
        36,     // longest semitone range between consecutive notes
        3,      // markov order
        16.0,   // longest duration we allow to be parsed in beats
    );
    // given a lower and upper dur range, this is how I'm filling them in
    let mut quantized_durations = Vec::new();
    while current_duration <= longest_duration {
        quantized_durations.push(current_duration);
        current_duration *= 2.0;
    }
    println!("{quantized_durations:?}");

    // default filename and initialized vecs
    let (filename, mut training_sequence, mut metricals): (&str, Vec<Vec<u32>>, Vec<u16>) = (
        &format!("./src/creations/{user_specification:?}_order_{markov_order}_creation.mid"),
        Vec::new(),
        Vec::new(),
    );

    for specified_genre in user_specification {
        // path to genre folder
        let directory_path = format!("./src/midi-files-by-genre/{specified_genre}/");
        let genre_files = read_dir(directory_path)?;

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
                longest_parsed_duration,
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
