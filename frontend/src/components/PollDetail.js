import React from 'react';

function PollDetail({ poll }) {
  return (
    <div className="PollDetail">
      <h2>{poll.question}</h2>
      <ul>
        {poll.options.map((option, index) => (
          <li key={index}>{option}</li>
        ))}
      </ul>
    </div>
  );
}

export default PollDetail;