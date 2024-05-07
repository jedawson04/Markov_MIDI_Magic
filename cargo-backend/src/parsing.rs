// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom
#[derive(Debug)]
enum Note {
    Key(u8),
    Rest,
} 

use midly::{num::u4, Smf, TrackEvent ,MidiMessage, TrackEventKind::Midi}; 
use std::{error::Error, fs};

// example code from reader-writer that shows how easy calculating durations could have been... lol.
// tbh it may STILL be worth reading their docs to see how they handle stuff and try and do things similarly.

    // let mut ticks_to_microseconds = ConvertTicksToMicroseconds::try_from(input_midi_file.header)?; -- we want something like this without reader writer...
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

// this file will have 2 main pub fns: from_midi() and to_midi() which will convert from and to midi files to our Markov model state

// takes in a midi file --> parses, converts to desired format and returns
pub fn from_midi(input_filepath: &str) -> Result<(), Box<dyn Error>> {
    let bytes = fs::read(input_filepath)?; // use read to convert filepath to bytes
    let input_midi_file = Smf::parse(&bytes)?; // use parse to create a midi object
    // store longest track with it's num
    println!("{0:?}",input_midi_file.header );
    let mut longest_track: &Vec<TrackEvent<'_>> = &Vec::new();
    for (i, track) in input_midi_file.tracks.iter().enumerate() {
        if track.len() > longest_track.len() {
            longest_track = track;
        }
        println!("track {} has {} events", i, track.len()); // print each track and num of events
    }
    // use track with most events
    let mut note_sequence: Vec<(Note, u32)> = Vec::new(); // whatever object we want to use to store our relevant information from events
    let mut current_note_val = 129; // key value of our current note on
    let mut ticks_since_on = 0;
    let mut rest_ticks = 0;
    let mut have_a_note: bool = false; // whether or not we have a note we are reading
    for event in longest_track.iter() // grabs first 150 notes of event
    { 
        let mut note_event = false;
        // match on event and set note delta and note on
        let (mut note, mut delta, mut note_on): (u8, u32, bool) = (0,0,false);
        match event.kind { 
            Midi {message, channel} => {  
                let channel_zero = u4::from(0); // this ensures we are only training on channels with piano -- can be changed
                match (message,channel) { 
                    // delta is how many MIDI ticks after the previous event should this event fire.
                    (MidiMessage::NoteOff{key, ..},channel_zero) => { 
                        // let info: String = format!("{key} off with delta {0}",event.delta);
                        (note, delta, note_on) = (u8::from(key), u32::from(event.delta), false);
                        note_event = true;
                        // if key == current_note_val {
                            //     note_sequence.push(note_tuple);
                            // }
                            // can use duration enum here to round -- depending on the tempo..? -- not sure
                            // println!("{key} off with delta {0}",event.delta);
                    }
                    (MidiMessage::NoteOn{key, ..},channel_zero) => { 
                        // current_note_val = key;
                        (note, delta, note_on) = (u8::from(key), u32::from(event.delta), true);
                        note_event = true;
                        // let info: String = format!("{key} on with delta {0}",event.delta);
                        // println!("{key} on with delta {0}",event.delta);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        if !note_event { // abstract - refactor this soon
            continue;
        }
        println!("The events note val is: {}, the current note val is: {} this note is on is {}", note, current_note_val, note_on);
        if !have_a_note {
            println!("\tWe do not have a current note.");
            let pitch_difference: i32 = (note as i32 - current_note_val as i32).try_into().unwrap();
            // if this note should become the current note (different if statement depending on if it's the starting note vs following another note)
            if current_note_val == 129 {
                println!("We are setting the very first current note.");
                // do we want to set this as the current note or not?
                if note >= 50 && note < 75 {
                    // set as the current note (and have_a_note bool to true)
                    current_note_val = note;
                    have_a_note = true;
                }
            } else if pitch_difference < 12 || pitch_difference > -12 {
                println!("We are changing to a new current note");
                // set as the current note (and have_a_note bool to true)
                current_note_val = note;
                have_a_note = true;
                // if rest_ticks is not 0
                if rest_ticks != 0 {
                    // increment rest ticks
                    // push a rest/rest_ticks tuple to our note sequence list
                    note_sequence.push((Note::Rest, rest_ticks + delta));
                    // reset the rest ticks variable to 0
                    rest_ticks = 0;
                }
            } else {
                println!("This note is not going to become out current note.");
                // increment rest ticks
                rest_ticks += delta;
            }
        }
        else {
            println!("We do have a current note");
            // if the current event is not our current note's Note Off signal
            if note != current_note_val {
                println!("This event is not the desired Note Off signal.");
                // increment ticks since on
                ticks_since_on += delta;
            } else { // else (should only catch if this is our desired Note Off signal)
                println!("This event IS the desired Note Off signal.");
                // increment ticks since on
                // push note/ticks since on tuple to our note sequence list
                note_sequence.push((Note::Key(current_note_val), ticks_since_on + delta));
                // set current note to none, set ticks since on variable to 0 and have_a_note bool to false
                have_a_note = false;
                ticks_since_on = 0;
            }
        }
        println!();
    }
    // prints out all the stored info and length of vec
    println!("{note_sequence:?}"); 
    println!("{}", note_sequence.len()); 

    // want to store notes with their on off and duration  

    Ok(())
}
// note about current output: we are not successfully keeping the following note limited within in octave either direction of the previous
// also there is some weird stuff w tick lengths in the thousands/tens of thousands, but that could be legit I guess

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