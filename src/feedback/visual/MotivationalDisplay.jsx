import { useEffect, useState } from 'react';
import { 
  MUSCLE_GROUP_SPECIFIC_MESSAGES, 
  GENERAL_MOTIVATIONAL_MESSAGES,
  REP_COMPLETION_MESSAGES
} from '../motivationalMessages';
import './MotivationalDisplay.css';

export default function MotivationalDisplay({ 
  muscleGroup, 
  trigger,
  repComplete = false 
}) {
  const [message, setMessage] = useState('');
  const [visible, setVisible] = useState(false);
  const [key, setKey] = useState(0);

  useEffect(() => {
    if (trigger) {
      const muscleMessages = MUSCLE_GROUP_SPECIFIC_MESSAGES[muscleGroup] || [];
      const messagePool = repComplete 
        ? [...REP_COMPLETION_MESSAGES, ...muscleMessages]
        : [...muscleMessages, ...GENERAL_MOTIVATIONAL_MESSAGES];
      
      const randomMessage = messagePool[Math.floor(Math.random() * messagePool.length)];
      
      setMessage(randomMessage);
      setVisible(true);
      setKey(prev => prev + 1); // Force re-render for animation
      
      const timer = setTimeout(() => setVisible(false), 2500);
      return () => clearTimeout(timer);
    }
  }, [trigger, muscleGroup, repComplete]);

  if (!visible) return null;

  return (
    <div key={key} className="motivational-display neon-blue-glow">
      {message}
    </div>
  );
}
