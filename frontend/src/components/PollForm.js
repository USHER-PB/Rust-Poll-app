import React, { useState } from 'react';

function PollForm() {
  const [question, setQuestion] = useState('');
  const [options, setOptions] = useState(['', '']);

  const addOption = () => {
    setOptions([...options, '']);
  };

  const submitPoll = (e) => {
    e.preventDefault();
    // Handle poll submission
  };

  return (
    <div className="PollForm">
      <h2>Create Poll</h2>
      <form onSubmit={submitPoll}>
        <div>
          <label>Poll Question:</label>
          <input
            type="text"
            value={question}
            onChange={(e) => setQuestion(e.target.value)}
          />
        </div>
        {options.map((option, index) => (
          <div key={index}>
            <label>Option {index + 1}:</label>
            <input
              type="text"
              value={option}
              onChange={(e) => {
                const newOptions = [...options];
                newOptions[index] = e.target.value;
                setOptions(newOptions);
              }}
            />
          </div>
        ))}
        <button type="button" onClick={addOption}>
          Add Option
        </button>
        <button type="submit">Create Poll</button>
      </form>
    </div>
  );
}

export default PollForm;