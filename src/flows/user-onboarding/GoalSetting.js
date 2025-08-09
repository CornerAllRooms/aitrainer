import React, { useState, useEffect } from 'react';
import { FaDumbbell, FaRunning, FaWeight, FaHeartbeat } from 'react-icons/fa';
import { useGoalMiddleware } from '@/middleware/goalMiddleware';
import './app.css'; // Reusing your existing styles

const GoalSetting = ({ onComplete }) => {
  const [goals, setGoals] = useState({
    primaryGoal: '',
    experienceLevel: '',
    weeklyTarget: 3,
    equipmentAvailable: []
  });
  const [isLoading, setIsLoading] = useState(true);
  const { getCachedGoals, cacheGoals, validateGoals } = useGoalMiddleware();

  // Load cached goals on mount
  useEffect(() => {
    const loadCachedData = async () => {
      const cached = await getCachedGoals();
      if (cached) {
        setGoals(cached);
      }
      setIsLoading(false);
    };
    loadCachedData();
  }, []);

  const handleChange = (field, value) => {
    const updated = { ...goals, [field]: value };
    setGoals(updated);
    cacheGoals(updated); // Cache on every change
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    const validation = await validateGoals(goals);
    if (validation.valid) {
      onComplete(goals);
    } else {
      alert(validation.message);
    }
  };

  if (isLoading) {
    return <div className="auth-loading">Loading your goals...</div>;
  }

  return (
    <div className="dashboard">
      <header className="header">
        <div className="logo">AI Trainer</div>
      </header>

      <main className="main-content">
        <div className="welcome-message">
          <h1>Define Your Fitness Journey</h1>
          <p>Set your goals to personalize your experience</p>
        </div>

        <form onSubmit={handleSubmit} className="goal-form">
          {/* Primary Goal Selection */}
          <div className="form-section">
            <h3>
              <FaDumbbell className="section-icon" />
              Primary Goal
            </h3>
            <div className="option-grid">
              {['Weight Loss', 'Muscle Gain', 'Endurance', 'General Fitness'].map(goal => (
                <div 
                  key={goal}
                  className={`option-card ${goals.primaryGoal === goal ? 'active' : ''}`}
                  onClick={() => handleChange('primaryGoal', goal)}
                >
                  {goal}
                </div>
              ))}
            </div>
          </div>

          {/* Experience Level */}
          <div className="form-section">
            <h3>
              <FaRunning className="section-icon" />
              Your Experience Level
            </h3>
            <div className="option-row">
              {['Beginner', 'Intermediate', 'Advanced'].map(level => (
                <button
                  type="button"
                  key={level}
                  className={`toggle-option ${goals.experienceLevel === level ? 'active' : ''}`}
                  onClick={() => handleChange('experienceLevel', level)}
                >
                  {level}
                </button>
              ))}
            </div>
          </div>

          {/* Weekly Target */}
          <div className="form-section">
            <h3>
              <FaHeartbeat className="section-icon" />
              Weekly Workout Target
            </h3>
            <div className="range-input">
              <input
                type="range"
                min="1"
                max="7"
                value={goals.weeklyTarget}
                onChange={(e) => handleChange('weeklyTarget', parseInt(e.target.value))}
              />
              <div className="range-labels">
                <span>{goals.weeklyTarget} days/week</span>
              </div>
            </div>
          </div>

          {/* Equipment */}
          <div className="form-section">
            <h3>
              <FaWeight className="section-icon" />
              Available Equipment
            </h3>
            <div className="checkbox-grid">
              {['Dumbbells', 'Resistance Bands', 'Yoga Mat', 'None'].map(item => (
                <label key={item} className="checkbox-option">
                  <input
                    type="checkbox"
                    checked={goals.equipmentAvailable.includes(item)}
                    onChange={() => {
                      const updated = goals.equipmentAvailable.includes(item)
                        ? goals.equipmentAvailable.filter(e => e !== item)
                        : [...goals.equipmentAvailable, item];
                      handleChange('equipmentAvailable', updated);
                    }}
                  />
                  {item}
                </label>
              ))}
            </div>
          </div>

          <div className="form-actions">
            <button type="submit" className="action-btn active">
              Save & Continue
            </button>
          </div>
        </form>
      </main>
    </div>
  );
};

export default GoalSetting;