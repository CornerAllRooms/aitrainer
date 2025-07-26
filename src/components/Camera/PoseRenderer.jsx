import SkeletonView from './components/SkeletonView';
import AngleIndicator from './components/AngleIndicator';
import FeedbackDisplay from './FeedbackDisplay';

export default function PoseRenderer({ exerciseId, keypoints, formErrors }) {
  return (
    <div className="absolute inset-0 pointer-events-none">
      {keypoints && (
        <>
          <SkeletonView keypoints={keypoints} />
          <AngleIndicator 
            keypoints={keypoints} 
            exerciseId={exerciseId}
          />
          <FeedbackDisplay errors={formErrors} />
        </>
      )}
    </div>
  );
}
