import React, { useState, useEffect } from 'react';
import { FaUser, FaUtensils, FaDumbbell, FaChartLine } from 'react-icons/fa';
import { useNavigate } from 'react-router-dom';
import '/app.css';
import { WorkoutProvider } from '@/state/WorkoutContext';
import { WasmProvider } from '@/libs/wasm/WasmContext';
import { AuthProvider } from '@/state/AuthContext';

function RootApp() {
  return (
    <AuthProvider>
      <WasmProvider>
        <WorkoutProvider>
          <App />
        </WorkoutProvider>
      </WasmProvider>
    </AuthProvider>
  );

function App() {
  const [activeButton, setActiveButton] = useState(null);
  const [authChecked, setAuthChecked] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    const verifyAuth = async () => {
      try {
        const response = await fetch('/server/api/auth/verify', {
          credentials: 'include',
        });

        if (!response.ok) throw new Error('Invalid session');
        setAuthChecked(true);
      } catch (error) {
        window.location.href = '/server/public/index.php?error=session_expired';
      }
    };
    verifyAuth();
  }, []);

  const handleNavigation = (section) => {
    setActiveButton(section);
    // Use PHP endpoints for full page reloads
    if (section === 'nutrition') window.location.href = '/server/public/profile.php?section=nutrition';
    else if (section === 'composition') window.location.href = '/bc.html';
    else window.location.href = '/server/public/profile.php?section=exercise';
  };

  if (!authChecked) {
    return <div className="auth-loading">Verifying session...</div>;
  }

  return (
    <div className="dashboard">
      <header className="header">
        <div className="logo">AI Trainer</div>
        <div className="user-menu">
          <button onClick={() => window.location.href = '/server/public/profile.php'} className="user-avatar">
            <FaUser />
          </button>
        </div>
      </header>

      <main className="main-content">
        <div className="welcome-message">
          <h1>Welcome to Your AI Training System</h1>
          <p>Select an area to begin your personalized fitness journey</p>
        </div>

        <div className="action-buttons">
          {[
            { id: 'nutrition', icon: <FaUtensils />, desc: 'Personalized meal plans' },
            { id: 'exercise', icon: <FaDumbbell />, desc: 'AI-powered workouts' },
            { id: 'composition', icon: <FaChartLine />, desc: 'Track your progress' }
          ].map((item) => (
            <button
              key={item.id}
              className={`action-btn ${activeButton === item.id ? 'active' : ''}`}
              onClick={() => handleNavigation(item.id)}
            >
              <div className="action-icon">{item.icon}</div>
              <h3>{item.id.charAt(0).toUpperCase() + item.id.slice(1)}</h3>
              <p>{item.desc}</p>
            </button>
          ))}
        </div>
      </main>
    </div>
  );
}
}
export default App;
