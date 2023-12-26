import React from 'react';
// import Card from 'react-bootstrap/Card'
import Col from 'react-bootstrap/Col';
import Row from 'react-bootstrap/Row';
import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import {  Link } from "react-router-dom";


export default function HomePage(){
    
    return(
        <div id="instruction">
            
            
               <Container fluid>
                    <Row md={2} className='g-5 justify-content-md-center'>
                        <Col >
                        <Col md={5} className="text-center text-md-right">
                            {/* <Card className="custom-class" >
                                <Card.Body>
                                    <Card.Title>Create DID</Card.Title>
                                    <Card.Text>
                                        You can create a digital identity with verification method.
                                    </Card.Text> */}
                                    <Link to ={'/createdid'} >
                                        <Button id='home_button' style={{marginTop:"24px"}} fullSized>
                                            Click Here to Create Your Digital Identity !
                                        </Button>
                                    </Link>
                                {/* </Card.Body>
                            </Card> */}
                        </Col>
                        <Col md={5} className="text-center text-md-right">     
                            {/* <Card className='custom-class'>
                                <Card.Body>
                                <Card.Title>Get Verification Presentation </Card.Title>
                                <Card.Text>
                                     You need to upload the verification credential file and the challenge, so as to get verification presentation.
                                </Card.Text> */}
                                <Link to ={'/getvp'} >
                                    <Button id='home_button'>
                                        Click Here to Get Verification Presentation !
                                    </Button>
                                </Link>
                                {/* </Card.Body>
                             </Card> */}
                        </Col>
                        </Col>
                    </Row>    
                </Container>
        </div>)
}