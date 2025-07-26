import { AudioFeedback } from './audio/AudioFeedback';
import MotivationalDisplay from './visual/MotivationalDisplay';
import { 
  MUSCLE_GROUP_SPECIFIC_MESSAGES,
  GENERAL_MOTIVATIONAL_MESSAGES,
  REP_COMPLETION_MESSAGES
} from './motivationalMessages';

export class FeedbackManager {
  constructor() {
    this.audio = new AudioFeedback();
    this.lastMessageTime = 0;
    this.messageCooldown = 5000; // 5 seconds between messages
    this.repCount = 0;
  }

  getRandomMessage(muscleGroup, isRepComplete = false) {
    const muscleMessages = MUSCLE_GROUP_SPECIFIC_MESSAGES[muscleGroup] || [];
    const generalMessages = isRepComplete 
      ? [...REP_COMPLETION_MESSAGES, ...muscleMessages]
      : [...muscleMessages, ...GENERAL_MOTIVATIONAL_MESSAGES];
    
    return generalMessages[Math.floor(Math.random() * generalMessages.length)];
  }

  async provideFeedback(muscleGroup, isRepComplete = false) {
    const now = Date.now();
    if (now - this.lastMessageTime < this.messageCooldown) return null;
    
    this.lastMessageTime = now;
    this.repCount++;
    
    // Play audio feedback
    this.audio.playSuccessTone(this.repCount % 4);
    const message = this.getRandomMessage(muscleGroup, isRepComplete);
    await this.audio.speakMessage(message);
    
    // Return visual component
    return (
      <MotivationalDisplay
        key={Date.now()}
        muscleGroup={muscleGroup}
        trigger={true}
        repComplete={isRepComplete}
      />
    );
  }
}
