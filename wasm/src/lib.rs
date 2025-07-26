// wasm/src/lib.rs
mod pose_detection;
mod rep_counter;
mod neon_render;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use js_sys::{Array, JsString};
use web_sys::console;

#[derive(Debug, Serialize, Deserialize)]
pub struct Keypoint {
    x: f32,
    y: f32,
    confidence: f32,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct AnalysisResult {
    pub rep_count: u32,
    pub engagement: f32,
    pub form_errors: Vec<String>,
    pub overlay_data: Vec<u8>, // PNG image data for neon overlay
}

#[wasm_bindgen]
pub struct ExerciseAnalyzer {
    rep_counter: rep_counter::RepCounter,
    pose_analyzer: pose_detection::PoseAnalyzer,
    neon_renderer: neon_render::NeonRenderer,
    current_exercise: String,
    last_frame_time: f64,
}

#[wasm_bindgen]
impl ExerciseAnalyzer {
    #[wasm_bindgen(constructor)]
    pub fn new(exercise_id: &str) -> Self {
        // Initialize debug logging
        console::log_1(&JsValue::from_str(&format!("Initializing analyzer for {}", exercise_id)));

        ExerciseAnalyzer {
            rep_counter: rep_counter::RepCounter::new(),
            pose_analyzer: pose_detection::PoseAnalyzer::new(),
            neon_renderer: neon_render::NeonRenderer::new(0.0, 1.0, 1.0), // Cyan neon
            current_exercise: exercise_id.to_string(),
            last_frame_time: 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn process_frame(&mut self, keypoints: &[f32], timestamp: f64) -> Result<JsValue, JsValue> {
        // Calculate frame delta time
        let delta_time = if self.last_frame_time > 0.0 {
            (timestamp - self.last_frame_time) / 1000.0 // Convert to seconds
        } else {
            1.0 / 30.0 // Default to 30fps on first frame
        };
        self.last_frame_time = timestamp;

        // Convert flat array to keypoints
        let keypoints = match self.parse_keypoints(keypoints) {
            Ok(k) => k,
            Err(e) => return Err(JsValue::from_str(&e)),
        };

        // 1. Pose Analysis
        let angles = self.pose_analyzer.calculate_angles(&keypoints);
        
        // 2. Rep Counting
        let rep_detected = self.rep_counter.check_rep(&self.current_exercise, &angles, delta_time);
        if rep_detected {
            console::log_1(&JsValue::from_str("Rep detected!"));
        }
        
        // 3. Form Analysis
        let form_errors = self.pose_analyzer.check_form(&self.current_exercise, &angles);
        if !form_errors.is_empty() {
            console::warn_1(&Array::from_iter(
                form_errors.iter().map(|e| JsString::from(e.as_str()))
            );
        }
        
        // 4. Engagement Calculation
        let engagement = self.pose_analyzer.calculate_engagement(&self.current_exercise, &angles, delta_time);
        self.neon_renderer.set_intensity(engagement);

        // 5. Neon Rendering
        let overlay_data = self.neon_renderer.render(&keypoints);

        let result = AnalysisResult {
            rep_count: self.rep_counter.count(),
            engagement,
            form_errors,
            overlay_data,
        };

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    }

    fn parse_keypoints(&self, flat_array: &[f32]) -> Result<Vec<Keypoint>, String> {
        if flat_array.len() % 3 != 0 {
            return Err(format!(
                "Invalid keypoints array length. Expected multiple of 3, got {}", 
                flat_array.len()
            ));
        }

        let mut keypoints = Vec::new();
        for chunk in flat_array.chunks_exact(3) {
            keypoints.push(Keypoint {
                x: chunk[0],
                y: chunk[1],
                confidence: chunk[2],
            });
        }

        Ok(keypoints)
    }
}

// Helper functions exposed to JS
#[wasm_bindgen]
pub fn get_supported_exercises() -> JsValue {
    let exercises = vec![
        // Quadriceps
        "squat", "bulgarian-splits", "bodyweight-lunges", "goblet-squats", 
        "weighted-lunges", "side-squats", "obstacle-overstep-touches",
        "stand-ups", "squat-steps", "front-squats", "leg-extensions",
        
        // Abs/Core
        "jack-knife", "back_arch", "lateral-leg-raises", "lateral-sit-ups",
        "dumbbell-leg-raises", "russian-roulette", "hanging-leg-raises",
        "ab-wheel-rollout", "reverse-crunch", "scissor-kicks", "plank-hip-dips",
        "situp", "plank", "russian-twist", "bicycle-crunches", "toe-touches",
        
        // Back
        "dumbbell-rows", "barbell-rows", "seated-dumbbell-rows", "chin-up-pull-ups",
        "open-butterfly", "lateral-russian-roulette", "deadlifts", "pull-ups",
        "face-pulls", "t-bar-rows", "lat-pulldown", "reverse-fly", "hyperextensions",
        
        // Biceps
        "isolated-dumbbell-curls", "barbell-curls", "dumbbell-curls",
        "open-grip-pull-ups", "lateral-push-ups", "half-rep-curls",
        "resistance-bands-pull", "outward-dumbbell-curls", "concentration-curls",
        "zottman-curls", "hammer-curls", "preacher-curls",
        
        // Calves
        "bench-calf-raises", "plate-raises", "bulgarian-raises", "barbell-raises",
        "jump-rope", "donkey-calf-raises", "seated-calf-raises", "stair-calf-raises",
        "farmer-walk-on-toes", "pogo-jumps", "standing-calf-raises", 
        
        // Chest
        "pushup", "inner-push-ups", "superman-push-ups", "butterfly",
        "dumbbell-overhead", "military-press", "bench-press-dumbbell",
        "bench-press-barbell", "bench-butterfly", "dumbbell-fly", "chest-dips",
        "incline-bench-press", "decline-bench-press",
        
        // Glutes
        "superman", "good-morning", "yoga-ball-glute-raises", "donkey-kick",
        "inverse-kick-back", "barbell-bench-touches", "barbell-hip-thrust",
        "curtsy-lunges", "cable-pull-through", "frog-pumps", "glute-bridge",
        "single-leg-hip-thrust", "clamshells",
        
        // Hamstrings
        "romanian-deadlifts", "hamstring-curls", "kettlebell-good-morning",
        "glute-ham-raises", "single-leg-deadlifts", "seated-leg-curls",
        "nordic-hamstring-curls", "stiff-leg-deadlifts", "swiss-ball-hamstring-curls",
        "reverse-hyperextensions", "sliding-leg-curls",
        
        // Shoulders
        "lateral-dumbbell-raises", "frontal-dumbbell-raises", "dumbbell-shrugs",
        "bench-dumbbell-raises", "exterior-dumbbell-raises", "arnold-press",
        "military-press", "handstand-pushups", "overhead-press", "rear-delt-fly",
        "face-pulls", "upright-rows",
        
        // Triceps
        "closed-grip-barbell", "lateral-barbell-extensions", "diamond-push-ups",
        "dumbbell-dips", "bench-dips", "lateral-dumbbell-raises",
        "hammer-dumbbell-raises", "forearm-push-ups", "barbell-overhead",
        "triceps-rope-pushdown", "skull-crushers", "close-grip-bench-press",
        
        // Full Body/Compound
        "burpees", "kettlebell-swings", "thrusters", "clean-and-jerk",
        "snatch", "box-jumps", "jumping-jacks", "mountain-climbers"
    ];

    serde_wasm_bindgen::to_value(&exercises).unwrap()
}