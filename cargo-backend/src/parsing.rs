// http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html
// this is an excellent website explaining what MIDI data looks like. It's a little tough to read but has great info
// plus it has example midi files at the bottom

enum _Duration {
    // not sure if this is how we'd like to store stuff, but I think we want an enum for Duration
    Half(u32),
    Quarter(u32),
    Eighth(u32),
    Sixteenth(u32),
} // this might go in parsing or markov, I'm not sure

use midly::{num::u4, Smf, TrackEvent, MidiMessage, TrackEventKind::Midi,}; // because the stupid other one isn't working :(
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
    
    let mut object: Vec<String> = Vec::new(); // whatever object we want to use to store our relevant information from events
    // for now just work with the track with the most events and print it.
    for event in longest_track.iter().take(150) // grabs first 150 notes of event
    { 
        // println!("{0:?}",event);
        match event.kind { 
            Midi {message, channel} => {  
                let channel_zero = u4::from(0); // this ensures we are only training on channels with piano -- can be changed
                match (message,channel) { 
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
                    // delta is how many MIDI ticks after the previous event should this event fire.
                    (MidiMessage::NoteOff{key, ..},channel_zero) => { 
                        let info: String = format!("{key} off with delta {0}",event.delta);
                        // can use duration enum here to round -- depending on the tempo..? -- not sure
                        object.push(info);
                        // println!("{key} off with delta {0}",event.delta);
                    }
                    (MidiMessage::NoteOn{key, ..},channel_zero) => { 
                        let info: String = format!("{key} on with delta {0}",event.delta);
                        object.push(info);
                        // println!("{key} on with delta {0}",event.delta);
                    }
                    _ => ()
                }
            }
            _ => (),
        }
    }
    // prints out all the stored info and length of vec
    println!("{object:?}"); 
    println!("{}", object.len()); 

    // want to store notes with their on off and duration  

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
