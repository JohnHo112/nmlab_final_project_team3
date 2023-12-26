import React, { useState } from 'react';
import instance from "../api";
import Card from 'react-bootstrap/Card';
import { JsonView } from 'react-json-view-lite';
import 'react-json-view-lite/dist/index.css';
import Form from 'react-bootstrap/Form';
import Container from 'react-bootstrap/Container';

export default function CreateDID(){
    const [errorMessage, setErrorMessage] = useState("");
    const [verificationMethod, setVerificationMethod] = useState("");
    const [loading, setLoading] = useState(false);
    const [iotaDID, setIotaDID] = useState("");

    const handleLogin = async (e) => {
        setLoading(true);
        setErrorMessage("");
        setIotaDID("");
        if(verificationMethod===""){
            setErrorMessage("Please enter verification method!");
            setLoading(false);
        }else{
            console.log("creating DID...");
                instance
                .post("/createDID", {verificationMethod: verificationMethod})
                .then((res) => {
                    const didId = res.data;
                    console.log("DID id: ", didId);
                    setIotaDID(didId);
                    setLoading(false);
                    console.log("DID created successfully!");
                })
                .catch((err) => {
                    setErrorMessage("Something went wrong while creating DID. Please try again later.");
                    console.log(err);
                });
        }
    };

    const handleVM = (e) => {
        setVerificationMethod(e.target.value);
    };
    
    return(
        <div id="login">
            <Card style={{ width: '20rem' }} className="DID_card">
                <Container className='login_container'>
                <Form>
                    <Form.Group className="mb-3" controlId="verification method">
                        <Form.Label>Verification Method</Form.Label>
                        <Form.Control type="text" placeholder="fragment" id="input-bordered" value={verificationMethod} onChange={handleVM} /> 
                    </Form.Group>
                    <Form.Group>
                        <div className="text-center align-items-center">
                            <button type="button" className="btn btn-outline-light" onClick={handleLogin} disabled={loading} style={{height: "40px", width:'10rem'}}>
                                {loading ? <p>Creating...</p> : <p>Create Your DID</p>}
                            </button>
                        </div>
                    </Form.Group>
                </Form>
                </Container>
            </Card>
            <div id="errorMessage" color="red">
                <p>{errorMessage}</p>
            </div>
            <div id="DID-create">
                <Card id="display-card">
                    {(iotaDID!=="" && iotaDID!=="Repeat") ?
                    <div>
                        <h1>Create Digital Identity</h1>
                            <p >
                            1. Please enter any fragment. Memorize the fragment you entered. You'll need it to get VP later.<br/><br/>
                            2. Fragment should not be blank.<br/><br/>
                            3. Click the "Create Your DID" button. It may take a little time to finish.<br/><br/>
                            4. After finishing, you'll see your DID below. Please copy and save it for later use.<br/><br/>
                            5. Click on the logo to go back to the home page.<br/><br/>
                            </p>
                        <p>Your DID : </p>
                        <JsonView id="json-word" data={iotaDID} shouldInitiallyExpand={(level) => true} />
                    </div>
                    :
                    <div>
                        <h1>Create Digital Identity</h1>
                        <p >
                        1. Please enter any fragment. Memorize the fragment you entered. You'll need it to get VP later.<br/><br/>
                        2. Fragment should not be blank.<br/><br/>
                        3. Click the "Create Your DID" button. It may take a little time to finish.<br/><br/>
                        4. After finishing, you'll see your DID here. Please copy and save it for later use.<br/><br/>
                        5. You can click on the logo to go back to the home page.<br/><br/>
                        </p>
                    </div>
                    }
                </Card>
            </div>
            
        </div>

    )

}