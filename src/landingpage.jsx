import { useState, React } from "react";
import { Link, Outlet } from "react-router-dom";
import logo from "./assets/logo.png";
import template from "./assets/template.png";
import { CheckIcon, LockClosedIcon, GlobeIcon } from "@radix-ui/react-icons";

import Footer from "./footer";
import Nav from "./navbar";

function Home() {
  const [count, setCount] = useState(0);

  const goals = [
    {
      id: 1,
      icon: (
        <CheckIcon className="  bg-[#778da91a] w-[60px] h-[60px] rounded-full p-2 text-[#61a5c2] " />
      ),
      title: "Instant Verification",
      content: "Instantly verify the authenticity of certificates, ",
    },
    {
      id: 2,
      icon: (
        <GlobeIcon className="  bg-[#778da91a] w-[60px] h-[60px] rounded-full p-2 text-[#61a5c2] " />
      ),
      title: "Global Accessibility",
      content: "Accessible anywhere in the world.",
    },
    {
      id: 3,
      icon: (
        <LockClosedIcon className="  bg-[#778da91a] w-[60px] h-[60px] rounded-full p-2 text-[#61a5c2]" />
      ),
      title: "Data Security",
      content: "We protect your data from unwanted access",
    },

    {
      id: 4,
      icon: (
        <CheckIcon className="  bg-[#778da91a] w-[60px] h-[60px] rounded-full p-2 text-[#61a5c2]" />
      ),
      title: "Easy",
      content: "We ensure an easy process to verifying these certificates",
    },
  ];

  const works = [
    {
      id: 1,
      content: ["Upload your certificate ", <Link to="/upload">here</Link>],
    },
    { id: 2, content: "We verify the certificate " },
    {
      id: 3,
      content: "BLAH BLAH ",
    },
    {
      id: 4,
      content: "yoyoyoupufvf",
    },
    {
      id: 5,
      content: "That's it!",
    },
  ];

  const Item = goals.map((goal) => (
    <div className=" w-fit mt-2 p-2 ">
      <span className=" mb-4 ">{goal.icon}</span>
      <div className=" mt-2  text-[#0d1b2a]">
        <h3 className=" mb-2 text-xl font-bold">{goal.title}</h3>
        <p className=" w-[300px] font-thin text-[#0d1b2adc]  ">
          {goal.content}
        </p>
      </div>
    </div>
  ));

  const Items = works.map((work) => (
    <div className="flex m-6 items-center w-[380px]">
      <div className="bg-white justify-self-start  text-[20px] text-[#0d1b2a] w-12 h-12 rounded-full flex justify-center items-center">
        {work.id}
      </div>
      <div className=" mx-4 text-[20px] w- text-left  ">{work.content}</div>
    </div>
  ));

  return (
    <div className=" h-[971px] bg-[#012a4a] absolute inset-0 font-['Rubik']">
      <Nav />
      <div className=" bg-[#012a4a] relative h-[650px] flex flex-col items-center   w-screen ">
        <div className="flex flex-col items-center bg-white w-screen h-[450px]">
          <h2 className=" mt-[80px] font-extrabold text-[50px] w-[600px] text-center text-[#0d1b2a] leading-tight">
            Send in the Certificate! Let's help you Verify.
          </h2>
          <a href="#about">
            <p className="p-2  w-[500px] text-[#415a77] text-center">
              Unlock the Power of Verified Credentials with Blockchain
              Technology!
            </p>
          </a>
        </div>
        <div className="absolute bottom-0">
          <img src={template} alt="" className=" w-[500px] shadow-2xl" />{" "}
        </div>
      </div>
      <section
        id="about"
        className=" flex justify-between items-center h-[600px] bg-white mt-[300px] text-[#415a77] p-4 "
      >
        <div>
          <h1 className=" text-[60px] font-extrabold w-[600px] text-[#0d1b2a] ml-20 ">
            Our goal is to help verify and secure the authenticity of
            certificates.
          </h1>
        </div>
        <div className=" grid grid-cols-2 gap-2 grid-rows-2 ml-0">{Item}</div>
      </section>
      <section
        id="howitworks"
        className=" bg-[#012a4a] h-fit p-5 flex flex-col justify-center items-center text-white"
      >
        <h3 className="mt-[80px] font-extrabold text-[50px] w-[600px] text-center text-white leading-tight">
          How it works
        </h3>
        <div>{Items}</div>
      </section>
      <Footer />
      <Outlet />
    </div>
  );
}

export default Home;
