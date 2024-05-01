import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <TextBox />
      <ClassicalButton/>
      <RockButton/>
      <RnbButton/>
      <JazzButton/>
    </div>
  );

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

}

export default App;