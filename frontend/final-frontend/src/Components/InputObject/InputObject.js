import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './InputObject.css';
import Midi_Display from '../MidiDisplay/Midi-Display';
import "../MidiDisplay/Midi-Display.css";

function TextBox() {
  return (
    <input type="number"  min= "1" placeholder="10" className='input'/>
  );
}

function StyleButton({ style, setStyle }) {
  return (
    <button onClick={() => setStyle(style)} className={style}>{style}</button>
  );
}

function SubmitButton({ style, setMidiDisplay }) {

  const [fileUrl, setFileUrl] = useState(null);

  useEffect(() => {
    console.log("fileUrl is " + fileUrl);
    if (fileUrl) {
      setMidiDisplay(<Midi_Display midiFilePath={fileUrl} />);
    }
  }, [fileUrl]);
  
  const handleSubmit = async (event) => {
    event.preventDefault();
  
    try {
      const response = await axios.post('http://localhost:3030/midi', { style }, {
        headers: {
          'Content-Type': 'application/json',
        },
        responseType: 'blob', // Tell axios to expect a Blob in the response
      });
      console.log("response is " + response);
      // Create a Blob URL from the response data
      const fileUrlGrab = URL.createObjectURL(response.data);
      setFileUrl(fileUrlGrab);
    } catch (error) {
      console.error('Error generating MIDI file:', error);
    }
  };
  

  return (
    <button onClick={handleSubmit} className='Submit'>Submit</button>
  );
}


function InputObject() {
  const [style, setStyle] = useState('');
  const [midiDisplay, setMidiDisplay] = useState(null);

  console.log("mididisplay is " + midiDisplay);

  return (
    <div>
      <TextBox />
      <StyleButton style='Classical' setStyle={setStyle} />
      <StyleButton style='Rock' setStyle={setStyle} />
      <StyleButton style='Rnb' setStyle={setStyle} />
      <StyleButton style='Jazz' setStyle={setStyle} />
      <SubmitButton style={style} setMidiDisplay={setMidiDisplay} />
      <div className='display'>{midiDisplay}</div>
    </div>
  );
}

export default InputObject;