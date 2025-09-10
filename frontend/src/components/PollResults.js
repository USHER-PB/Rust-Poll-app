import React from 'react';

function PollResults({ poll }) {
  return (
    <div className="PollResults">
      <h2>Results</h2>
      <ul>
        {poll.options.map((option, index) => (
          <li key={index}>
            {option} - {poll.votes[index] || 0} votes
          </li>
        ))}
      </ul>
    </div>
  );
}

export default PollResults;