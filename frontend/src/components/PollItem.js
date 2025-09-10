import React from 'react';

function PollItem({ poll, onVote }) {
  const totalVotes = poll.votes.reduce((sum, vote) => sum + vote, 0);

  return (
    <div className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition duration-200">
      <h3 className="text-xl font-semibold text-gray-800 mb-4">{poll.title}</h3>
      
      <div className="space-y-3">
        {poll.options.map((option, index) => {
          const voteCount = poll.votes[index] || 0;
          const percentage = totalVotes > 0 ? (voteCount / totalVotes) * 100 : 0;
          
          return (
            <div key={index} className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-gray-700 font-medium">{option}</span>
                <span className="text-sm text-gray-500">
                  {voteCount} vote{voteCount !== 1 ? 's' : ''} ({percentage.toFixed(1)}%)
                </span>
              </div>
              
              {/* Progress bar */}
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div 
                  className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                  style={{ width: `${percentage}%` }}
                ></div>
              </div>
              
              <button
                onClick={() => onVote(index)}
                className="w-full bg-blue-50 text-blue-600 hover:bg-blue-100 px-4 py-2 rounded-md text-sm font-medium transition duration-200 border border-blue-200"
              >
                Vote
              </button>
            </div>
          );
        })}
      </div>
      
      <div className="mt-4 pt-4 border-t border-gray-200">
        <p className="text-sm text-gray-500">
          Total votes: {totalVotes} • Created: {new Date(poll.created_at).toLocaleDateString()}
        </p>
      </div>
    </div>
  );
}

export default PollItem;