import { useState, React } from "react";
import Nav from "./navbar";
import Footer from "./footer";

function Upload() {
  const fileTypes = ["PDF", "JPG", "PNG"];
  const [firstname, setFirstName] = useState("");
  const [lastame, setLastName] = useState("");
  const [institution, setInstitution] = useState("");

  

  return (
    <div className="flex flex-col inset-0 h-screen font-['Lato'] bg-[#012a4a]">
      <Nav />
      <div className="flex-grow mb-10 flex flex-col justify-center items-center w-screen ">
        <div className="p-2 h-fit bg-white w-fit rounded-lg shadow-lg ">
          <h4 className="text-[26px] text-[#012a4a] font-bold p-2">
            Upload your certificate
          </h4>
          <form
            action=""
            className="flex flex-col justify-center items-center p-2 "
          >
            <span className="flex">
              <div>
                <label htmlFor="#first-name"></label>
                <input
                  type="text"
                  name="first"
                  id="first-name"
                  placeholder="First Name"
                  className="w-[400px] h-[40px] m-2 rounded-lg p-2 border border-[#415a77] focus:ring-blue-500 focus:border-blue-500  "
                  required
                />
              </div>

              <input
                type="text"
                name=""
                id=""
                placeholder="Last Name"
                className="w-[400px] h-[40px] m-2 rounded-lg p-2 border border-[#415a77] focus:ring-blue-500 focus:border-blue-500 "
                required
              />
            </span>
            <select
              name="Institution"
              id=""
              required
              className="w-[818px] border border-[#415a77] m-2 p-2 rounded-lg bg-transparent focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">Select your Institution</option>
              <option value="ABUAD">Afe-Babalola University</option>
              <option value="Madonna">Madonna University</option>
              <option value="Covenant">Covenant University</option>
            </select>
            <span className="flex flex-col">
              <label htmlFor="file" className="text-[#012a4a] mt-4 ">
                Attach Your Certificate File Here!
              </label>
              <input
                type="file"
                name="file"
                id=""
                required
                className=" block w-[350px] text-sm text-gray-500 my-2 mb-4
              file:me-4 file:py-2 file:px-4
              file:rounded-lg file:border-0
              file:text-sm file:font-semibold
              file:bg-[#012a4a] file:text-white
              hover:file:bg-[#012a4ab7]
              file:disabled:opacity-50 file:disabled:pointer-events-none
             "
              />
            </span>
            {/* <FileUploader
              handleChange={handleChange}
              name="file"
              types={fileTypes}
              classes=' bg-transparent w-[800px] text-[#012a4a] '
            /> */}

            <input
              type="submit"
              value="Submit"
              className=" w-[300px] h-[50px] text-[18px] my-2 py-2 px-4 bg-[#012a4a] text-white rounded-lg"
            />
          </form>
        </div>
      </div>
      <Footer />
    </div>
  );
}

export default Upload;
