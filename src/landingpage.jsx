import { React } from "react";
import { Link, Outlet } from "react-router-dom";
import template from "./assets/homepagetemplate.png";
import { CheckIcon, LockClosedIcon, GlobeIcon } from "@radix-ui/react-icons";
import Footer from "./footer";
import Nav from "./navbar";

function Home() {
  const goals = [
    {
      id: 1,
      icon: (
        <CheckIcon className="  bg-[#778da91a] w-[55px] h-[55px] rounded-full p-3 text-[#61a5c2] " />
      ),
      title: "Instant Verification",
      content: "Instantly verify the authenticity of certificates, ",
    },
    {
      id: 2,
      icon: (
        <GlobeIcon className="  bg-[#778da91a] w-[55px] h-[55px] rounded-full p-3 text-[#61a5c2] " />
      ),
      title: "Global Accessibility",
      content: "Accessible anywhere in the world.",
    },
    {
      id: 3,
      icon: (
        <LockClosedIcon className="  bg-[#778da91a] w-[55px] h-[55px] rounded-full p-3 text-[#61a5c2]" />
      ),
      title: "Data Security",
      content: "We protect your data from unwanted access",
    },

    {
      id: 4,
      icon: (
        <CheckIcon className="  bg-[#778da91a] w-[55px] h-[55px] rounded-full p-3 text-[#61a5c2]" />
      ),
      title: "Easy",
      content: "We ensure an easy process to verifying these certificates",
    },
  ];

  const works = [
    {
      id: 1,
      content: [
        "Upload your certificate ",
        <Link to="/upload" className=" hover:text-[#ffffffb1]">
          here.
        </Link>,
      ],
    },
    { id: 2, content: "We verify the certificate. " },
    {
      id: 3,
      content: "You get issued an id. ",
    },
    {
      id: 4,
      content: "Share your issued id with anyone.",
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
      <div className="bg-white justify-self-start  text-[17px] text-[#0d1b2a] w-[45px] h-[45px] p-1 rounded-full flex justify-center items-center">
        {work.id}
      </div>
      <div className=" mx-4 text-[20px] w- text-left  ">{work.content}</div>
    </div>
  ));

  return (
    <div className=" h-[971px] bg-[#012a4a] absolute inset-0 font-['Lato']">
      <Nav />
      <div className=" bg-[#012a4a] relative h-[650px] flex flex-col items-center   w-screen ">
        <div className="flex flex-col items-center bg-white w-screen h-[450px]">
          <h2 className=" mt-[80px] font-extrabold text-[50px] w-[600px] text-center text-[#0d1b2a] leading-tight">
            Send in the Certificate! Let's help you Verify.
          </h2>
          <a href="#about">
            <p className="p-2  w-[500px] text-[#415a77] text-center">
              Unlocking the Power of Verified Credentials with Blockchain
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
        className=" flex justify-between items-center h-[600px] bg-white mt-[170px] text-[#415a77] py-[300px] border-y-[50px] border-[#012a4a] mb-0"
      >
        {" "}
        <div className=" flex justify-between items-center  w-screen">
          <div className=" w-fit">
            <h1 className=" text-[55px] font-extrabold w-[600px] text-[#0d1b2a] ml-16 ">
              Our goal is to help verify and secure the authenticity of
              certificates.
            </h1>
          </div>
          <div className="  grid grid-cols-2 gap-2 grid-rows-2 ml-0">{Item}</div>{" "}
        </div>
      </section>
      <section
        id="howitworks"
        className=" border-y-[60px] border-[#012a4a]  bg-[#012a4a] h-fit p-5 flex flex-col justify-center  items-center text-white mt-0"
      >
        <h3 className=" mt-[0px] mb-[20px] font-extrabold text-[50px] w-fit text-center text-white leading-tight">
          How it works
        </h3>
        <div className=" mb-[30px]">{Items}</div>
      </section>
      <Footer />
      <Outlet />
    </div>
  );
}

export default Home;
