import React from 'react';
import { useState } from 'react';
import { FaUser } from 'react-icons/fa';
import './App.css';

useEffect(() => {
  if (!localStorage.getItem('authToken')) {
    window.location.href = '/';
  }
}, []);

function App() {
  const [activeButton, setActiveButton] = useState(null);

  const handleButtonClick = (buttonId) => {
    setActiveButton(buttonId);
    // In a real app, you would navigate to the corresponding page
    console.log(`Navigating to ${buttonId}`);
  };

  return (
  <><div className="dashboard">
      <header className="header">
        <div className="logo">
          AI Trainer
        </div>
        <div className="user-menu">
          <div className="user-avatar" title="User Profile">
          <form action="./profile.php" method="POST">
            <FaUser />
          </form>
        </div>
      </div>
    </header><main className="main-content">
        <div className="welcome-message">
          <h1>Welcome to Your AI Training System</h1>
          <p>Select an area to begin your personalized fitness journey powered by artificial intelligence</p>
        </div>

        <div className="action-buttons">
          <div
            className={`action-btn ${activeButton === 'nutrition' ? 'active' : ''}`}
            id="nutrition-btn"
            onClick={() => handleButtonClick('nutrition')}
          >
            <div className="sketch-icon nutrition-icon"></div>
            <h3>Nutrition</h3>
            <form action="./nutrition.jsx" method="POST"></form>
            <p>Personalized meal plans</p>
          </div>

          <div
            className={`action-btn ${activeButton === 'exercise' ? 'active' : ''}`}
            id="exercise-btn"
            onClick={() => handleButtonClick('exercise')}
          >
            <div className="sketch-icon exercise-icon"></div>
            <h3>Exercise</h3>
              <form action="./src/components/Exercises/MuscleMap.jsx" method="POST"></form>
            <p>AI-powered workouts</p>
          </div>

          <div
            className={`action-btn ${activeButton === 'composition' ? 'active' : ''}`}
            id="composition-btn"
            onClick={() => handleButtonClick('composition')}
          >
            <div className="sketch-icon composition-icon"></div>
            <h3>Body Composition</h3>
            <form action="./nutrition.jsx" method="POST"></form>
            <p>Track your progress</p>
          </div>
        </div>
      </main>
    </div>
    </>
  );
}

export default App;