import React, { useState } from "react";
import instance from "../api";
import Card from "react-bootstrap/Card";
import "react-json-view-lite/dist/index.css";
import downloadFile from "../downloadFileAPI";
import FileDownload from "js-file-download";
import Form from 'react-bootstrap/Form';
import Container from 'react-bootstrap/Container';

export default function Login() {
  const [userDID, setUserDID] = useState("");
  const [loading, setLoading] = useState(false);
  const [file, setFile] = useState();
  const [verificationMethod, setVerificationMethod] = useState("");
  const [challenge, setChallenge] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const handleFileChange = (e) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
      console.log(e.target.files[0]);
    }
  };
  const handleChallenge = (e) => {
    setChallenge(e.target.value);
  };
  const handleUserDID = (e) => {
    setUserDID(e.target.value);
  };
  const handleVM = (e) => {
    setVerificationMethod(e.target.value);
  };

  const handleLogin = (e) => {
    setLoading(true);
    setErrorMessage("");

    if (userDID === "") {
      setErrorMessage("Please enter userDID!");
      setLoading(false);
    } else if (!file) {
      setErrorMessage("Please choose the correct file!");
      setLoading(false);
    } else if (verificationMethod === "") {
      setErrorMessage("Please enter the fragment!");
      setLoading(false);
    } else if (challenge === "") {
      setErrorMessage("Please enter challenge!");
      setLoading(false);
    } else {
        let formData = new FormData();
        console.log("file: ", file);
        formData.append("file", file);
        console.log("formData: ", formData);
        const config = {
          headers: { "Content-Type": "multipart/form-data" },
        };
        
        instance
          .post("/uploadFile", formData, config)
          .catch((err) => {
            console.log(err);
            setLoading(false);
          })
          .then((res) => {
            console.log("111111111111\n", res);

            instance
              .get("/VP", {
                params: {
                  holderDID: userDID,
                  fragment: verificationMethod,
                  credentialFile: file.name,
                  challenge: challenge,
                },
              })
              .then((res) => {
                console.log(res);  // "VP was successfully created." if lucky.
                downloadFile
                  .get("/downloadVP", { params: {fragment: verificationMethod} })
                  .then((res) => {
                    FileDownload(
                      res.data,
                      verificationMethod.concat("-presentation.json")
                    );
                    setLoading(false);
                    instance.post("/removeFile");
                  })
                  .catch((err) => {
                    console.log(err);
                    setErrorMessage("Something went wrong while downloading file!");
                    setLoading(false);
                  })})
              .catch((err) => {
                console.log(err);
                setErrorMessage("Something went wrong!");
                setLoading(false);
              });
          });

    };
  };

  return (
    <div id="login">
      <Card style={{ width: '25rem' }} className="DID_card">
        <Container className='login_container'>
          <Form>
              <Form.Group className="mb-3" controlId="user name">
                  <Form.Label>User DID</Form.Label>
                  <Form.Control type="text" placeholder="user name"  value={userDID} onChange={handleUserDID} />
              </Form.Group>
              <Form.Group className="mb-3" controlId="fragment">
                  <Form.Label>Verification Method</Form.Label>
                  <Form.Control type="text" placeholder="fragment"  value={verificationMethod} onChange={handleVM} /> 
              </Form.Group>
              <Form.Group className="mb-3" controlId="challenge">
                  <Form.Label>Challenge</Form.Label>
                  <Form.Control type="text" placeholder="challenge"  value={challenge} onChange={handleChallenge} /> 
              </Form.Group>
              <Form.Group controlId="formFile" className="mb-3">
                <Form.Label>Credential File</Form.Label>
                <Form.Control type="file" accept=".json" onChange={handleFileChange}/>
              </Form.Group>
              <Form.Group>
                  <div className="text-center align-items-center">
                      <button type="button" className="btn btn-outline-light" onClick={handleLogin} disabled={loading} style={{height: "40px", width:'10rem'}}>
                          {loading ? <p>Getting...</p> : <p>Get</p>}
                      </button>
                  </div>
              </Form.Group>
          </Form>
        </Container>
      </Card>
      <div id="errorMessage">
          <p>{errorMessage}</p>
      </div>
      <div id="DID-vp">
          <Card id="display-card">
            <h1>
                Get Verification Presentation
            </h1>
            <p>
              1. Enter correct user DID and fragment.<br/><br/>
              2. Enter the challenge provided by verifier.<br/><br/>
              2. Select the credential json file, which should be provided by issuer.<br/><br/>
              3. Challeng is randomly generated by verifier.
            </p>
          </Card>
      </div>
    </div>
  );
}