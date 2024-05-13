// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom

use crate::Result;
use midly::{
    num::u7, Format, Header, MidiMessage, Smf, Timing::Metrical, Track, TrackEvent,
    TrackEventKind::Midi,
};
use std::fs;

#[derive(Debug, PartialEq)]
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
        // only consider Midi events
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
                // set the first note
                if current_note_val == 128 && (50..75).contains(&note) {
                    current_note = true;
                    current_note_val = note;
                }
                // sets note if we are within an octave
                let pitch_difference: i32 = note as i32 - current_note_val as i32;
                if (-24..24).contains(&pitch_difference) {
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
                    // increment rest ticks
                    rest_ticks += delta;
                }
            } else if note == current_note_val {
                // add note to sequence
                let beat_length: f32 =
                    (ticks_since_on as f32 + delta as f32) / this_metrical as f32;
                note_sequence.push((Note::Key(current_note_val), beat_length));
                // reset ticks and set current_note to false
                current_note = false;
                ticks_since_on = 0;
            } else {
                // increment ticks since on
                ticks_since_on += delta;
            }
        }
    }
    Ok(note_sequence)
}

// takes in a markov object and returns a midi file
pub fn to_midi(parsed_sequence: Vec<(Note, f32)>, output_filename: &str) {
    let metrical: u16 = 16929; // i just picked a number... lol
    let single_track_format = Format::SingleTrack;
    let metrical_timing = Metrical(metrical.into());
    let header = Header::new(single_track_format, metrical_timing); // create our header for the file

    let mut predicted_track = Track::new(); // create our predicted track

    // populate track with our parsed sequence of notes
    for (note, dur) in parsed_sequence.iter() {
        let delta = *dur as u32 * metrical as u32;
        let key = match note {
            Note::Key(key) => u7::from(*key),
            Note::Rest => u7::from(0), // need to change this to actually be a rest...
        };
        // note on event with a delta of 0 after the previous note off
        let on_event = TrackEvent {
            delta: 0.into(),
            kind: Midi {
                channel: 0.into(),
                message: MidiMessage::NoteOn {
                    key,
                    vel: 50.into(),
                },
            },
        };
        // note off event delta time later
        let off_event = TrackEvent {
            delta: delta.into(), // delta time later
            kind: Midi {
                channel: 0.into(),
                message: MidiMessage::NoteOff {
                    key,
                    vel: 50.into(),
                },
            },
        };
        // push on and off events
        predicted_track.push(on_event);
        predicted_track.push(off_event);
    }
    // create standard midi file object
    let output_midi = Smf {
        header,
        tracks: vec![predicted_track.clone()],
    };
    // println!("{predicted_track:?}"); // what our note events look like
    let _ = output_midi.save(output_filename);
    println!(
        "Length of parsed seqeunce output of markov model... {}",
        parsed_sequence.len()
    );
}

// convert a (Note, beat) tuple to a single unique num to pass into our markov model
pub fn tuples_to_nums(
    note_sequence: Vec<(Note, f32)>,
    num_octaves: u32,
    lowest_allowed_pitch: u32,
    quantized_durations: &[f32],
) -> Vec<u32> {
    let mut new_note_sequence: Vec<u32> = Vec::new();
    for (note, duration) in note_sequence {
        let mut normalized_pitch_val = 0; // assume rest normalized pitch val

        if let Note::Key(pitch) = note {
            // normalize pitch to lowest_allowed_pitch
            normalized_pitch_val = pitch as i32 - lowest_allowed_pitch as i32;
            // if we have a negative number, add octaves until positive
            while normalized_pitch_val < 1 {
                normalized_pitch_val += 12;
            }
            // if we have a number larger than our max value, subtract octaves
            while normalized_pitch_val > (num_octaves as i32 * 12) {
                normalized_pitch_val -= 12;
            }
        }
        // you can probably do this in one line somehow with a map or filter / closure but this works and is pretty readable
        let (mut min_difference, mut closest_duration, mut length_multiplier) =
            (f32::MAX, -1.0, u32::MAX);
        for quantized_duration in quantized_durations {
            let difference = (duration - quantized_duration).abs();
            if min_difference > difference {
                min_difference = difference;
                closest_duration = *quantized_duration;
            }
        }
        // sets the length multiplier based on the closest length
        for (i, dur) in quantized_durations.iter().enumerate() {
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
        let hashed_pitch_val =
            normalized_pitch_val as u32 + (12 * num_octaves + 1) * length_multiplier;
        new_note_sequence.push(hashed_pitch_val);
    }
    new_note_sequence
}

// converts from a single num to a (Note, beat) tuple
pub fn nums_to_tuples(
    predicted_sequence: Vec<u32>,
    num_octaves: u32,
    lowest_allowed_pitch: u32,
    quantized_durations: &[f32],
) -> Vec<(Note, f32)> {
    let mut tuple_note_sequence: Vec<(Note, f32)> = Vec::new();

    for hashed_note in predicted_sequence {
        // finding note length
        let multiplier = hashed_note / (12 * &num_octaves + 1);
        let actual_length = quantized_durations[multiplier as usize];

        // finding key val
        let base_note_val = hashed_note % (12 * &num_octaves + 1);
        // println!("\nThe hashed val is: {}", hashed_note);
        // println!("The modder is: {}", (12 * &num_octaves + 1));
        // println!("The mod is: {}\n", hashed_note % (12 * &num_octaves + 1));

        // push either a rest and length or a note and a length
        if base_note_val == 0 {
            tuple_note_sequence.push((Note::Rest, actual_length));
        } else {
            tuple_note_sequence.push((
                Note::Key(lowest_allowed_pitch as u8 + base_note_val as u8 - 1),
                actual_length,
            ));
        }
    }
    tuple_note_sequence
}

#[cfg(test)]
mod test {
    use super::{nums_to_tuples, tuples_to_nums, Note};

    #[test]
    fn simple_vals_parseing() {
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Key(56), 0.25472),
            (Note::Key(76), 1.25472),
            (Note::Key(63), 0.65472),
        ];

        // 6 + 37 * 0, 26 + 37 * 2, 13 + 37 * 1
        let correct_list = vec![6, 100, 50];

        assert_eq!(
            correct_list,
            tuples_to_nums(example_list, 3, 50, &vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn rests_of_different_lengths() {
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Rest, 0.25472),
            (Note::Rest, 1.25472),
            (Note::Rest, 0.65472),
        ];

        // 0 + 37 * 0, 0 + 37 * 2, 0 + 37 * 1
        let correct_list = vec![0, 74, 37];

        assert_eq!(
            correct_list,
            tuples_to_nums(example_list, 3, 50, &vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn small_and_large_lengths() {
        // also tests having a different number of allowed quantized lengths
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Rest, 0.0),
            (Note::Rest, 25.0),
            (Note::Key(43), 0.2), // also tests if not value is being parsed correctly
            (Note::Key(89), 5.9), // also tests if not value is being parsed correctly
        ];

        // 0 + 37 * 0, 0 + 37 * 6, 5 + 37 * 1, 27 + 37 * 5
        let correct_list = vec![0, 222, 42, 212];

        assert_eq!(
            correct_list,
            tuples_to_nums(
                example_list,
                3,
                50,
                &vec![0.125, 0.25, 0.5, 1.0, 2.0, 4.0, 8.0]
            )
        );
    }

    #[test]
    fn different_num_octaves() {
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Rest, 1.0),
            (Note::Key(67), 2.0),
            (Note::Key(33), 0.5), // also tests if not value is being parsed correctly
            (Note::Key(89), 58.0),
            (Note::Key(124), 58.0), // also tests if not value is being parsed correctly
        ];

        // 0 + 61 * 2, 27 + 61 * 3, 5 + 61 * 1, 49 + 61 * 4, 60 + 61 * 4
        let correct_list = vec![122, 210, 66, 293, 304];

        assert_eq!(
            correct_list,
            tuples_to_nums(example_list, 5, 40, &vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn crazy_starting_notes() {
        let example_list: Vec<(Note, f32)> = vec![
            (Note::Key(1), 0.25472),
            (Note::Key(127), 1.25472),
            (Note::Key(15), 0.65472),
            (Note::Key(111), 0.65472),
        ];

        // (-> 61) 11 + 49 * 0, (-> 91) 41 + 49 * 2, (-> 51) 1 + 49 * 1, (-> 87) 37 + 49 * 1
        let correct_list = vec![11, 139, 50, 86];

        assert_eq!(
            correct_list,
            tuples_to_nums(example_list, 4, 50, &vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn basic_nums_to_tuples() {
        let example_list: Vec<u32> = vec![20, 87, 5, 105, 37, 1];

        let correct_list = vec![
            (Note::Key(69), 0.25),
            (Note::Key(62), 1.0),
            (Note::Key(54), 0.25),
            (Note::Key(80), 1.0),
            (Note::Rest, 0.5),
            (Note::Key(50), 0.25),
        ];

        assert_eq!(
            correct_list,
            nums_to_tuples(example_list, 3, 50, &vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn close_to_rests() {
        let example_list: Vec<u32> = vec![36, 37, 38, 73, 74, 75, 110, 111, 112];

        let correct_list = vec![
            (Note::Key(85), 0.25),
            (Note::Rest, 0.5),
            (Note::Key(50), 0.5),
            (Note::Key(85), 0.5),
            (Note::Rest, 1.0),
            (Note::Key(50), 1.0),
            (Note::Key(85), 1.0),
            (Note::Rest, 2.0),
            (Note::Key(50), 2.0),
        ];

        assert_eq!(
            correct_list,
            nums_to_tuples(example_list, 3, 50, &vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }

    #[test]
    fn dif_octaves() {
        let example_list: Vec<u32> = vec![20, 87, 5, 105, 183, 245, 200];

        let correct_list = vec![
            (Note::Key(69), 0.25),
            (Note::Key(75), 0.5),
            (Note::Key(54), 0.25),
            (Note::Key(93), 0.5),
            (Note::Rest, 2.0),
            (Note::Key(50), 4.0),
            (Note::Key(66), 2.0),
        ];

        assert_eq!(
            correct_list,
            nums_to_tuples(example_list, 5, 50, &vec![0.25, 0.5, 1.0, 2.0, 4.0])
        );
    }
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
