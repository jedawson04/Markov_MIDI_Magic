pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Generic Result type
mod markov; // module for trainig and predicting markov model
mod parsing; // module for parsing to and from midi files
use std::fs::read_dir;

fn run() -> Result<()> {
    
    for specified_genre in vec!["Classical", "Jazz", "Rnb", "Rock"] { 
        
        // let specified_genre = "Rock";
        let directory_path = format!("./src/midi-files-by-genre/{specified_genre}/"); // path to genre folder

        let filename = &format!("./src/midi-files-by-genre/test/{specified_genre}_creation.mid"); // default filename
        let (num_octaves, lowest_allowed_pitch, quantized_durations) =
            (3, 50, vec![0.0625, 0.125, 0.25, 0.5, 1.0, 2.0, 4.0]); // user selected (?) parameters
        let genre_files = read_dir(directory_path)?;
        let melody_pitch_dif = 12; // currently allowed notes within an octave

        let mut genre_sequence = Vec::new();
        let mut min_metrical = u16::MAX;
        let mut max_metrical: u16 = 0;
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
            if metrical < min_metrical {
                min_metrical = metrical;
            }
            if metrical > max_metrical {
                max_metrical = metrical;
            }

            // convert note sequence into a trainable string
            let hashed_sequence: Vec<u32> = parsing::tuples_to_nums(
                note_sequence,
                num_octaves,
                lowest_allowed_pitch,
                &quantized_durations,
            );
            genre_sequence.push(hashed_sequence);
        }

        let chain = markov::train_model(&genre_sequence)?;

        let predicted_sequence = markov::predict_sequence(chain, genre_sequence.len())?;

        let parsed_sequence = parsing::nums_to_tuples(
            predicted_sequence,
            num_octaves,
            lowest_allowed_pitch,
            &quantized_durations,
        );

        parsing::to_midi(parsed_sequence, filename, (min_metrical + max_metrical) / 2); // create and save a midi file with name filename

    }
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
