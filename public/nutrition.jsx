import React, { useState, useEffect, useRef } from 'react';
import axios from 'axios';
import { FiActivity, FiTrendingUp, FiTrendingDown, FiPlus, FiTrash2, FiEdit, FiInfo, FiClock } from 'react-icons/fi';
import { CircularProgressbar, buildStyles } from 'react-circular-progressbar';
import 'react-circular-progressbar/dist/styles.css';
import './assets/nutrition.css';

// Enhanced Caching Middleware with TTL (Time To Live)
const createCacheMiddleware = (ttl = 3600000) => { // 1 hour default
  const cache = {};

  return {
    get: (key) => {
      const entry = cache[key];
      if (!entry) return null;
      
      // Check if cache entry is expired
      if (Date.now() > entry.expiry) {
        delete cache[key];
        return null;
      }
      return entry.value;
    },
    set: (key, value) => {
      cache[key] = {
        value,
        expiry: Date.now() + ttl
      };
      return value;
    },
    clear: () => {
      for (const key in cache) {
        delete cache[key];
      }
    },
    getStats: () => {
      return {
        size: Object.keys(cache).length,
        hits: Object.values(cache).filter(entry => entry.hit).length,
        expiresAt: Math.min(...Object.values(cache).map(entry => entry.expiry))
      };
    }
  };
};

const cache = createCacheMiddleware();

// Comprehensive South African Foods Database
const southAfricanFoods = [
  {
    id: 1,
    name: "Pap (Maize Meal)",
    category: "Staple",
    calories: 250,
    protein: 6,
    carbs: 56,
    fat: 1,
    fiber: 3,
    servingSize: "1 cup",
    image: "https://images.unsplash.com/photo-1601050690597-df0568f70950",
    description: "A traditional porridge made from maize meal, staple food in South Africa."
  },
  {
    id: 2,
    name: "Chakalaka",
    category: "Vegetable",
    calories: 120,
    protein: 3,
    carbs: 18,
    fat: 4,
    fiber: 5,
    servingSize: "1/2 cup",
    image: "https://images.unsplash.com/photo-1546069901-ba9599a7e63c",
    description: "Spicy vegetable relish made with onions, tomatoes, peppers, carrots, beans and spices."
  },
  // ... (more items with full details)
];

// Activity levels with detailed descriptions
const activityLevels = [
  {
    value: "sedentary",
    label: "Sedentary",
    description: "Little or no exercise, desk job"
  },
  {
    value: "light",
    label: "Lightly Active",
    description: "Light exercise 1-3 days/week"
  },
  {
    value: "moderate",
    label: "Moderately Active",
    description: "Moderate exercise 3-5 days/week"
  },
  {
    value: "active",
    label: "Very Active",
    description: "Hard exercise 6-7 days/week"
  },
  {
    value: "veryActive",
    label: "Extra Active",
    description: "Very hard exercise & physical job"
  }
];

// Goals for personalized recommendations
const goals = [
  { value: "weightLoss", label: "Weight Loss", icon: <FiTrendingDown /> },
  { value: "maintenance", label: "Maintenance", icon: <FiActivity /> },
  { value: "muscleGain", label: "Muscle Gain", icon: <FiTrendingUp /> }
];

const NutritionTracker = () => {
  // User Profile State
  const [profile, setProfile] = useState({
    weight: '',
    height: '',
    age: '',
    gender: 'male',
    activityLevel: 'moderate',
    goal: 'maintenance'
  });

  // Nutrition Data State
  const [nutritionData, setNutritionData] = useState({
    bmr: null,
    tdee: null,
    macros: { protein: 0, carbs: 0, fat: 0 },
    remaining: { protein: 0, carbs: 0, fat: 0, calories: 0 }
  });

  // Food Log State
  const [foodLog, setFoodLog] = useState([]);
  const [selectedFood, setSelectedFood] = useState('');
  const [portionSize, setPortionSize] = useState(1);
  const [mealType, setMealType] = useState('breakfast');
  const [logDate, setLogDate] = useState(new Date().toISOString().split('T')[0]);

  // UI State
  const [currentTab, setCurrentTab] = useState('dashboard');
  const [isCalculating, setIsCalculating] = useState(false);
  const [aiResponse, setAiResponse] = useState(null);
  const [isAiLoading, setIsAiLoading] = useState(false);
  const [foodDetails, setFoodDetails] = useState(null);
  const [currentImageIndex, setCurrentImageIndex] = useState(0);
  const galleryRef = useRef(null);

  // Calculate BMR with caching
  const calculateBMR = () => {
    const { weight, height, age, gender } = profile;
    const weightNum = parseFloat(weight);
    const heightNum = parseFloat(height);
    const ageNum = parseFloat(age);
    
    if (isNaN(weightNum)) return { error: "Please enter a valid weight" };
    if (isNaN(heightNum)) return { error: "Please enter a valid height" };
    if (isNaN(ageNum)) return { error: "Please enter a valid age" };

    const cacheKey = `bmr-${gender}-${weightNum}-${heightNum}-${ageNum}`;
    const cached = cache.get(cacheKey);
    
    if (cached) return cached;

    let bmr;
    if (gender === 'male') {
      bmr = 88.362 + (13.397 * weightNum) + (4.799 * heightNum) - (5.677 * ageNum);
    } else {
      bmr = 447.593 + (9.247 * weightNum) + (3.098 * heightNum) - (4.330 * ageNum);
    }

    return cache.set(cacheKey, { bmr: Math.round(bmr) });
  };

  // Calculate TDEE and macros based on goal
  const calculateNutritionNeeds = () => {
    const result = calculateBMR();
    if (result.error) return result;

    const { bmr } = result;
    const { activityLevel, goal } = profile;

    // Activity multipliers
    const activityFactors = {
      sedentary: 1.2,
      light: 1.375,
      moderate: 1.55,
      active: 1.725,
      veryActive: 1.9
    };

    // Goal adjustments
    const goalAdjustments = {
      weightLoss: 0.85,  // 15% deficit
      maintenance: 1.0,
      muscleGain: 1.15   // 15% surplus
    };

    // Calculate TDEE
    const tdee = Math.round(bmr * activityFactors[activityLevel] * goalAdjustments[goal]);

    // Macro ratios based on goal
    let macroRatios;
    switch(goal) {
      case 'weightLoss':
        macroRatios = { protein: 0.3, carbs: 0.4, fat: 0.3 }; // Higher protein for satiety
        break;
      case 'muscleGain':
        macroRatios = { protein: 0.35, carbs: 0.45, fat: 0.2 }; // Higher protein and carbs
        break;
      default:
        macroRatios = { protein: 0.25, carbs: 0.5, fat: 0.25 }; // Balanced
    }

    // Calculate macros in grams (1g protein/carbs = 4kcal, 1g fat = 9kcal)
    const protein = Math.round((tdee * macroRatios.protein) / 4);
    const carbs = Math.round((tdee * macroRatios.carbs) / 4);
    const fat = Math.round((tdee * macroRatios.fat) / 9);

    return {
      bmr,
      tdee,
      macros: { protein, carbs, fat },
      remaining: { protein, carbs, fat, calories: tdee }
    };
  };

  // Handle calculation
  const handleCalculate = () => {
    setIsCalculating(true);
    setTimeout(() => {
      const result = calculateNutritionNeeds();
      if (result.error) {
        alert(result.error);
      } else {
        setNutritionData(result);
      }
      setIsCalculating(false);
    }, 800); // Simulate processing delay
  };

  // Add food to log
  const handleAddFood = () => {
    if (!selectedFood || portionSize <= 0) return;
    
    const food = southAfricanFoods.find(f => f.name === selectedFood);
    if (!food) return;
    
    const loggedFood = {
      ...food,
      portionSize,
      mealType,
      date: logDate,
      totalCalories: Math.round(food.calories * portionSize),
      totalProtein: Math.round(food.protein * portionSize),
      totalCarbs: Math.round(food.carbs * portionSize),
      totalFat: Math.round(food.fat * portionSize)
    };
    
    setFoodLog([...foodLog, loggedFood]);
    updateRemainingMacros(loggedFood, 'add');
    setSelectedFood('');
    setPortionSize(1);
  };

  // Remove food from log
  const handleRemoveFood = (index) => {
    const foodToRemove = foodLog[index];
    const newFoodLog = [...foodLog];
    newFoodLog.splice(index, 1);
    setFoodLog(newFoodLog);
    updateRemainingMacros(foodToRemove, 'remove');
  };

  // Update remaining macros when adding/removing food
  const updateRemainingMacros = (food, action) => {
    if (!nutritionData.tdee) return;
    
    const multiplier = action === 'add' ? -1 : 1;
    setNutritionData(prev => ({
      ...prev,
      remaining: {
        calories: prev.remaining.calories + (food.totalCalories * multiplier),
        protein: prev.remaining.protein + (food.totalProtein * multiplier),
        carbs: prev.remaining.carbs + (food.totalCarbs * multiplier),
        fat: prev.remaining.fat + (food.totalFat * multiplier)
      }
    }));
  };

  // Calculate totals for the day
  const calculateDailyTotals = () => {
    const todayLog = foodLog.filter(item => item.date === logDate);
    
    return todayLog.reduce((totals, food) => ({
      calories: totals.calories + food.totalCalories,
      protein: totals.protein + food.totalProtein,
      carbs: totals.carbs + food.totalCarbs,
      fat: totals.fat + food.totalFat
    }), { calories: 0, protein: 0, carbs: 0, fat: 0 });
  };

  // Get AI nutrition advice
  const getAiAdvice = async () => {
    if (!nutritionData.tdee || foodLog.length === 0) {
      alert("Please calculate your nutrition needs and log some foods first");
      return;
    }

    setIsAiLoading(true);
    
    try {
      // In a real app, this would call your Roomie AI API
      // For demo, we'll simulate an API response
      const dailyTotals = calculateDailyTotals();
      const progress = {
        calories: Math.round((dailyTotals.calories / nutritionData.tdee) * 100),
        protein: Math.round((dailyTotals.protein / nutritionData.macros.protein) * 100),
        carbs: Math.round((dailyTotals.carbs / nutritionData.macros.carbs) * 100),
        fat: Math.round((dailyTotals.fat / nutritionData.macros.fat) * 100)
      };

      // Simulated AI response based on user data
      const response = {
        analysis: `Based on your ${profile.goal.replace(/([A-Z])/g, ' $1').toLowerCase()} goal, you've consumed ${dailyTotals.calories} of ${nutritionData.tdee} calories today (${progress.calories}%).`,
        recommendation: progress.calories > 100 ? 
          "Consider reducing portion sizes or choosing lower calorie options for your remaining meals." :
          "You're on track! Focus on nutrient-dense foods to meet your remaining needs.",
        mealSuggestions: getMealSuggestions(),
        nutrientFocus: getNutrientFocus(progress)
      };

      // Simulate API delay
      await new Promise(resolve => setTimeout(resolve, 1500));
      setAiResponse(response);
    } catch (error) {
      console.error("AI Error:", error);
      alert("Failed to get AI recommendations. Please try again.");
    } finally {
      setIsAiLoading(false);
    }
  };

  // Helper for AI meal suggestions
  const getMealSuggestions = () => {
    const { goal } = profile;
    const southAfricanOptions = southAfricanFoods.filter(food => {
      if (goal === 'weightLoss') return food.calories < 300 && food.fiber > 3;
      if (goal === 'muscleGain') return food.protein > 15;
      return true;
    }).slice(0, 5);

    return southAfricanOptions.map(food => ({
      name: food.name,
      reason: goal === 'weightLoss' ? 
        `High in fiber (${food.fiber}g) to keep you full` :
        `Rich in protein (${food.protein}g) for muscle support`
    }));
  };

  // Helper for AI nutrient focus
  const getNutrientFocus = (progress) => {
    const lowest = Math.min(progress.protein, progress.carbs, progress.fat);
    if (lowest === progress.protein) return "protein";
    if (lowest === progress.carbs) return "carbohydrates";
    return "healthy fats";
  };

  // Show food details modal
  const showFoodDetails = (food) => {
    setFoodDetails(food);
  };

  // Auto-scrolling food gallery with manual control
  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentImageIndex(prev => (prev + 1) % healthyFoodImages.length);
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  // Handle manual gallery navigation
  const navigateGallery = (direction) => {
    setCurrentImageIndex(prev => {
      if (direction === 'prev') {
        return prev === 0 ? healthyFoodImages.length - 1 : prev - 1;
      } else {
        return (prev + 1) % healthyFoodImages.length;
      }
    });
  };

  // Healthy food images for gallery
  const healthyFoodImages = [
    { url: "https://images.unsplash.com/photo-1490645935967-10de6ba17061", caption: "Balanced Breakfast" },
    { url: "https://images.unsplash.com/photo-1546069901-ba9599a7e63c", caption: "Colorful Vegetables" },
    // ... more images
  ];

  // Calculate progress percentages
  const dailyTotals = calculateDailyTotals();
  const progress = {
    calories: nutritionData.tdee ? Math.min(Math.round((dailyTotals.calories / nutritionData.tdee) * 100), 100) : 0,
    protein: nutritionData.macros.protein ? Math.min(Math.round((dailyTotals.protein / nutritionData.macros.protein) * 100), 100) : 0,
    carbs: nutritionData.macros.carbs ? Math.min(Math.round((dailyTotals.carbs / nutritionData.macros.carbs) * 100), 100) : 0,
    fat: nutritionData.macros.fat ? Math.min(Math.round((dailyTotals.fat / nutritionData.macros.fat) * 100), 100) : 0
  };

  return (
    <div className="nutrition-app">
      {/* Header with Navigation */}
      <header className="app-header">
        <h1>Roomie Nutrition Tracker</h1>
        <nav className="app-nav">
          <button 
            className={currentTab === 'dashboard' ? 'active' : ''}
            onClick={() => setCurrentTab('dashboard')}
          >
            Dashboard
          </button>
          <button 
            className={currentTab === 'diary' ? 'active' : ''}
            onClick={() => setCurrentTab('diary')}
          >
            Food Diary
          </button>
          <button 
            className={currentTab === 'analysis' ? 'active' : ''}
            onClick={() => setCurrentTab('analysis')}
          >
            AI Analysis
          </button>
        </nav>
      </header>

      {/* Interactive Food Gallery */}
      <section className="food-gallery" ref={galleryRef}>
        <button className="gallery-nav prev" onClick={() => navigateGallery('prev')}>
          &lt;
        </button>
        
        <div className="gallery-slides">
          {healthyFoodImages.map((img, index) => (
            <div 
              key={index}
              className={`gallery-slide ${index === currentImageIndex ? 'active' : ''}`}
              style={{ backgroundImage: `url(${img.url})` }}
            >
              <div className="slide-caption">
                <h3>{img.caption}</h3>
                <p>Healthy eating inspiration for your {profile.goal} journey</p>
              </div>
            </div>
          ))}
        </div>
        
        <button className="gallery-nav next" onClick={() => navigateGallery('next')}>
          &gt;
        </button>
        
        <div className="gallery-dots">
          {healthyFoodImages.map((_, index) => (
            <button
              key={index}
              className={index === currentImageIndex ? 'active' : ''}
              onClick={() => setCurrentImageIndex(index)}
            />
          ))}
        </div>
      </section>

      {/* Dashboard Tab */}
      {currentTab === 'dashboard' && (
        <div className="dashboard-tab">
          <section className="profile-section">
            <h2>Your Nutrition Profile</h2>
            <div className="profile-form">
              <div className="form-group">
                <label>Weight (kg)</label>
                <input
                  type="number"
                  value={profile.weight}
                  onChange={(e) => setProfile({...profile, weight: e.target.value})}
                  placeholder="Enter weight"
                />
              </div>
              
              <div className="form-group">
                <label>Height (cm)</label>
                <input
                  type="number"
                  value={profile.height}
                  onChange={(e) => setProfile({...profile, height: e.target.value})}
                  placeholder="Enter height"
                />
              </div>
              
              <div className="form-group">
                <label>Age</label>
                <input
                  type="number"
                  value={profile.age}
                  onChange={(e) => setProfile({...profile, age: e.target.value})}
                  placeholder="Enter age"
                />
              </div>
              
              <div className="form-group">
                <label>Gender</label>
                <select
                  value={profile.gender}
                  onChange={(e) => setProfile({...profile, gender: e.target.value})}
                >
                  <option value="male">Male</option>
                  <option value="female">Female</option>
                </select>
              </div>
              
              <div className="form-group">
                <label>Activity Level</label>
                <select
                  value={profile.activityLevel}
                  onChange={(e) => setProfile({...profile, activityLevel: e.target.value})}
                >
                  {activityLevels.map(level => (
                    <option key={level.value} value={level.value}>
                      {level.label} - {level.description}
                    </option>
                  ))}
                </select>
              </div>
              
              <div className="form-group">
                <label>Goal</label>
                <div className="goal-options">
                  {goals.map(g => (
                    <button
                      key={g.value}
                      className={profile.goal === g.value ? 'active' : ''}
                      onClick={() => setProfile({...profile, goal: g.value})}
                    >
                      {g.icon}
                      {g.label}
                    </button>
                  ))}
                </div>
              </div>
              
              <button 
                className="calculate-btn"
                onClick={handleCalculate}
                disabled={isCalculating}
              >
                {isCalculating ? 'Calculating...' : 'Calculate Nutrition Needs'}
              </button>
            </div>
          </section>

          {nutritionData.tdee && (
            <section className="results-section">
              <h2>Your Daily Nutrition Targets</h2>
              
              <div className="targets-grid">
                <div className="target-card calories">
                  <h3>Calories</h3>
                  <CircularProgressbar
                    value={progress.calories}
                    text={`${progress.calories}%`}
                    styles={buildStyles({
                      pathColor: progress.calories > 100 ? '#e74c3c' : '#2ecc71',
                      textColor: '#2c3e50',
                      trailColor: '#f0f0f0'
                    })}
                  />
                  <div className="target-details">
                    <p>Consumed: {dailyTotals.calories} kcal</p>
                    <p>Target: {nutritionData.tdee} kcal</p>
                    <p>Remaining: {Math.max(0, nutritionData.remaining.calories)} kcal</p>
                  </div>
                </div>
                
                <div className="target-card protein">
                  <h3>Protein</h3>
                  <CircularProgressbar
                    value={progress.protein}
                    text={`${progress.protein}%`}
                    styles={buildStyles({
                      pathColor: progress.protein > 100 ? '#e74c3c' : '#3498db',
                      textColor: '#2c3e50',
                      trailColor: '#f0f0f0'
                    })}
                  />
                  <div className="target-details">
                    <p>Consumed: {dailyTotals.protein}g</p>
                    <p>Target: {nutritionData.macros.protein}g</p>
                    <p>Remaining: {Math.max(0, nutritionData.remaining.protein)}g</p>
                  </div>
                </div>
                
                <div className="target-card carbs">
                  <h3>Carbs</h3>
                  <CircularProgressbar
                    value={progress.carbs}
                    text={`${progress.carbs}%`}
                    styles={buildStyles({
                      pathColor: progress.carbs > 100 ? '#e74c3c' : '#f39c12',
                      textColor: '#2c3e50',
                      trailColor: '#f0f0f0'
                    })}
                  />
                  <div className="target-details">
                    <p>Consumed: {dailyTotals.carbs}g</p>
                    <p>Target: {nutritionData.macros.carbs}g</p>
                    <p>Remaining: {Math.max(0, nutritionData.remaining.carbs)}g</p>
                  </div>
                </div>
                
                <div className="target-card fat">
                  <h3>Fat</h3>
                  <CircularProgressbar
                    value={progress.fat}
                    text={`${progress.fat}%`}
                    styles={buildStyles({
                      pathColor: progress.fat > 100 ? '#e74c3c' : '#9b59b6',
                      textColor: '#2c3e50',
                      trailColor: '#f0f0f0'
                    })}
                  />
                  <div className="target-details">
                    <p>Consumed: {dailyTotals.fat}g</p>
                    <p>Target: {nutritionData.macros.fat}g</p>
                    <p>Remaining: {Math.max(0, nutritionData.remaining.fat)}g</p>
                  </div>
                </div>
              </div>
            </section>
          )}
        </div>
      )}

      {/* Food Diary Tab */}
      {currentTab === 'diary' && (
        <div className="diary-tab">
          <div className="diary-header">
            <h2>Food Diary</h2>
            <div className="date-selector">
              <label>Date:</label>
              <input
                type="date"
                value={logDate}
                onChange={(e) => setLogDate(e.target.value)}
                max={new Date().toISOString().split('T')[0]}
              />
            </div>
          </div>
          
          <div className="add-food-form">
            <div className="form-group">
              <label>Meal Type</label>
              <select
                value={mealType}
                onChange={(e) => setMealType(e.target.value)}
              >
                <option value="breakfast">Breakfast</option>
                <option value="lunch">Lunch</option>
                <option value="dinner">Dinner</option>
                <option value="snack">Snack</option>
              </select>
            </div>
            
            <div className="form-group">
              <label>Food</label>
              <select
                value={selectedFood}
                onChange={(e) => setSelectedFood(e.target.value)}
              >
                <option value="">Select a food</option>
                {southAfricanFoods.map(food => (
                  <option key={food.id} value={food.name}>
                    {food.name} ({food.calories} kcal)
                  </option>
                ))}
              </select>
            </div>
            
            <div className="form-group">
              <label>Portion Size</label>
              <input
                type="number"
                value={portionSize}
                onChange={(e) => setPortionSize(e.target.value)}
                min="0.1"
                step="0.1"
              />
            </div>
            
            <button className="add-btn" onClick={handleAddFood}>
              <FiPlus /> Add Food
            </button>
          </div>
          
          <div className="food-log">
            {foodLog.filter(item => item.date === logDate).length === 0 ? (
              <div className="empty-log">
                <p>No foods logged for this day</p>
                <p>Start by adding foods above</p>
              </div>
            ) : (
              <div className="meal-sections">
                {['breakfast', 'lunch', 'dinner', 'snack'].map(meal => {
                  const mealItems = foodLog.filter(
                    item => item.date === logDate && item.mealType === meal
                  );
                  
                  if (mealItems.length === 0) return null;
                  
                  const mealTotal = mealItems.reduce((total, item) => ({
                    calories: total.calories + item.totalCalories,
                    protein: total.protein + item.totalProtein,
                    carbs: total.carbs + item.totalCarbs,
                    fat: total.fat + item.totalFat
                  }), { calories: 0, protein: 0, carbs: 0, fat: 0 });
                  
                  return (
                    <div key={meal} className="meal-section">
                      <h3 className="meal-title">
                        {meal.charAt(0).toUpperCase() + meal.slice(1)}
                        <span className="meal-total">
                          {mealTotal.calories} kcal | P: {mealTotal.protein}g | C: {mealTotal.carbs}g | F: {mealTotal.fat}g
                        </span>
                      </h3>
                      
                      <div className="food-items">
                        {mealItems.map((item, index) => (
                          <div key={index} className="food-item">
                            <div className="food-info">
                              <img 
                                src={item.image} 
                                alt={item.name}
                                className="food-image"
                                onError={(e) => {
                                  e.target.src = 'https://images.unsplash.com/photo-1546069901-ba9599a7e63c';
                                }}
                              />
                              <div className="food-details">
                                <h4>{item.name}</h4>
                                <p>{item.portionSize} serving(s)</p>
                                <div className="food-macros">
                                  <span>{item.totalCalories} kcal</span>
                                  <span>P: {item.totalProtein}g</span>
                                  <span>C: {item.totalCarbs}g</span>
                                  <span>F: {item.totalFat}g</span>
                                </div>
                              </div>
                            </div>
                            
                            <div className="food-actions">
                              <button 
                                className="info-btn"
                                onClick={() => showFoodDetails(item)}
                              >
                                <FiInfo />
                              </button>
                              <button 
                                className="remove-btn"
                                onClick={() => handleRemoveFood(
                                  foodLog.findIndex(logItem => 
                                    logItem.date === item.date && 
                                    logItem.mealType === item.mealType && 
                                    logItem.name === item.name
                                  )
                                )}
                              >
                                <FiTrash2 />
                              </button>
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        </div>
      )}

      {/* AI Analysis Tab */}
      {currentTab === 'analysis' && (
        <div className="analysis-tab">
          <h2>AI Nutrition Analysis</h2>
          
          <div className="ai-analysis-container">
            {!nutritionData.tdee ? (
              <div className="ai-prompt">
                <p>Calculate your nutrition needs first to get personalized AI recommendations</p>
                <button 
                  className="calculate-btn"
                  onClick={() => setCurrentTab('dashboard')}
                >
                  Go to Calculator
                </button>
              </div>
            ) : (
              <>
                <div className="ai-request">
                  <p>Get personalized nutrition advice based on your goals and food log</p>
                  <button 
                    className="ai-btn"
                    onClick={getAiAdvice}
                    disabled={isAiLoading}
                  >
                    {isAiLoading ? 'Analyzing...' : 'Ask Roomie AI'}
                  </button>
                </div>
                
                {aiResponse && (
                  <div className="ai-response">
                    <div className="ai-analysis">
                      <h3>Analysis</h3>
                      <p>{aiResponse.analysis}</p>
                    </div>
                    
                    <div className="ai-recommendation">
                      <h3>Recommendation</h3>
                      <p>{aiResponse.recommendation}</p>
                    </div>
                    
                    <div className="ai-meal-suggestions">
                      <h3>Meal Suggestions</h3>
                      <div className="suggestion-cards">
                        {aiResponse.mealSuggestions.map((suggestion, index) => {
                          const food = southAfricanFoods.find(f => f.name === suggestion.name);
                          return (
                            <div key={index} className="suggestion-card">
                              <img 
                                src={food.image} 
                                alt={food.name}
                                className="suggestion-image"
                              />
                              <h4>{food.name}</h4>
                              <p>{suggestion.reason}</p>
                              <button 
                                className="add-suggestion-btn"
                                onClick={() => {
                                  setSelectedFood(food.name);
                                  setPortionSize(1);
                                  setCurrentTab('diary');
                                }}
                              >
                                Add to Diary
                              </button>
                            </div>
                          );
                        })}
                      </div>
                    </div>
                    
                    <div className="ai-focus">
                      <h3>Nutrient Focus</h3>
                      <p>Based on your current intake, you should focus on getting more {aiResponse.nutrientFocus}.</p>
                    </div>
                  </div>
                )}
              </>
            )}
          </div>
        </div>
      )}

      {/* Food Details Modal */}
      {foodDetails && (
        <div className="food-modal">
          <div className="modal-content">
            <button 
              className="close-modal"
              onClick={() => setFoodDetails(null)}
            >
              &times;
            </button>
            
            <div className="modal-header">
              <img 
                src={foodDetails.image} 
                alt={foodDetails.name}
                className="modal-food-image"
              />
              <h2>{foodDetails.name}</h2>
              <p className="food-category">{foodDetails.category}</p>
            </div>
            
            <div className="modal-body">
              <p className="food-description">{foodDetails.description}</p>
              
              <div className="nutrition-facts">
                <h3>Nutrition Facts (per {foodDetails.servingSize})</h3>
                <div className="facts-grid">
                  <div className="fact-item">
                    <span className="fact-value">{foodDetails.calories}</span>
                    <span className="fact-label">Calories</span>
                  </div>
                  <div className="fact-item">
                    <span className="fact-value">{foodDetails.protein}g</span>
                    <span className="fact-label">Protein</span>
                  </div>
                  <div className="fact-item">
                    <span className="fact-value">{foodDetails.carbs}g</span>
                    <span className="fact-label">Carbohydrates</span>
                  </div>
                  <div className="fact-item">
                    <span className="fact-value">{foodDetails.fat}g</span>
                    <span className="fact-label">Fat</span>
                  </div>
                  <div className="fact-item">
                    <span className="fact-value">{foodDetails.fiber}g</span>
                    <span className="fact-label">Fiber</span>
                  </div>
                </div>
              </div>
              
              <div className="modal-actions">
                <button 
                  className="add-to-diary"
                  onClick={() => {
                    setSelectedFood(foodDetails.name);
                    setFoodDetails(null);
                    setCurrentTab('diary');
                  }}
                >
                  <FiPlus /> Add to Diary
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default NutritionTracker;