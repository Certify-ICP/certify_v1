import { useState, React, useRef } from "react";
import logo from "./assets/logo.png";
import { Link, Outlet } from "react-router-dom";

function Nav() {
  const ref = useRef(null);
  const handleClick = () => {
    ref.current?.scrollIntoView({ behaviour: "smooth" });
  };

  return (
    <nav className=" bg-white flex  w-screen items-center justify-between p-4 border-b ">
      <div className="justify-self-start">
        <Link to="/">
          <img src={logo} alt="" srcset="" className=" w-[100px]" />
        </Link>
      </div>
      <ul className=" grid grid-cols-3 grid-rows-1 gap-4 text-[#012a4a]">
        
        <a
          className="p-0 font-bold w-fit "
          onClick={handleClick}
          href="/#about"
        >
          About Certify
        </a>{" "}
        <a className="p-0 font-bold w-fit " href="/#howitworks">
          How it works?
        </a>
        <a className="p-0 font-bold w-fit " onClick={handleClick} href="">
          <Link to="/universitymodal"> Universities</Link>
        </a>
      </ul>
      <div className=" justify-self-end">
        <Link to="/upload">
          <button
            formTarget=""
            className="bg-[#1b263b] hover:bg-[#1b263bde] font-['Lato'] font-bold p-3 text-white rounded-lg text-[13px]"
          >
            Verify your Certificate
          </button>
        </Link>
      </div>
      <Outlet />
    </nav>
  );
}
export default Nav;
