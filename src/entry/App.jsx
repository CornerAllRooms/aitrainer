import React, { useState, useEffect } from 'react';
import { FaUser } from 'react-icons/fa';
import { useNavigate } from 'react-router-dom';
import '/app.css'; // Updated CSS import path

function App() {
  const [activeButton, setActiveButton] = useState(null);
  const [authChecked, setAuthChecked] = useState(false);
  const navigate = useNavigate();

  // Enhanced authentication check
  useEffect(() => {
    const verifyAuth = async () => {
      try {
        // 1. Check for token in cookies (set by PHP)
        const response = await fetch('/api/auth/verify', {
          credentials: 'include', // Sends cookies automatically
        });

        if (!response.ok) {
          throw new Error('Invalid session');
        }

        // 2. If valid, proceed with app
        setAuthChecked(true);

      } catch (error) {
        // 3. Redirect to PHP login on failure
        window.location.href = '/index.php?error=session_expired';
      }
    };

    verifyAuth();
  }, []);

  const handleButtonClick = (buttonId) => {
    setActiveButton(buttonId);
    // Use React Router for navigation
    navigate(`/${buttonId}`);
  };

  const handleProfileClick = (e) => {
    e.preventDefault();
    // Redirect to PHP profile page
    window.location.href = '/profile.php';
  };

  if (!authChecked) {
    return (
      <div className="auth-loading">
        <p>Verifying session...</p>
      </div>
    );
  }

  return (
    <div className="dashboard">
      <header className="header">
        <div className="logo">
          AI Trainer
        </div>
        <div className="user-menu">
          <form onSubmit={handleProfileClick}>
            <button type="submit" className="user-avatar">
              <FaUser />
            </button>
          </form>
        </div>
      </header>

      <main className="main-content">
        <div className="welcome-message">
          <h1>Welcome to Your AI Training System</h1>
          <p>Select an area to begin your personalized fitness journey powered by artificial intelligence</p>
        </div>

        <div className="action-buttons">
          {['nutrition', 'exercise', 'composition'].map((section) => (
            <button
              key={section}
              className={`action-btn ${activeButton === section ? 'active' : ''}`}
              onClick={() => handleButtonClick(section)}
              aria-label={section.charAt(0).toUpperCase() + section.slice(1)}
            >
              <div className={`sketch-icon ${section}-icon`} />
              <h3>{section.charAt(0).toUpperCase() + section.slice(1)}</h3>
              <p>
                {section === 'nutrition' && 'Personalized meal plans'}
                {section === 'exercise' && 'AI-powered workouts'}
                {section === 'composition' && 'Track your progress'}
              </p>
            </button>
          ))}
        </div>
      </main>
    </div>
  );
}

export default App;