import { TextToSpeechClient } from '@google-cloud/text-to-speech';
import path from 'path';
import { promises as fs } from 'fs';

const CACHE_DIR = path.resolve(process.cwd(), '../../public/audio/tts');

export class GoogleTTS {
  constructor() {
    this.client = new TextToSpeechClient({
      keyFilename: path.join(process.cwd(), '../../google-tts-key.json')
    });
  }

  async synthesize(text, config) {
    const filename = `${text.replace(/[^a-z0-9]/gi, '_').toLowerCase()}.mp3`;
    const filePath = path.join(CACHE_DIR, filename);

    try {
      // Check cache
      try {
        await fs.access(filePath);
        return `/audio/tts/${filename}`;
      } catch {}

      // Generate new audio
      const [response] = await this.client.synthesizeSpeech({
        input: { text },
        voice: config.voice,
        audioConfig: {
          audioEncoding: config.audioEncoding,
          effectsProfileId: config.effectsProfileId,
          pitch: config.pitch,
          speakingRate: config.speakingRate
        }
      });

      await fs.mkdir(CACHE_DIR, { recursive: true });
      await fs.writeFile(filePath, response.audioContent, 'binary');
      return `/audio/tts/${filename}`;

    } catch (error) {
      console.error('TTS Error:', error);
      throw error;
    }
  }
}