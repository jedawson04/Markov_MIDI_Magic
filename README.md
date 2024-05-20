# Welcome to Markov MIDI Magic       

Our program leverages rust's fast build times and verbose crates to generate and display MIDI files based off of different genres and other user specifications.  

Originally started as a Final Project for CSCI 241, Systems Programming.

## Contributors: Joshua Dawson, Colvin Iorio, Ezra Crowe, and Jacob Pellechia  

## todo!() list lol / things to work on/add

### the big ones -- front end stuff scary

    - get frontend/backend communication working successsfully -- within webserver.rs

    - add implementation for music playing/scrolling -- the big one

    - a way for the user to provide their own files to train on, in conjuntion other other files -- I've added a 'user_selection' directory in genres but this is barely a first step lol.

    - let the user store their most recent creation in case they want to save it somehow -- down the line, maybe add a saved creations directory - would probably go away on refresh, idk how frontend works tbh :(


### back end goals/wip stuff

    - Since we allow for selection of multiple genres (in backend/src/main)- should normalize metrical tempo somehow...
        - haven't made any progress on this but sometimes the metrical is totally fine... requires a bit more experimentation.

    - weigh certian genres more than others, upon user specification. Maybe they want 3x jazz and 1x classical, so we train it on jazz 3x and classical once? 
        - did it the 'simple' way and it seems to work pretty well, although adding more training greatly increases duration of output file. Should normalize this somehow. 
        - In my vision for the the front end, the user can decide how much to 'weigh' a certain genre/music directory. If they weigh it 4x, then we add it 4x. 
        
    - find a good way to balance using webserver vs main. Probably have to put main in the bin and rework the inline mods.. 
        - i believe this has been successfully fixed using rust's submodules.