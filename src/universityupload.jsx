import React from "react";
import Nav from "./navbar";
import Footer from "./footer";

function Org() {
  return (
    <div className="absolute flex flex-col w-screen inset-0 font-['Lato']   items-center justify-between">
      <Nav/>
        <h1 className=" flex text-[100px] bg-gradient-to-r from-[#74ebd5] to-[#012a4a] text-transparent bg-clip-text ">Coming soon...</h1>
      <Footer/>
    </div>
  );
}

export default Org;
