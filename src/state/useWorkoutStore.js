import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import axios from 'axios';

// Configure axios for CSRF protection
axios.defaults.withCredentials = true;
axios.interceptors.request.use(config => {
  const csrfToken = document.cookie
    .split('; ')
    .find(row => row.startsWith('XSRF-TOKEN='))
    ?.split('=')[1];
  
  if (csrfToken) {
    config.headers['X-XSRF-TOKEN'] = csrfToken;
  }
  return config;
});

const useWorkoutStore = create(
  persist(
    (set, get) => ({
      // Workout state
      workouts: [],
      currentWorkout: null,
      activeSession: null,
      isLoading: false,
      error: null,

      // Session synchronization
      lastSync: null,
      syncStatus: 'idle', // 'idle' | 'syncing' | 'error'

      // Fetch workouts from server
      fetchWorkouts: async () => {
        set({ isLoading: true, error: null });
        try {
          const response = await axios.get('/api/workouts');
          set({
            workouts: response.data,
            lastSync: new Date().toISOString(),
            isLoading: false
          });
        } catch (error) {
          set({ 
            error: error.response?.data?.message || error.message,
            isLoading: false,
            syncStatus: 'error'
          });
        }
      },

      // Start new workout session
      startWorkout: async (workoutId) => {
        set({ isLoading: true });
        try {
          const response = await axios.post('/api/workouts/sessions', {
            workoutId,
            startedAt: new Date().toISOString()
          });

          set({
            currentWorkout: response.data.workout,
            activeSession: response.data.session,
            isLoading: false,
            syncStatus: 'idle'
          });
        } catch (error) {
          set({
            error: error.response?.data?.message || error.message,
            isLoading: false,
            syncStatus: 'error'
          });
        }
      },

      // Update workout progress
      logExerciseSet: async (exerciseId, data) => {
        if (!get().activeSession) return;
        
        set({ isLoading: true });
        try {
          const response = await axios.patch(
            `/api/workouts/sessions/${get().activeSession.id}/exercises/${exerciseId}`,
            data
          );

          set({
            currentWorkout: response.data.workout,
            activeSession: response.data.session,
            lastSync: new Date().toISOString(),
            isLoading: false
          });
        } catch (error) {
          set({
            error: error.response?.data?.message || error.message,
            isLoading: false,
            syncStatus: 'error'
          });
        }
      },

      // Complete workout session
      completeWorkout: async () => {
        if (!get().activeSession) return;
        
        set({ isLoading: true });
        try {
          const response = await axios.patch(
            `/api/workouts/sessions/${get().activeSession.id}/complete`,
            { completedAt: new Date().toISOString() }
          );

          set({
            workouts: response.data.workouts,
            currentWorkout: null,
            activeSession: null,
            lastSync: new Date().toISOString(),
            isLoading: false
          });
        } catch (error) {
          set({
            error: error.response?.data?.message || error.message,
            isLoading: false,
            syncStatus: 'error'
          });
        }
      },

      // Sync with server
      syncWorkoutData: async () => {
        set({ syncStatus: 'syncing' });
        try {
          await get().fetchWorkouts();
          set({ syncStatus: 'idle' });
        } catch (error) {
          set({ 
            syncStatus: 'error',
            error: error.message
          });
        }
      },

      // Clear local data
      clearLocalData: () => {
        set({
          workouts: [],
          currentWorkout: null,
          activeSession: null,
          lastSync: null
        });
      }
    }),
    {
      name: 'workout-storage',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        workouts: state.workouts,
        lastSync: state.lastSync
      }),
      version: 1,
      migrate: (persistedState, version) => {
        if (version === 0) {
          // Migration from v0 to v1
          return {
            ...persistedState,
            syncStatus: 'idle'
          };
        }
        return persistedState;
      }
    }
  )
);

// Session synchronization handler
const syncWithServer = async () => {
  const { lastSync, syncWorkoutData } = useWorkoutStore.getState();
  
  // Sync if never synced or last sync > 5 minutes ago
  if (!lastSync || (Date.now() - new Date(lastSync).getTime()) > 300000) {
    await syncWorkoutData();
  }
};

// Initialize store
syncWithServer();

// Periodic sync every 5 minutes
setInterval(syncWithServer, 300000);

export default useWorkoutStore;
