import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import './DifficultyTabs.css'; // Create this CSS file for styling

const DifficultyTabs = ({ 
  onDifficultyChange,
  initialDifficulty = 'all',
  showIcons = true,
  compact = false
}) => {
  const [activeTab, setActiveTab] = useState(initialDifficulty);
  const [isMounted, setIsMounted] = useState(false);

  // Difficulty configurations
  const difficultyLevels = [
    { id: 'all', label: 'All Levels', icon: '🌟' },
    { id: 'beginner', label: 'Beginner', icon: '🟢' },
    { id: 'intermediate', label: 'Intermediate', icon: '🟡' },
    { id: 'advanced', label: 'Advanced', icon: '🔴' }
  ];

  // Handle tab changes with animation
  const handleTabChange = (difficulty) => {
    if (difficulty === activeTab) return;
    
    setActiveTab(difficulty);
    if (onDifficultyChange) {
      onDifficultyChange(difficulty);
    }
  };

  // Add mount animation
  useEffect(() => {
    setIsMounted(true);
    return () => setIsMounted(false);
  }, []);

  return (
    <div className={`difficulty-tabs ${compact ? 'compact' : ''} ${isMounted ? 'mounted' : ''}`}>
      <div className="tabs-header">
        <h3 className="tabs-title">Filter by Difficulty</h3>
      </div>
      
      <div className="tabs-container">
        {difficultyLevels.map((level) => (
          <button
            key={level.id}
            className={`tab-item ${activeTab === level.id ? 'active' : ''}`}
            onClick={() => handleTabChange(level.id)}
            aria-label={`Show ${level.label} exercises`}
            data-difficulty={level.id}
          >
            {showIcons && <span className="tab-icon">{level.icon}</span>}
            <span className="tab-label">{level.label}</span>
          </button>
        ))}
      </div>

      {/* Visual indicator for active tab */}
      <div className="active-indicator" data-active-tab={activeTab} />
    </div>
  );
};

DifficultyTabs.propTypes = {
  onDifficultyChange: PropTypes.func,
  initialDifficulty: PropTypes.oneOf(['all', 'beginner', 'intermediate', 'advanced']),
  showIcons: PropTypes.bool,
  compact: PropTypes.bool
};

export default DifficultyTabs;