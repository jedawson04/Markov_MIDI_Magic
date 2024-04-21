# Systems Programming Final Project Proposal

## Group Members: Colvin Iorio, Joshua Dawson, Ezra Crowe, and Jacob Pellechia

## Proposal	

### initial claim 

Our group’s project proposal is to train a machine learning model to generate midi information. With the model trained using MIDI information available online from popular songs or from chord packs that are sold to music producers, our program will be able to generate new MIDI information. We also plan on creating a friendly user interface to host the program. Depending on time remaining towards the end of the project, we could implement a more detailed prompt interface. In order to complete this project, we will implement and train a machine learning model available to us through Rust’s crates. To train the model, we will use MIDI data available online that can be downloaded for free. Then, to implement a user interface, we will work with React.

### what we will need to use 

This project will require us to learn about machine learning models. Of course, there are various types of models available for use. In our specific case, we have an unsupervised task, and a training set composed of MIDI data, so certain models will work better than others. Identifying what machine learning model to use will be the first step of our project.

Next, we will learn more about MIDI data files. Several group members are musicians and have worked with MIDI data before, but this differs from utilizing the raw MIDI files. Depending on which model we choose, the data may have to be in a certain format, so some preprocessing may be required. After this, we will have to learn how to tailor the training set for the model. MIDI data is available for many genres of music, and the type chosen for the training set will constrain the output of the music generated.

### Potential Complications

Some complications we expect with this process of creating our training set concern the complexity of machine learning in our specific instance. We will have to determine how much MIDI data we need to properly train our model while completing this project on time. In doing this, we will explore the level of complexity that comes with the timing of overlapping notes due to chords and/or harmonies.

One of the latter aspects of the project will be creating a user interface. Depending on how much time remains for this aspect of the project, we may attempt to build a website versus an app. We will likely use ReactJS, which some members of the group have worked with previously when designing websites.

### Prospective Timeline

For our timeline, we have split the project into 5 weeks from now until the final project presentation day. In week 1 (Wed, April 10–Tue, April 16), we will research and decide which learning model to use and identify how we should construct our data set. Week 2 (Wed, April 17–Tue, April 23) will be used to collect midi data, design a basic user interface, and begin model training. In week 3 (Wed, April 24–Tue, April 30), we will split into two groups, one working on the front end and one working on the backend. The frontend group will begin working with ReactJS to make the UI, while the backend group will go more in depth with the model training. 

This is also the week that our status report is due; depending on our progress at that point, we will adjust our UI and model training goals to be more realistic or ambitious. In week 4 (Wed, May 1–Tue, May 7) the two groups will continue working in their same respective areas, hopefully coming fairly close to completion. Finally, in week 5 (Wed, May 8–Wed, May 15), we will bring the model and IU together to create a working program, and we will prepare our final report and presentation. For the report and presentation, we will create a visualization to display our overall achievements. Both the report and presentation are due at 9 am on Wednesday, May 15th, bringing an end to our project.
