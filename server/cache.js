const redis = require('redis');
const memoryCache = require('memory-cache');
const { promisify } = require('util');

const isProduction = process.env.NODE_ENV === 'production';

// Redis client setup
let redisClient;
if (isProduction) {
  redisClient = redis.createClient({
    url: process.env.REDIS_URL,
    socket: {
      tls: true,
      rejectUnauthorized: false
    }
  });
} else {
  redisClient = redis.createClient();
}

const redisGet = promisify(redisClient.get).bind(redisClient);
const redisSet = promisify(redisClient.set).bind(redisClient);

// Cache middleware
module.exports = {
  getCache: async (key) => {
    if (isProduction) {
      try {
        const data = await redisGet(key);
        return data ? JSON.parse(data) : null;
      } catch (err) {
        console.error('Redis error:', err);
        return null;
      }
    }
    // Fallback to memory cache in development
    return memoryCache.get(key);
  },

  setCache: async (key, value, ttl = 86400) => {
    const stringValue = JSON.stringify(value);
    if (isProduction) {
      try {
        await redisSet(key, stringValue, 'EX', ttl);
      } catch (err) {
        console.error('Redis error:', err);
      }
    } else {
      memoryCache.put(key, value, ttl * 1000);
    }
  },

  clearCache: async (key) => {
    if (isProduction) {
      try {
        await redisClient.del(key);
      } catch (err) {
        console.error('Redis error:', err);
      }
    } else {
      memoryCache.del(key);
    }
  }
};