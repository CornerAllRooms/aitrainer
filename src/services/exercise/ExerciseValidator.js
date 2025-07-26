import exercises from '../data/exercises';

export default class ExerciseValidator {
  constructor(exerciseId) {
    this.exercise = exercises[exerciseId];
    this.lastAngles = {};
    this.repCount = 0;
    this.repState = false;
  }

  validatePose(keypoints) {
    const angles = this.calculateAngles(keypoints);
    const errors = this.checkForm(angles);
    const repDetected = this.checkRep(angles);
    
    this.lastAngles = angles;
    return { angles, errors, repDetected, repCount: this.repCount };
  }

  calculateAngles(keypoints) {
    const angles = {};
    const joints = {
      'left_elbow': [keypoints.left_wrist, keypoints.left_elbow, keypoints.left_shoulder],
      'right_elbow': [keypoints.right_wrist, keypoints.right_elbow, keypoints.right_shoulder],
      'left_shoulder': [keypoints.left_elbow, keypoints.left_shoulder, keypoints.left_hip],
      'right_shoulder': [keypoints.right_elbow, keypoints.right_shoulder, keypoints.right_hip],
      'left_hip': [keypoints.left_shoulder, keypoints.left_hip, keypoints.left_knee],
      'right_hip': [keypoints.right_shoulder, keypoints.right_hip, keypoints.right_knee],
      'left_knee': [keypoints.left_hip, keypoints.left_knee, keypoints.left_ankle],
      'right_knee': [keypoints.right_hip, keypoints.right_knee, keypoints.right_ankle]
    };

    Object.entries(joints).forEach(([name, [a, b, c]]) => {
      if (a && b && c) {
        angles[name] = this.calculateAngle(a, b, c);
      }
    });

    return angles;
  }

  calculateAngle(a, b, c) {
    const ab = [b.x - a.x, b.y - a.y];
    const cb = [b.x - c.x, b.y - c.y];
    
    const dot = ab[0] * cb[0] + ab[1] * cb[1];
    const cross = ab[0] * cb[1] - ab[1] * cb[0];
    
    return Math.atan2(cross, dot) * (180 / Math.PI);
  }

  checkForm(angles) {
    const errors = [];
    this.exercise.idealAngles.forEach(({ joint, min, max }) => {
      const angle = angles[joint];
      if (angle < min || angle > max) {
        errors.push({
          joint,
          angle,
          message: `${joint} angle should be between ${min}°-${max}° (current: ${angle.toFixed(1)}°)`
        });
      }
    });
    return errors;
  }

  checkRep(angles) {
    const { repTrigger } = this.exercise;
    const currentAngle = angles[repTrigger.joint];
    
    if (!currentAngle) return false;

    const isTriggered = repTrigger.direction === 'above' 
      ? currentAngle > repTrigger.threshold 
      : currentAngle < repTrigger.threshold;

    if (isTriggered && !this.repState) {
      this.repCount++;
      this.repState = true;
      return true;
    } else if (!isTriggered && this.repState) {
      this.repState = false;
    }
    return false;
  }
}
