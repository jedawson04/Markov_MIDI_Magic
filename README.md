# Welcome to Markov MIDI Magic       

Our program leverages rust's fast build times and verbose crates to generate and display MIDI files based off of different genres and other user specifications.  

Originally started as a Final Project for CSCI 241, Systems Programming.

## Contributors: Joshua Dawson, Colvin Iorio, Ezra Crowe, and Jacob Pellechia  

## todo!() list lol / things to work on/add

    - a way for the user to provide their own files to train on, in conjuntion other other files -- I've added a file in genres but this is barely a first step lol.
    - weigh certian genres more than others, upon user specification. Maybe they want 3x jazz and 1x classical, so we train it on jazz 3x and classical once? - this might be as simple as adding it to the user specs vec as many times as specified
    - Since we allow for selection of multiple genres (in backend/src/main)- should normalize metrical tempo somehow...
    - let the user store their most recent creation in case they want to save it somehow -- down the line, maybe add a saved creations directory - would probably go away on refresh, idk how frontend works tbh :(
    - find a good way to balance using webserver vs main. Probably have to put main in the bin and rework the inline mods..