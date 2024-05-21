# Welcome to Markov MIDI Magic       

Our program leverages rust's fast build times and verbose crates to generate and display MIDI files based off of different genres and other user specifications.  

Originally started as a Final Project for CSCI 241, Systems Programming.

## Contributors: Joshua Dawson, Colvin Iorio, Ezra Crowe, and Jacob Pellechia  

## todo!() list lol / things to work on/add

### the big ones -- front end stuff scary

    - get frontend/backend communication working successsfully -- within webserver.rs 
        - this means, allow libraries and hyperparameters selected by the user to govern the resulting output. 

    - add implementation for music playing/scrolling -- the big one
        - this is the most important part of our final project imo.

    - a way for the user to provide their own files to train on, in conjuntion other other files -- I've added a 'user_selection' directory in genres but this is barely a first step lol.
        - this requires a decent bit of work. I'm ignorant about how it would be done, but we would need a way for the user to interact with and send files to the backend, and this is all done when they click the magic button. 

    - let the user store their most recent creation in case they want to save it somehow -- down the line, maybe add a saved creations directory - would probably go away on refresh, idk how frontend works tbh :(

    - for all the data we have to train on, we should provided detailed descriptions of what kind of midi files are contained in each subfolder. This should include a summary of the type of music and some of its key features such as how it may inform the behavior of the model resulting from training on it (for example jazz is mostly solos so you get headers and liccs).

#### summarizing these ideas together, here's my idea of the user interface control flow experience
 
    - open the server, see static libraries to train on, each with a summary about what they contain and what musical features they have. User selects the libraries they would like to use, including possibly adding their own files (upto a limit of some kind?) and decide how much they want to weigh each library. 
 
    - They then decide the hyperparameters: markov model order, longest/shortest duration and lowest starting note are the important ones (We don't want too many of these, maybe there's an 'Advanced mode' where they change a lot?). If not selected, each of these parameters has default values. 
    
    - Finally, they click 'Magic' and the file generated is displayed and played for the user with our beautiful interface, and they can decide to save their creation if desired. Either way, there should be a way to reset the process and generate again. Maybe the parameters are chosen on one page and clicking 'Magic' takes you to a new page where the music is displayed, and there is a button to go back and 'Try Again' or 'Magic Again.'

    - something that would be really interesting is allowing the user to use their own creations to generate new ones with. While this is possible by them downloading the file and adding it to the user_selections, it would be cool to make it a more concrete feature. Of course, this is way, way, way down the line.

### back end goals/wip stuff

    - Since we allow for selection of multiple genres (in backend/src/main) we should normalize metrical tempo somehow...
        - haven't made any progress on this but sometimes the metrical is totally fine... requires a bit more experimentation.

    - user can 'weigh' certain libraries x times more which means they are feed through the model x times. However, since the output length is proportional to the training data length, this causes huge files to be created. 
        - this means we should decide some way to cap/normalize/otherwise restrict the length of the files. Especially if the user selects classical 5x and then jazz, we'll have 3+ hours of music...

    - try to identify why certain notes are repeated for long duration! Is this a flaw in the model, or the parsing? Would it be fixed by expanding the size of the training data? 

    - otherwise the most important next steps for the backend is adding more training data, and experimenting with it to identify how it chages the performance of the generated midi. 