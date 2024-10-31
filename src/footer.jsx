import React from "react";
import { Link, Outlet } from "react-router-dom";
import logo from "./assets/logo.png";

function Footer() {
  return (
    <footer className=" bg-[white] h-fit pt-8 pb-0 flex flex-col items-center w-screen  ">
      <div className="flex justify-around w-screen ">
        <div>
          {" "}
          <img src={logo} alt="" className="w-[150px]" />
        </div>
        <ul className="flex  ">
          <li className="m-2 mx-4">
            {" "}
            <Link to="/"> Home</Link>
          </li>
          <a href="/#about" className="m-2 mx-4">
            About
          </a>
          <a href="" className="m-2 mx-4">
            Contact
          </a>
        </ul>
        <ul className="flex  ">
          <li className="m-2">Privacy Policy</li>
          <li className="m-2">Terms of use</li>
        </ul>
      </div>
      <div className=" m-5 font-['Raleway'] ">
        <span className="   text-gray-300 font-light text-[14px]">
          Copyright@2023 . Certify. All rights reserved
        </span>
      </div>
    </footer>
  );
}

export default Footer;
