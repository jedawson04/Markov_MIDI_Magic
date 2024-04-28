// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom

// https://crates.io/crates/midi-reader-writer docs for midi-reader-writer
// example from doc page

// illustrates the steps that could typically be used in an application that transforms midi data.

// use midi_reader_writer::{ -- remove this totally -- for now.
//     ConvertTicksToMicroseconds, ConvertMicroSecondsToTicks,
//     //midly_0_5::{exports::Smf, merge_tracks, TrackSeparator},
// };

enum _Duration {
    // not sure if this is how we'd like to store stuff, but I think we want an enum for Duration
    Half,
    Quarter,
    Eighth,
    Sixteenth,
} // this might go in parsing or markov, I'm not sure

use midly::{Smf, TrackEvent}; // because the stupid other one isn't working :(
use std::{error::Error, fs};

pub fn _example() -> Result<(), Box<dyn Error>> {
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
pub fn from_midi(input_filepath: &str) -> Result<(), Box<dyn Error>> {
    let bytes = fs::read(input_filepath)?; // use read to convert filepath to bytes
    let input_midi_file = Smf::parse(&bytes)?; // use parse to create a midi object

    // store longest track with it's num
    let (mut num, mut longest_track): (usize, &Vec<TrackEvent<'_>>) = (0, &Vec::new());
    for (i, track) in input_midi_file.tracks.iter().enumerate() {
        if track.len() > longest_track.len() {
            (num, longest_track) = (i, track);
        }
        println!("track {} has {} events", i, track.len()); // print each track and num of events
    }

    // not sure if we need num rn
    println!(
        "track {num} is the largest track with {} events :-)",
        longest_track.len()
    );

    // for now just work with the track with the most events and print it.
    for event in longest_track.iter().take(5) {
        // print the first 5 events of the longest track
        println!("{event:?}");
    }
    let _object: Vec<String> = Vec::new(); // whatever object we want to use to store our relevant information from events

    // right now we only want kind: Midi maybe with channel: 0, NoteOn { key: } and NoteOff { key:}

    // first 15 events rn for reference:
    // TrackEvent { delta: u28(0), kind: Midi { channel: u4(0), message: ProgramChange { program: u7(0) } } }
    // TrackEvent { delta: u28(4), kind: Midi { channel: u4(0), message: NoteOn { key: u7(63), vel: u7(54) } } }
    // TrackEvent { delta: u28(329), kind: Midi { channel: u4(0), message: Controller { controller: u7(64), value: u7(0) } } }
    // TrackEvent { delta: u28(7), kind: Midi { channel: u4(0), message: NoteOn { key: u7(62), vel: u7(47) } } }
    // TrackEvent { delta: u28(7), kind: Midi { channel: u4(0), message: NoteOn { key: u7(65), vel: u7(58) } } }
    // TrackEvent { delta: u28(15), kind: Midi { channel: u4(0), message: NoteOff { key: u7(63), vel: u7(64) } } }
    // TrackEvent { delta: u28(17), kind: Midi { channel: u4(0), message: Controller { controller: u7(64), value: u7(127) } } }
    // TrackEvent { delta: u28(38), kind: Midi { channel: u4(0), message: NoteOff { key: u7(62), vel: u7(64) } } }
    // TrackEvent { delta: u28(100), kind: Midi { channel: u4(0), message: Controller { controller: u7(64), value: u7(0) } } }
    // TrackEvent { delta: u28(5), kind: Midi { channel: u4(0), message: NoteOn { key: u7(67), vel: u7(60) } } }
    // TrackEvent { delta: u28(8), kind: Midi { channel: u4(0), message: NoteOff { key: u7(65), vel: u7(64) } } }
    // TrackEvent { delta: u28(1), kind: Midi { channel: u4(0), message: NoteOn { key: u7(63), vel: u7(52) } } }
    // TrackEvent { delta: u28(4), kind: Midi { channel: u4(0), message: NoteOn { key: u7(61), vel: u7(49) } } }
    // TrackEvent { delta: u28(17), kind: Midi { channel: u4(0), message: Controller { controller: u7(64), value: u7(127) } } }

    // pair note on with offs to use enum to round duration -- rn chopin is much too complicated LOL. Start simple

    // --> parse, convert to desired string and return
    Ok(())
}

// takes in a markov object and returns a midi file
pub fn _to_midi(_predicted_sequence: &str, _output_filename: &str) {
    // parse string and create a midi_file with output_filename
    // let output_midi_file = Smf {
    //     header: input_midi_file.header,
    //     tracks, // will only have one track
    // };
    // output_midi_file.save(output_filename)?;
    todo!();
}
