import "./App.css";
import { useState } from "react";
import Axios from "axios";
import { saveAs } from "file-saver";
import Information from "./Information";
import Revocation from "./Revocation";

function App() {
  const [info, setInfo] = useState({});
  const [revocationIdx, setRevocationIdx] = useState("");
  const [revocationList, setRevocationList] = useState(null);

  const handleInfoChange = (childData) => {
    setInfo(childData);
  };
  const handleRevocationChange = (childData) => {
    setRevocationIdx(childData);
  };

  const createVC = () => {
    const data = {
      name: info.name,
      did: info.did,
      country: info.country,
      birth: info.birth,
    };

    console.log(data)

    Axios.post("http://localhost:5001/create", data)
      .then((response) => {
        const blob = new Blob([JSON.stringify(response.data)], {
          type: "application/json",
        });
        saveAs(blob, info.name + "-VC.json");
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  };

  const revocation = () => {
    console.log(revocationIdx);
    Axios.post("http://localhost:5001/revocation", revocationIdx);
  };

  const showRevocationList = () => {
    Axios.get("http://localhost:5001/showRevocationList", revocationIdx).then(
      (response) => {
        console.log(response.data);
        setRevocationList(response.data);
      }
    );
  };

  return (
    <div className="App">
      <div className="Form">
        <div className="information">
          <Information onChildDataChange={handleInfoChange}></Information>
        </div>
        <button onClick={createVC}>createVC</button>
      </div>

      <div className="revocation from">
        <Revocation onChildDataChange={handleRevocationChange}></Revocation>
        <button onClick={revocation}>revocation</button>
        <button onClick={showRevocationList}>show revocation list</button>
        <br />
        <div>
          Revocation List:
          {revocationList
            ? " " + JSON.stringify(revocationList)
            : " Click show revocation list"}
        </div>
      </div>
    </div>
  );
}

export default App;
