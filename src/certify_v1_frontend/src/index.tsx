import React from "react";
import { render } from "react-dom";
import App from "./App";
import "./main.css";

const Index = () => {
  return (
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
};

render(<Index />, document.getElementById("app"));
