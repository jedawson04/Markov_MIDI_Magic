import './App.css';
import InputObject from './Components/InputObject/InputObject';
import Title from './Components/Header/Title';
import MidiPlayerComponent from './Components/MidiPlayer/MidiPlayer';
import React from 'react';

function App() {
  return (
    <div className="App">
      <div className='title'>
        <Title/>
      </div>
      <div className='inputContainer'>
        <InputObject/>
      </div>
      <div className='MidiInteract'>
        <MidiPlayerComponent/>
      </div>
    </div>
  );
}


export default App;