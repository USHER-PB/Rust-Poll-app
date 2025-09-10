import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import LandingPage from './components/LandingPage';
import Login from './components/Login';
import Register from './components/Register';
import PollList from './components/PollList';
import PollForm from './components/PollForm';
import PollDetail from './components/PollDetail';
import Navbar from './components/Navbar';

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [user, setUser] = useState(null);

  useEffect(() => {
    // Check if user is logged in (check for JWT token)
    const token = localStorage.getItem('jwt_token');
    if (token) {
      setIsAuthenticated(true);
      // You could decode the JWT here to get user info
    }
  }, []);

  const handleLogin = (token, userData) => {
    localStorage.setItem('jwt_token', token);
    setIsAuthenticated(true);
    setUser(userData);
  };

  const handleLogout = () => {
    localStorage.removeItem('jwt_token');
    setIsAuthenticated(false);
    setUser(null);
  };

  return (
    <Router>
      <div className="App">
        {isAuthenticated && <Navbar onLogout={handleLogout} user={user} />}
        <Routes>
          <Route path="/" element={<LandingPage isAuthenticated={isAuthenticated} />} />
          <Route path="/login" element={<Login onLogin={handleLogin} />} />
          <Route path="/register" element={<Register onLogin={handleLogin} />} />
          <Route 
            path="/polls" 
            element={isAuthenticated ? <PollList /> : <Navigate to="/login" />} 
          />
          <Route 
            path="/create" 
            element={isAuthenticated ? <PollForm /> : <Navigate to="/login" />} 
          />
          <Route 
            path="/poll/:id" 
            element={isAuthenticated ? <PollDetail /> : <Navigate to="/login" />} 
          />
        </Routes>
      </div>
    </Router>
  );
}

export default App;