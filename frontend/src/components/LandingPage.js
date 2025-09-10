import React from 'react';
import { Link } from 'react-router-dom';

function LandingPage({ isAuthenticated }) {
  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-900 via-purple-900 to-pink-900 relative overflow-hidden">
      {/* Animated background elements */}
      <div className="absolute inset-0">
        <div className="absolute top-20 left-20 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob"></div>
        <div className="absolute top-40 right-20 w-72 h-72 bg-yellow-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-2000"></div>
        <div className="absolute -bottom-8 left-40 w-72 h-72 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-4000"></div>
      </div>

      <div className="relative z-10 container mx-auto px-4 py-16">
        <div className="text-center">
          {/* Enhanced App Logo/Image */}
          <div className="mb-12">
            <div className="w-40 h-40 mx-auto bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 rounded-full flex items-center justify-center shadow-2xl transform hover:scale-110 transition-all duration-300 animate-pulse relative">
              {/* Outer ring */}
              <div className="absolute inset-0 rounded-full border-4 border-white/30 animate-spin-slow"></div>
              
              {/* Main icon container */}
              <div className="w-24 h-24 bg-white rounded-full flex items-center justify-center shadow-lg relative z-10">
                {/* Poll icon with multiple elements */}
                <div className="relative">
                  {/* Main circle */}
                  <div className="w-12 h-12 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
                    <svg className="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </div>
                  
                  {/* Floating elements around the main icon */}
                  <div className="absolute -top-2 -right-2 w-4 h-4 bg-yellow-400 rounded-full animate-bounce"></div>
                  <div className="absolute -bottom-2 -left-2 w-3 h-3 bg-pink-400 rounded-full animate-bounce animation-delay-1000"></div>
                  <div className="absolute -top-1 -left-1 w-2 h-2 bg-green-400 rounded-full animate-bounce animation-delay-2000"></div>
                </div>
              </div>
              
              {/* Decorative dots */}
              <div className="absolute -top-4 -right-4 w-3 h-3 bg-white/60 rounded-full animate-pulse"></div>
              <div className="absolute -bottom-4 -left-4 w-2 h-2 bg-white/60 rounded-full animate-pulse animation-delay-1000"></div>
              <div className="absolute top-1/2 -right-6 w-2 h-2 bg-white/60 rounded-full animate-pulse animation-delay-2000"></div>
              <div className="absolute top-1/2 -left-6 w-2 h-2 bg-white/60 rounded-full animate-pulse animation-delay-3000"></div>
            </div>
          </div>

          {/* Enhanced App Title */}
          <h1 className="text-6xl font-bold text-white mb-6 drop-shadow-2xl">
            <span className="bg-gradient-to-r from-blue-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
              Poll App
            </span>
          </h1>
          
          {/* Enhanced App Description */}
          <p className="text-xl text-gray-200 mb-12 max-w-3xl mx-auto leading-relaxed">
            Create engaging polls, gather opinions, and make data-driven decisions. 
            Our intuitive polling platform helps you collect feedback quickly and efficiently.
          </p>

          {/* Enhanced Features */}
          <div className="grid md:grid-cols-3 gap-8 mb-16 max-w-6xl mx-auto">
            <div className="bg-white/10 backdrop-blur-lg p-8 rounded-2xl shadow-2xl border border-white/20 transform hover:scale-105 transition-all duration-300 hover:bg-white/20">
              <div className="w-16 h-16 bg-gradient-to-r from-blue-400 to-blue-600 rounded-2xl flex items-center justify-center mb-6 mx-auto shadow-lg">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                </svg>
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Create Polls</h3>
              <p className="text-gray-300 leading-relaxed">Easily create custom polls with multiple options and real-time results. Beautiful, intuitive interface for seamless poll creation.</p>
            </div>

            <div className="bg-white/10 backdrop-blur-lg p-8 rounded-2xl shadow-2xl border border-white/20 transform hover:scale-105 transition-all duration-300 hover:bg-white/20">
              <div className="w-16 h-16 bg-gradient-to-r from-green-400 to-green-600 rounded-2xl flex items-center justify-center mb-6 mx-auto shadow-lg">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Real-time Results</h3>
              <p className="text-gray-300 leading-relaxed">See voting results instantly with beautiful charts and statistics. Watch your polls come to life in real-time.</p>
            </div>

            <div className="bg-white/10 backdrop-blur-lg p-8 rounded-2xl shadow-2xl border border-white/20 transform hover:scale-105 transition-all duration-300 hover:bg-white/20">
              <div className="w-16 h-16 bg-gradient-to-r from-purple-400 to-purple-600 rounded-2xl flex items-center justify-center mb-6 mx-auto shadow-lg">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                </svg>
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Secure & Private</h3>
              <p className="text-gray-300 leading-relaxed">Your polls are secure with user authentication and data protection. Your privacy is our top priority.</p>
            </div>
          </div>

          {/* Enhanced Action Buttons */}
          {!isAuthenticated ? (
            <div className="space-x-6">
              <Link
                to="/register"
                className="inline-block bg-gradient-to-r from-blue-500 to-purple-600 text-white px-10 py-4 rounded-full font-bold text-lg hover:from-blue-600 hover:to-purple-700 transition-all duration-300 shadow-2xl hover:shadow-purple-500/25 transform hover:scale-105"
              >
                Get Started
              </Link>
              <Link
                to="/login"
                className="inline-block bg-white/10 backdrop-blur-lg text-white px-10 py-4 rounded-full font-bold text-lg border-2 border-white/30 hover:bg-white/20 transition-all duration-300 shadow-2xl transform hover:scale-105"
              >
                Sign In
              </Link>
            </div>
          ) : (
            <div className="space-x-6">
              <Link
                to="/polls"
                className="inline-block bg-gradient-to-r from-blue-500 to-purple-600 text-white px-10 py-4 rounded-full font-bold text-lg hover:from-blue-600 hover:to-purple-700 transition-all duration-300 shadow-2xl hover:shadow-purple-500/25 transform hover:scale-105"
              >
                View Polls
              </Link>
              <Link
                to="/create"
                className="inline-block bg-gradient-to-r from-green-500 to-teal-600 text-white px-10 py-4 rounded-full font-bold text-lg hover:from-green-600 hover:to-teal-700 transition-all duration-300 shadow-2xl hover:shadow-teal-500/25 transform hover:scale-105"
              >
                Create Poll
              </Link>
            </div>
          )}
        </div>
      </div>

      {/* Floating particles */}
      <div className="absolute inset-0 pointer-events-none">
        {[...Array(20)].map((_, i) => (
          <div
            key={i}
            className="absolute w-2 h-2 bg-white/30 rounded-full animate-float"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              animationDelay: `${Math.random() * 3}s`,
              animationDuration: `${3 + Math.random() * 2}s`
            }}
          />
        ))}
      </div>
    </div>
  );
}

export default LandingPage;

