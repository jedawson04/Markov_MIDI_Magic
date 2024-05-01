import React from 'react';

function TextBox() {
  return (
    <input type="text" placeholder="Enter text here" />
  );
}

function ClassicalButton() {
  return (
    <button>Classical</button>
  );
}

function RockButton() {
  return (
    <button>Rock</button>
  );
}

function RnbButton() {
  return (
    <button>Rnb</button>
  );
}

function JazzButton() {
  return (
    <button>Jazz</button>
  );
}

function SubmitButton() {
  return (
    <button>Submit</button>
  );
}

function InputObject() {
  return (
    <div>
      <TextBox />
      <ClassicalButton />
      <RockButton />
      <RnbButton />
      <JazzButton />
      <SubmitButton />
    </div>
  );
}

export default InputObject;