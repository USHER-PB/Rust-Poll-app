import React from 'react';
import { Link } from 'react-router-dom';

function Navbar({ onLogout, user }) {
  return (
    <nav className="bg-white/10 backdrop-blur-lg shadow-2xl border-b border-white/20">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          <div className="flex items-center">
            <Link to="/" className="flex-shrink-0 flex items-center group">
              <div className="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl flex items-center justify-center mr-3 shadow-lg group-hover:scale-110 transition-transform duration-200">
                <svg className="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <span className="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                Poll App
              </span>
            </Link>
          </div>

          <div className="flex items-center space-x-6">
            <Link
              to="/polls"
              className="text-gray-700 hover:text-blue-600 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 hover:bg-white/20 backdrop-blur-sm"
            >
              Polls
            </Link>
            <Link
              to="/create"
              className="bg-gradient-to-r from-blue-500 to-purple-600 text-white px-6 py-2 rounded-lg text-sm font-medium transition-all duration-200 hover:from-blue-600 hover:to-purple-700 shadow-lg transform hover:scale-105"
            >
              Create Poll
            </Link>
            <div className="flex items-center space-x-3">
              <span className="text-sm text-gray-600 bg-white/20 px-3 py-1 rounded-full backdrop-blur-sm">
                Welcome, {user?.name}
              </span>
              <button
                onClick={onLogout}
                className="text-gray-700 hover:text-red-600 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 hover:bg-red-50 backdrop-blur-sm"
              >
                Logout
              </button>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
}

export default Navbar;

