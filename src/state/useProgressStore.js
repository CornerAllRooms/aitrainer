const useProgressStore = create(
  persist(
    (set, get) => ({
      // ... existing state ...

      // Updated loadProgress
      loadProgress: async () => {
        try {
          const response = await fetch('/api/progress', {
            credentials: 'include'
          });
          const data = await response.json();
          
          set({
            workoutHistory: data.history.map(workout => ({
              ...workout,
              id: workout._id
            })),
            personalRecords: data.records
          });
        } catch (error) {
          console.error('Progress load failed:', error);
        }
      },

      // Updated completeWorkout
      completeWorkout: async () => {
        const { currentWorkout } = get();
        if (!currentWorkout) return;

        try {
          const response = await fetch('/api/progress/save', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'X-CSRF-Token': useUserStore.getState().csrfToken
            },
            body: JSON.stringify({
              startTime: currentWorkout.startTime,
              exercises: currentWorkout.completedExercises
            }),
            credentials: 'include'
          });

          if (response.ok) {
            await get().loadProgress(); // Refresh data
            set({ currentWorkout: null });
          }
        } catch (error) {
          console.error('Workout save failed:', error);
        }
      }
    }),
    {
      name: 'workout-progress',
      storage: {
        getItem: () => Promise.resolve(null),
        setItem: () => Promise.resolve(),
        removeItem: () => Promise.resolve()
      }
    }
  )
);