import React from 'react';
import './InputObject.css';

function TextBox() {
  return (
    <input type="number"  min= "1" placeholder="10" className='input'/>
  );
}

function ClassicalButton() {
  return (
    <button className='Classical'>Classical</button>
  );
}

function RockButton() {
  return (
    <button className='Rock'>Rock </button>
  );
}

function RnbButton() {
  return (
    <button className='Rnb'>Rnb</button>
  );
}

function JazzButton() {
  return (
    <button className='Jazz'>Jazz</button>
  );
}

function SubmitButton() {
  return (
    <button className='Submit'>Submit</button>
  );
}

function InputObject() {
  return (
    <div className="inputObjectContainer">
      <div className="inputAndSubmitContainer">  
        <TextBox />
        <SubmitButton />
      </div>
      <div className="buttonContainer">
        <ClassicalButton />
        <RockButton />
        <RnbButton />
        <JazzButton />
      </div>
    </div>
  );
}

export default InputObject;