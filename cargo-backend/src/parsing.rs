// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom

use crate::Result;
use midly::{MidiMessage, Smf, Timing::Metrical, TrackEvent, TrackEventKind::Midi};
use std::fs;

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
                        let beat_length: f32 =
                            (rest_ticks as f32 + delta as f32) / this_metrical as f32;
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
                    let beat_length: f32 =
                        (ticks_since_on as f32 + delta as f32) / this_metrical as f32;
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

fn _convert_to_hash(_note: u8, _beats: f32) {
    // converts to hashed number for markov model
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

// covert a (Note, beat) tuple to a single unique num to pass into our markov model
pub fn tuples_to_nums(
    note_sequence: Vec<(Note, f32)>,
    num_octaves: u32,
    lowest_allowed_pitch: u32,
    quantized_durations: Vec<f32>,
) -> Vec<u32> {
    let mut new_note_sequence: Vec<u32> = Vec::new();
    for (note, duration) in note_sequence {
        let mut normalized_pitch_val = 0; // assume rest normalized pitch val

        if let Note::Key(pitch) = note {
            // normalize pitch to lowest_allowed_pitch
            normalized_pitch_val = pitch as u32 - lowest_allowed_pitch;
            // if we have a negative number, add octaves until positive
            while normalized_pitch_val < 1 {
                normalized_pitch_val += 12;
            }
            // if we have a number larger than our max value, subtract octaves
            while normalized_pitch_val > (num_octaves * 12) {
                normalized_pitch_val -= 12;
            }
        }
        // you can probably do this in one line somehow with a map or filter / closure but this works and is pretty readable
        let (mut min_difference, mut closest_duration, mut length_multiplier) =
            (f32::MAX, -1.0, u32::MAX);
        for quantized_duration in &quantized_durations {
            let difference = (duration - quantized_duration).abs();
            if min_difference > difference {
                min_difference = difference;
                closest_duration = *quantized_duration;
            }
        }
        // sets the length multiplier based on the closest length
        for (i, dur) in quantized_durations
            .iter()
            .enumerate()
            .take(quantized_durations.len() - 1)
        {
            if closest_duration - dur == 0.0 {
                length_multiplier = i as u32;
            }
        }
        // throw an error if length multiplier is still initial value
        if length_multiplier == u32::MAX {
            eprintln!(
                "SOMETHING HAS GONE WRONG. THIS SHOULD'VE BEEN UPDATED TO BE A SMALLER NUMBER."
            )
        }
        // hash val depending on normalized pitch, num octaves and length multiplier
        let hashed_pitch_val = normalized_pitch_val + 12 * num_octaves * length_multiplier;
        new_note_sequence.push(hashed_pitch_val);
    }
    new_note_sequence
}

#[cfg(test)]
mod test {
    use super::{tuples_to_nums, Note};

    #[test]
    fn simple_vals_parseing() {
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Key(56), 0.25472),
            (Note::Key(76), 1.25472),
            (Note::Key(63), 0.65472),
        ];

        // 6 + 36 * 0, 26 + 36 * 2, 13 + 36 * 1
        let correct_list = vec![6, 98, 49];

        assert_eq!(
            correct_list,
            tuples_to_nums(example_list, 3, 50, vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn rests_of_different_lengths() {
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Rest, 0.25472),
            (Note::Rest, 1.25472),
            (Note::Rest, 0.65472),
        ];

        // 0 + 36 * 0, 0 + 36 * 2, 0 + 36 * 1
        let correct_list = vec![0, 72, 36];

        assert_eq!(
            correct_list,
            tuples_to_nums(example_list, 3, 50, vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn testing_super_small_and_large_lengths() {
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Rest, 0.0),
            (Note::Rest, 25.0),
            (Note::Key(43), 0.0), // also tests if not value is being parsed correctly
            (Note::Key(89), 58.0), // also tests if not value is being parsed correctly
        ];

        // 0 + 36 * 0, 0 + 36 * 4, 5 + 36 * 0, 27 + 36 * 4
        let correct_list = vec![0, 144, 5, 171];

        assert_eq!(
            correct_list,
            tuples_to_nums(example_list, 3, 50, vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn different_num_octaces() {}
    let example_list: Vec<(Note, f32)> = vec![
        (Note::Rest, 1.0),
        (Note::Key(67), 2.0),
        (Note::Key(33), 0.5), // also tests if not value is being parsed correctly
        (Note::Key(89), 58.0),
        (Note::Key(124), 58.0), // also tests if not value is being parsed correctly
    ];

    // 0 + 36 * 0, 0 + 36 * 4, 5 + 36 * 0, 27 + 36 * 4
    let correct_list = vec![0, 144, 5, 171];

    assert_eq!(
        correct_list,
        tuples_to_nums(example_list, 5, 40, vec![0.25, 0.5, 1.0, 2.0, 4.0])
    );

    // 0 + 36 * 0, 0 + 36 * 4, 5 + 36 * 0, 27 + 36 * 4
    let correct_list = vec![0, 144, 5, 171];

    assert_eq!(
        correct_list,
        tuples_to_nums(example_list, 3, 50, vec![0.25, 0.5, 1.0, 2.0, 4.0])
    );

    #[test]
    fn random_starting_notes() {}
}
