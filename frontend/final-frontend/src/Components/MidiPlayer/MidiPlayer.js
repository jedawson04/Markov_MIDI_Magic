import React, { useEffect } from "react";

function MidiPlayerComponent() {
  useEffect(() => {
    const script = document.createElement("script");
    script.src =
      "https://cdn.jsdelivr.net/combine/npm/tone@14.7.58,npm/@magenta/music@1.23.1/es6/core.js,npm/focus-visible@5,npm/html-midi-player@1.5.0";
    script.async = true;
    document.body.appendChild(script);
  }, []);

  return (
    <div>
      {/* <midi-player
        src="public\Pirates.mid"
        sound-font
        visualizer="#myVisualizer"
      ></midi-player>
      <midi-visualizer type="piano-roll" id="myVisualizer"></midi-visualizer> */}
      <midi-player
        src="https://magenta.github.io/magenta-js/music/demos/melody.mid"
        sound-font
        visualizer="#myVisualizer"
      ></midi-player>
      <midi-visualizer type="piano-roll" id="myVisualizer"></midi-visualizer>
    </div>
  );
}

export default MidiPlayerComponent;
