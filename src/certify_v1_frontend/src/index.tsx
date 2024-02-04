import "./main.css";
import React from "react";
import { render } from "react-dom";
const MyHello = () => {
  const [name, setName] = React.useState("");
  const [message, setMessage] = React.useState("Test");

  return (
    <div style={{ fontSize: "30px" }}>
      <div className="bg-red-500">
        <p>Greetings, from DFINITY!</p>
        <p>
          {" "}
          Type your message in the Name input field, then click{" "}
          <b> Get Greeting</b> to display the result.
        </p>
      </div>
      <div style={{ margin: "30px" }}>
        <input
          id="name"
          value={name}
          onChange={(ev) => setName(ev.target.value)}
        ></input>
      </div>
      <div>
        Greeting is: "<span style={{ color: "blue" }}>{message}</span>"
      </div>
    </div>
  );
};

const App = () => {
  return (
    <div>
      <MyHello />
    </div>
  );
};

render(<App />, document.getElementById("app"));
