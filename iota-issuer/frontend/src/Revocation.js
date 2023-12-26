import React, { useState, useEffect } from 'react';

const Revocation = ({ onChildDataChange }) => {
    const [revocationIdx, setrevocationIdx] = useState("");

    useEffect(() => {
        onChildDataChange(revocationIdx);
    });

    const handleInputChange = (value) => {
        setrevocationIdx(prevState => ({
            ...prevState,
            revocationIdx: value
        }));
    };

    return (
        <div>
            <h2>Revocation</h2>
            <label>revocation index: </label>
            <input
                type="text"
                onChange={(e) => handleInputChange(e.target.value)
                }
            />
        </div>
    );

}

export default Revocation;