import { GoogleTTS } from './GoogleTTS';
import feedbackConfig from '../../../config/feedbackConfig.json';

const tts = new GoogleTTS();
const audioElements = {};

export async function playFeedback(type) {
  const { variants, config } = feedbackConfig.audio[type];
  const text = variants[Math.floor(Math.random() * variants.length)];

  try {
    if (!audioElements[text]) {
      const audioPath = await tts.synthesize(text, config);
      audioElements[text] = new Audio(audioPath);
    }

    audioElements[text].currentTime = 0;
    await audioElements[text].play();
  } catch (error) {
    console.error('Playback Error:', error);
    // Fallback to browser TTS
    const utterance = new SpeechSynthesisUtterance(text);
    utterance.rate = config.speakingRate;
    window.speechSynthesis.speak(utterance);
  }
}