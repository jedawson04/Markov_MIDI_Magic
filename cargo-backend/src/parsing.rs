// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom

use midly::{MidiMessage, Smf, TrackEvent, TrackEventKind::Midi, Timing::Metrical};
use std::fs;
use crate::Result;

#[derive(Debug)]
pub enum Note {
    // each note is either a key (with specified value) or a rest
    Key(u8),
    Rest,
}

// this file will have 2 main pub fns: from_midi() and to_midi() which will convert from and to midi files to our Markov model state
// takes in a midi file --> parses, converts to desired format and returns
pub fn from_midi(input_filepath: &str) -> Result<Vec<(Note, f32)>> {
    let bytes = fs::read(input_filepath)?; // use read to convert filepath to bytes
    let input_midi_file = Smf::parse(&bytes)?; // use parse to create a midi object
                                               // use track with most events
    let mut this_metrical: u32 = 0;
    if let Metrical(u15) = input_midi_file.header.timing {
        let conversion_metrical: u16 = u15.into(); // converts metrical to a u16
        this_metrical = u32::from(conversion_metrical); // casts metrical as a u32
    }
    let mut longest_track: &Vec<TrackEvent<'_>> = &Vec::new();
    for track in input_midi_file.tracks.iter() {
        if track.len() > longest_track.len() {
            longest_track = track;
        }
    }
    let mut note_sequence: Vec<(Note, f32)> = Vec::new(); // store desired notes in a sequence with num of ticks
                                                          // initalize variables for parsing midi
    let (mut current_note_val, mut ticks_since_on, mut rest_ticks, mut current_note) =
        (128, 0, 0, false);

    for event in longest_track.iter() {
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
                if (-24..24).contains(&pitch_difference) {
                    // println!("This note IS going to become our current note.");
                    current_note_val = note;
                    current_note = true;
                    // add a rest to note_sequence if applicable
                    if rest_ticks != 0 {
                        let beat_length: f32 = (rest_ticks as f32 + delta as f32) / this_metrical as f32;
                        note_sequence.push((Note::Rest, beat_length));
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
                    let beat_length: f32 = (ticks_since_on as f32 + delta as f32) / this_metrical as f32;
                    note_sequence.push((Note::Key(current_note_val), beat_length));
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

fn _convert_to_hash(_note: u8, _beats: f32) { // converts to hashed number for markov model
    todo!();
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

pub fn tuples_to_nums(notes_and_lengths: Vec<(Note, f32)>, num_of_octaves: u32, starting_pitch: u32, lengths_to_include: Vec<f32>) -> Vec<u32> {
	let mut new_note_sequence: Vec<u32> = Vec::new();

    // for each note duration pair in our previous vector
	for (note, duration) in notes_and_lengths {
    	let mut normalized_pitch_val: u32;

        // setting normalized pitch value to the initial. 
    	match note {
        	Note::Key(pitch) => {
            	normalized_pitch_val = pitch as u32 - starting_pitch;
            	while normalized_pitch_val < 1 {
                	normalized_pitch_val += 12;
            	}
            	while normalized_pitch_val > (num_of_octaves * 12) {
                	normalized_pitch_val -= 12;
            	}
        	},
        	Note::Rest => normalized_pitch_val = 0,
    	}

    	let mut length_difference: f32 = f32::MAX;
        let mut closest_length: f32 = -1.0;
        let mut length_multiplier: u32 = u32::MAX;
        // loop determines the closest quantized length we want to look at
    	for possible_length in &lengths_to_include {
            if length_difference > (duration - possible_length).abs() {
                length_difference = duration - possible_length;
                closest_length = *possible_length;
            }
    	}

        // this loop sets the length multiplier based on the closest length
        for i in 0..lengths_to_include.len() - 1 {
            if closest_length - lengths_to_include[i] == 0.0 {
                length_multiplier = i as u32;
            }
        }

        // if length multiplier is still initial value, we've got a problem
        if length_multiplier == u32::MAX {
            println!("SOMETHING HAS GONE WRONG. THIS SHOULD'VE BEEN UPDATED TO BE A SMALLER NUMBER.")
        }

        new_note_sequence.push(normalized_pitch_val + 12 * num_of_octaves * length_multiplier);
	}

	new_note_sequence
}



#[cfg(test)]
mod test {
    use super::{tuples_to_nums, Note};


    #[test]
    fn do_simple_vals_parse_right() {
        let example_list: Vec<(Note, f32)> = vec![(Note::Key(56), 0.25472), (Note::Key(76), 1.25472), (Note::Key(63), 0.65472)];

        // 6 + 36 * 0, 26 + 36 * 2, 13 + 36 * 1
        let correct_list = vec![6, 98, 49];

        assert_eq!(correct_list, tuples_to_nums(example_list, 3, 50, vec![0.25, 0.5, 1.0, 2.0, 4.0]));
    }

    fn rests_of_different_lengths() {
        todo!();
    }

    fn testing_super_small_and_large_lengths() {
        todo!();
    }

    fn different_num_octaces() {

    }

    fn random_starting_notes() {

    }
}