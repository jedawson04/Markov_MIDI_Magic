// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom

use midly::{MidiMessage, Smf, TrackEvent, TrackEventKind::Midi};
use std::{error::Error, fs};

#[derive(Debug)]
pub enum Note {
    // each note is either a key (with specified value) or a rest
    Key(u8),
    Rest,
}

// this file will have 2 main pub fns: from_midi() and to_midi() which will convert from and to midi files to our Markov model state
// takes in a midi file --> parses, converts to desired format and returns
pub fn from_midi(input_filepath: &str) -> Result<Vec<(Note, u32)>, Box<dyn Error>> {
    let bytes = fs::read(input_filepath)?; // use read to convert filepath to bytes
    let input_midi_file = Smf::parse(&bytes)?; // use parse to create a midi object
                                               // use track with most events
    let mut longest_track: &Vec<TrackEvent<'_>> = &Vec::new();
    for track in input_midi_file.tracks.iter() {
        if track.len() > longest_track.len() {
            longest_track = track;
        }
    }
    let mut note_sequence: Vec<(Note, u32)> = Vec::new(); // store desired notes in a sequence with num of ticks
                                                          // initalize variables for parsing midi
    let (mut current_note_val, mut ticks_since_on, mut rest_ticks, mut current_note) =
        (128, 0, 0, false);
    for event in longest_track.iter(){
        // if event.kind is Midi check if we have a NoteOn/Off event
        if let Midi { message, .. } = event.kind {
            let (note, delta): (u8, u32);
            match message {
                // store note, delta tuple
                MidiMessage::NoteOff { key, .. } | MidiMessage::NoteOn { key, .. } => {
                    (note, delta) = (u8::from(key), u32::from(event.delta));
                }
                _ => continue, // if we don't have a NoteOn or NoteOff event, continue
            }
            // println!(
            //     // "The events note val is: {}, the current note val is: {} ",
            //     note, current_note_val
            // );
            if !current_note {
                // println!("\tWe do not have a current note.");
                // set the first note
                if current_note_val == 128 && (50..75).contains(&note) {
                    // println!("We are setting the very first current note.");
                    current_note = true;
                    current_note_val = note;
                }
                // sets note if we are within an octave
                let pitch_difference: i32 = note as i32 - current_note_val as i32;
                if (-12..12).contains(&pitch_difference) {
                    // println!("This note IS going to become our current note.");
                    current_note_val = note;
                    current_note = true;
                    // add a rest to note_sequence if applicable
                    if rest_ticks != 0 {
                        note_sequence.push((Note::Rest, rest_ticks + delta));
                        rest_ticks = 0;
                    }
                } else {
                    // println!("This note is not going to become our current note.");
                    // increment rest ticks
                    rest_ticks += delta;
                }
            } else {
                // println!("We do have a current note");
                if note == current_note_val {
                    // println!("This event IS the desired Note Off signal.");
                    // add note to sequence
                    note_sequence.push((Note::Key(current_note_val), ticks_since_on + delta));
                    // reset ticks and set current_note to false
                    current_note = false;
                    ticks_since_on = 0;
                } else {
                    // println!("This event is not the desired Note Off signal.");
                    // increment ticks since on
                    ticks_since_on += delta;
                }
            }
            // println!();
        }
    }
    Ok(note_sequence)
}
// there is some weird stuff w tick lengths in the thousands/tens of thousands, but it could be legit

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
// other options/ideas for identifying duration of each note
// TO CALCULATE DURATIONS -- FROM EZRA
// CREATE A HASHMAP CALLED ON - of all notes currently on,
// whenever we hit another tick -- if it is an on, update all the durations by delta
// if it is an off update durations by deltas

// CREATE A HASHMAP OF NOTES -- ALSO FROM EZRA
// every tick update the duration for every note
// when a note is turned on or off reset its duration
// this method allows for us to count both rests and note length even if rests are unused
// 30 - 90 might be a good cutoff range for markov model.. -- depends on training data.

// if we hit a note on or off print it with it's delta --
