
// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom

// https://crates.io/crates/midi-reader-writer docs for midi-reader-writer
// example from doc page 

// illustrates the steps that could typically be used in an application that transforms midi data. 

use midi_reader_writer::{
    ConvertTicksToMicroseconds, ConvertMicroSecondsToTicks,
    //midly_0_5::{exports::Smf, merge_tracks, TrackSeparator},
};

enum _Duration { // not sure if this is how we'd like to store stuff, but I think we want an enum for Duration
    Half, 
    Quarter, 
    Eighth, 
    Sixteenth,
} // this might go in parsing or markov, I'm not sure

use midly::Smf; // because the stupid other one isn't working :(
use std::{fs, error::Error, convert::TryFrom};
pub fn example<'a>(input_filename: &str, output_filename: &str)-> Result<(), Box<dyn Error>> {
    // Read the midi file
    
    let bytes = fs::read("chwapsth.mid")?;
    let input_midi_file = Smf::parse(&bytes)?;
    // let smf = Smf::parse(include_bytes!("./genres/chwapsth.mid"))?; -- one line
   
    for (i, track) in input_midi_file.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
    }

    // these only work if we get midi_reader_writer::midly_0_5 to work :/
    // let mut ticks_to_microseconds = ConvertTicksToMicroseconds::try_from(input_midi_file.header)?;
    // let mut microseconds_to_ticks = ConvertMicroSecondsToTicks::from(input_midi_file.header);
    // let mut separator = TrackSeparator::new();

    // Iterate over the events from all tracks:
    // for (ticks, track_index, event) in merge_tracks(&input_midi_file.tracks) {

    //     // Convert the ticks to microseconds:
    //     let microseconds = ticks_to_microseconds.convert(ticks, &event);

        // Do something with the event:

        // transform event to our hashed system 


        // ... <- Insert your code here

        // Convert from microseconds to ticks:
        // let new_ticks = microseconds_to_ticks.convert(microseconds, &event)?;

        // Push the event to the apprmidi_file
    // }
    Ok(())

}

// This crate has a number of cargo features:

// convert-time (enabled by default): converting time stamps from ticks to microseconds and vice versa.
// read (enabled by default): create an iterator over all the tracks of a midi file, merged
// engine-midly-0-5: use version 0.5.x of the midly crate. 

// this file will have 2 main pub fns: from_midi() and to_midi() which will convert from and to midi files to our Markov model state

// takes in a file and parses it into a markov object trainable string
pub fn from_midi(input_filename: &str) -> Result<(), Box<dyn Error>> {
    // these don't work rn but ideally something like this 
    let bytes = fs::read(input_filename)?;
    let _input_midi_file = Smf::parse(&bytes)?;
    // --> parse and convert to desired string
    todo!();
}

// takes in a markov object and returns a midi file
pub fn to_midi(predicted_sequence: &str, output_filename: &str) { 
    // parse string and create a midi_file with output_filename
    // let output_midi_file = Smf {
    //     header: input_midi_file.header,
    //     tracks, // will only have one track
    // };
    // output_midi_file.save(output_filename)?;
    todo!();
}