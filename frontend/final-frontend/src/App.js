import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
      <TextBox />
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