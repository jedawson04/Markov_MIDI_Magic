import './App.css';
import InputObject from './Components/InputObject/InputObject';
import Title from './Components/Header/Title';
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
    </div>
  );
}


export default App;