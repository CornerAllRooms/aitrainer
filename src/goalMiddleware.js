import { getCache, setCache } from '@/libs/cache';

export const useGoalMiddleware = () => {
  const CACHE_KEY = 'user_goals_v1';

  const getCachedGoals = async () => {
    try {
      return await getCache(CACHE_KEY);
    } catch (error) {
      console.error('Cache read error:', error);
      return null;
    }
  };

  const cacheGoals = async (goals) => {
    try {
      await setCache(CACHE_KEY, goals, 86400); // 24h cache
    } catch (error) {
      console.error('Cache write error:', error);
    }
  };

  const validateGoals = async (goals) => {
    // Add server validation if needed
    if (!goals.primaryGoal) {
      return { valid: false, message: 'Please select a primary goal' };
    }
    return { valid: true };
  };

  return { getCachedGoals, cacheGoals, validateGoals };
};