import React, { useState, useEffect } from 'react';

const Information = ({ onChildDataChange }) => {
    const [info, setInfo] = useState({
        name: "",
        did: "",
        country: "",
        birth: ""
    });

    useEffect(() => {
        onChildDataChange(info);
    });

    const handleInputChange = (field, value) => {
        setInfo(prevState => ({
            ...prevState,
            [field]: value
        }));
    };

    return (
        <div>
            <h1> Information </h1>
            <label>Name: </label>
            <input
                type="text"
                onChange={(e) => handleInputChange("name", e.target.value)}
            />
            <br></br>
            <label>DID: </label>
            <input
                type="text"
                onChange={(e) => handleInputChange("did", e.target.value)}
            />
            <br></br>
            <label>Country: </label>
            <input
                type="text"
                onChange={(e) => handleInputChange("country", e.target.value)}
            />
            <br></br>
            <label>Birth: </label>
            <input
                type="text"
                onChange={(e) => handleInputChange("birth", e.target.value)}
            />
            <br></br>
        </div>
    );
}

export default Information;