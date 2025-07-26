// wasm/src/rep_counter.rs
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovementPhase {
    Concentric,
    Eccentric,
    StaticHold,
    None,
}

#[derive(Debug)]
pub struct RepCounter {
    count: u32,
    current_phase: MovementPhase,
    last_angles: HashMap<String, f32>,
    last_timestamp: f32,
    exercise_profile: ExerciseProfile,
    velocity_window: Vec<f32>,
    rom_window: Vec<f32>,
}

#[derive(Debug)]
struct ExerciseProfile {
    primary_joint: String,
    secondary_joints: Vec<String>,
    range_min: f32,
    range_max: f32,
    velocity_threshold: f32,
    min_rom_percentage: f32,
    lockout_angle: Option<f32>,
    stretch_angle: Option<f32>,
    movement_pattern: MovementPattern,
}

#[derive(Debug)]
enum MovementPattern {
    Crunch,
    LegRaise,
    Rotation,
    Static,
    HipFlexion,
    SpinalFlexion,
    AntiRotation,
    CompoundCore,
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Triceps Exercises
            "closed-grip-barbell" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::CloseGripPress,
            },
            "lateral-barbell-extensions" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::SkullCrusher,
            },
            "diamond-push-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "wrist".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::TricepsPushup,
            },
            "dumbbell-dips" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.55,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::Dip,
            },
            "bench-dips" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::BenchDip,
            },
            "lateral-dumbbell-raises" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::Kickback,
            },
            "hammer-dumbbell-raises" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::OverheadExtension,
            },
            "forearm-push-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::TricepsPushup,
            },
            "barbell-overhead" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::OverheadPress,
            },
            "tricep-rope-pushdown" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["wrist".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::Pushdown,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::CloseGripPress | MovementPattern::Dip => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::SkullCrusher | MovementPattern::OverheadExtension => {
                if velocity > self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Kickback => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Pushdown => {
                if velocity > self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::TricepsPushup => {
                if velocity > self.exercise_profile.velocity_threshold * 1.2 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::CloseGripPress | MovementPattern::Dip => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::SkullCrusher | MovementPattern::OverheadExtension => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::Kickback => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::Pushdown => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.85 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::TricepsPushup => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.75 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Shoulders Exercises
            "lateral-dumbbell-raises" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::LateralRaise,
            },
            "frontal-dumbbell-raises" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::FrontRaise,
            },
            "dumbbell-shrugs" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 45.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.8,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Shrug,
            },
            "bench-dumbbell-raises" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.75,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::RearDeltRaise,
            },
            "exterior-dumbbell-raises" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::ExternalRotation,
            },
            "arnold-press" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: None,
                movement_pattern: MovementPattern::RotationalPress,
            },
            "military-press" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: None,
                movement_pattern: MovementPattern::OverheadPress,
            },
            "handstand-pushups" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: None,
                movement_pattern: MovementPattern::VerticalPress,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::LateralRaise | MovementPattern::FrontRaise => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::OverheadPress | MovementPattern::VerticalPress => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::RotationalPress => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Shrug => {
                if velocity > self.exercise_profile.velocity_threshold * 1.2 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::RearDeltRaise | MovementPattern::ExternalRotation => {
                if velocity > self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::OverheadPress | MovementPattern::VerticalPress => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::RotationalPress => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::LateralRaise | MovementPattern::FrontRaise => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.7 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::Shrug => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.6 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::RearDeltRaise | MovementPattern::ExternalRotation => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.75 {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Quadriceps Exercises
            "bulgarian-splits" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string(), "ankle".to_string()],
                range_min: 80.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(80.0),
                movement_pattern: MovementPattern::SplitSquat,
            },
            "bodyweight-lunges" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.75,
                lockout_angle: Some(175.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::Lunge,
            },
            "goblet-squats" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string(), "ankle".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::Squat,
            },
            "weighted-lunges" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.85,
                lockout_angle: Some(175.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::WeightedLunge,
            },
            "side-squats" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string(), "ankle".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::LateralSquat,
            },
            "obstacle-overstep-touches" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string(), "ankle".to_string()],
                range_min: 45.0,
                range_max: 180.0,
                velocity_threshold: 0.55,
                min_rom_percentage: 0.65,
                lockout_angle: Some(170.0),
                stretch_angle: Some(45.0),
                movement_pattern: MovementPattern::StepUp,
            },
            "stand-ups" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::KneeExtension,
            },
            "squat-steps" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string(), "ankle".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::StepUp,
            },
            "front-squats" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string(), "ankle".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.55,
                min_rom_percentage: 0.95,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::OlympicSquat,
            },
            "leg-extensions" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["ankle".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.9,
                lockout_angle: Some(175.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::IsolationExtension,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::Squat | MovementPattern::OlympicSquat => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Lunge | MovementPattern::WeightedLunge => {
                if velocity > self.exercise_profile.velocity_threshold * 1.2 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::IsolationExtension => {
                if velocity > self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::StepUp => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::KneeExtension => {
                if velocity > self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::Squat | MovementPattern::OlympicSquat => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::Lunge | MovementPattern::WeightedLunge => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::IsolationExtension => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.95 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::StepUp => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::KneeExtension => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Hamstrings Exercises
            "romanian-deadlifts" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string(), "spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipHinge,
            },
            "hamstring-curls" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::LegCurl,
            },
            "kettlebell-good-morning" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string(), "spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipHinge,
            },
            "glute-ham-raises" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string(), "spine".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::GluteHamRaise,
            },
            "single-leg-deadlifts" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string(), "spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::SingleLegHinge,
            },
            "seated-leg-curls" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::SeatedCurl,
            },
            "nordic-hamstring-curls" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::EccentricCurl,
            },
            "stiff-leg-deadlifts" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string(), "spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.55,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::StiffLegHinge,
            },
            "swiss-ball-hamstring-curls" => ExerciseProfile {
                primary_joint: "knee".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.75,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::BallCurl,
            },
            "reverse-hyperextensions" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 45.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::ReverseHyper,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::HipHinge | MovementPattern::StiffLegHinge => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::LegCurl | MovementPattern::SeatedCurl => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::GluteHamRaise => {
                if velocity > self.exercise_profile.velocity_threshold * 1.2 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::EccentricCurl => {
                if velocity < -self.exercise_profile.velocity_threshold * 0.5 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::BallCurl => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::ReverseHyper => {
                if velocity > self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::HipHinge | MovementPattern::StiffLegHinge => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::LegCurl | MovementPattern::SeatedCurl => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::GluteHamRaise => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::EccentricCurl => {
                if *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.7 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::BallCurl => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.75 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::ReverseHyper => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Glutes Exercises
            "superman" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 30.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.6,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::IsometricHold,
            },
            "good-morning" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string(), "knee".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipHinge,
            },
            "yoga-ball-glute-raises" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 0.0,
                range_max: 45.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.8,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::GluteBridge,
            },
            "donkey-kick" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::Kickback,
            },
            "inverse-kick-back" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 0.0,
                range_max: 60.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.75,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::ReverseKickback,
            },
            "barbell-bench-touches" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipThrust,
            },
            "barbell-hip-thrust" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipThrust,
            },
            "curtsy-lunges" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string(), "ankle".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::LateralLunge,
            },
            "cable-pull-through" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipHinge,
            },
            "frog-pumps" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 0.0,
                range_max: 45.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::GluteBridge,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::HipThrust | MovementPattern::GluteBridge => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::HipHinge => {
                if velocity > self.exercise_profile.velocity_threshold * 1.2 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Kickback | MovementPattern::ReverseKickback => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::LateralLunge => {
                if velocity > self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::IsometricHold => {
                if rom > 0.5 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::HipThrust => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::GluteBridge => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::HipHinge => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::Kickback => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::LateralLunge => {
                if let Some(stretch) = self.exercise_profile.stretch_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle <= stretch && 
                           *new_phase == MovementPhase::Concentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::IsometricHold => {
                if *new_phase == MovementPhase::StaticHold &&
                   rom >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Chest Exercises
            "inner-push-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "wrist".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::CloseGripPress,
            },
            "superman-push-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "spine".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::ExplosivePress,
            },
            "butterfly" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.8,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::Fly,
            },
            "dumbbell-overhead" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::VerticalPress,
            },
            "military-press" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::StrictPress,
            },
            "bench-press-dumbbell" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::DumbbellPress,
            },
            "bench-press-barbell" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.55,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::BarbellPress,
            },
            "bench-butterfly" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.75,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::MachineFly,
            },
            "dumbbell-rows" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::ChestSupportedRow,
            },
            "open-butterfly" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 120.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::CableFly,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::BarbellPress | MovementPattern::DumbbellPress => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Fly | MovementPattern::CableFly => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::ExplosivePress => {
                if velocity > self.exercise_profile.velocity_threshold * 1.5 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::VerticalPress => {
                if velocity > self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::BarbellPress | MovementPattern::DumbbellPress => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::Fly | MovementPattern::CableFly => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::ExplosivePress => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.7 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::VerticalPress => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Calves Exercises
            "bench-calf-raises" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::StraightLegRaise,
            },
            "plate-raises" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::WeightedRaise,
            },
            "bulgarian-raises" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string(), "hip".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::SingleLegRaise,
            },
            "barbell-raises" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.55,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::WeightedRaise,
            },
            "jump-rope" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.8,
                min_rom_percentage: 0.5,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Plyometric,
            },
            "donkey-calf-raises" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string(), "hip".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::BentLegRaise,
            },
            "seated-calf-raises" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::SeatedRaise,
            },
            "stair-calf-raises" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::PartialRaise,
            },
            "farmer-walk-on-toes" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 135.0,
                range_max: 180.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: None,
                movement_pattern: MovementPattern::Isometric,
            },
            "pogo-jumps" => ExerciseProfile {
                primary_joint: "ankle".to_string(),
                secondary_joints: vec!["knee".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 1.0,
                min_rom_percentage: 0.4,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Plyometric,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::StraightLegRaise | MovementPattern::WeightedRaise => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Plyometric => {
                if velocity > self.exercise_profile.velocity_threshold * 1.5 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.5 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::SeatedRaise => {
                if velocity > self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Isometric => {
                if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::StraightLegRaise | MovementPattern::WeightedRaise => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::Plyometric => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::SeatedRaise => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::Isometric => {
                if *new_phase == MovementPhase::StaticHold &&
                   rom >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Biceps Exercises
            "isolated-dumbbell-curls" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "wrist".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::IsolationCurl,
            },
            "barbell-curls" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::BarbellCurl,
            },
            "dumbbell-curls" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "wrist".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::DumbbellCurl,
            },
            "open-grip-pull-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::ChinUp,
            },
            "lateral-push-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "wrist".to_string()],
                range_min: 60.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(60.0),
                movement_pattern: MovementPattern::Pushup,
            },
            "half-rep-curls" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 45.0,
                range_max: 90.0,
                velocity_threshold: 0.25,
                min_rom_percentage: 0.9,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::PartialCurl,
            },
            "resistance-bands-pull" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::BandCurl,
            },
            "outward-dumbbell-curls" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "wrist".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::ReverseCurl,
            },
            "concentration-curls" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.25,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::IsolationCurl,
            },
            "zottman-curls" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["wrist".to_string(), "shoulder".to_string()],
                range_min: 0.0,
                range_max: 135.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::ZottmanCurl,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all existing methods)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::BarbellCurl | MovementPattern::DumbbellCurl => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::ChinUp => {
                if velocity > self.exercise_profile.velocity_threshold * 1.2 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::ZottmanCurl => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::PartialCurl => {
                if velocity > self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.7 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::BarbellCurl | MovementPattern::DumbbellCurl => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::ChinUp => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::ZottmanCurl => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::PartialCurl => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.6 {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Back Exercises
            "dumbbell-rows" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "spine".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.7,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::VerticalPull,
            },
            "barbell-rows" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "spine".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::HorizontalPull,
            },
            "seated-dumbbell-rows" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::HorizontalPull,
            },
            "chin-up-pull-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.85,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::VerticalPull,
            },
            "open-butterfly" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string(), "spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::Isolation,
            },
            "lateral-russian-roulette" => ExerciseProfile {
                primary_joint: "spine".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: -30.0,
                range_max: 30.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.6,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Rotation,
            },
            "deadlifts" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["knee".to_string(), "spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.7,
                min_rom_percentage: 0.9,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipHinge,
            },
            "pull-ups" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string()],
                range_min: 0.0,
                range_max: 180.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: Some(170.0),
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::VerticalPull,
            },
            "face-pulls" => ExerciseProfile {
                primary_joint: "shoulder".to_string(),
                secondary_joints: vec!["elbow".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::RearDeltPull,
            },
            "t-bar-rows" => ExerciseProfile {
                primary_joint: "elbow".to_string(),
                secondary_joints: vec!["shoulder".to_string(), "spine".to_string()],
                range_min: 90.0,
                range_max: 180.0,
                velocity_threshold: 0.45,
                min_rom_percentage: 0.75,
                lockout_angle: Some(170.0),
                stretch_angle: Some(90.0),
                movement_pattern: MovementPattern::HorizontalPull,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Isolation,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    // ... (keep all the existing methods from previous implementation)

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::VerticalPull | MovementPattern::HorizontalPull => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom > 0.9 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::HipHinge => {
                if velocity > self.exercise_profile.velocity_threshold * 1.2 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 0.8 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::RearDeltPull => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Rotation => {
                if velocity.abs() > self.exercise_profile.velocity_threshold * 0.7 {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::VerticalPull | MovementPattern::HorizontalPull => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::HipHinge => {
                if let Some(lockout) = self.exercise_profile.lockout_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle >= lockout && 
                           *new_phase == MovementPhase::Eccentric &&
                           rom >= self.exercise_profile.min_rom_percentage * 0.9 {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::RearDeltPull => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::Rotation => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom.abs() >= self.exercise_profile.min_rom_percentage * 0.6 {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }
}

impl RepCounter {
    pub fn new(exercise_id: &str) -> Self {
        let exercise_profile = match exercise_id {
            // Abs Exercises
            "jack-knife" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipFlexion,
            },
            "back_arch" => ExerciseProfile {
                primary_joint: "spine".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 0.0,
                range_max: 45.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.6,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::SpinalFlexion,
            },
            "lateral-leg-raises" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 75.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.65,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::LegRaise,
            },
            "lateral-sit-ups" => ExerciseProfile {
                primary_joint: "spine".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: 0.0,
                range_max: 60.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::Rotation,
            },
            "dumbbell-leg-raises" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.3,
                min_rom_percentage: 0.75,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::LegRaise,
            },
            "russian-roulette" => ExerciseProfile {
                primary_joint: "spine".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: -45.0,
                range_max: 45.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.8,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Rotation,
            },
            "hanging-leg-raises" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.8,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::LegRaise,
            },
            "ab-wheel-rollout" => ExerciseProfile {
                primary_joint: "spine".to_string(),
                secondary_joints: vec!["hip".to_string(), "shoulder".to_string()],
                range_min: 0.0,
                range_max: 60.0,
                velocity_threshold: 0.5,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::CompoundCore,
            },
            "reverse-crunch" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 90.0,
                velocity_threshold: 0.35,
                min_rom_percentage: 0.7,
                lockout_angle: None,
                stretch_angle: Some(0.0),
                movement_pattern: MovementPattern::HipFlexion,
            },
            "scissor-kicks" => ExerciseProfile {
                primary_joint: "hip".to_string(),
                secondary_joints: vec!["spine".to_string()],
                range_min: 0.0,
                range_max: 45.0,
                velocity_threshold: 0.6,
                min_rom_percentage: 0.5,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::LegRaise,
            },
            "plank-hip-dips" => ExerciseProfile {
                primary_joint: "spine".to_string(),
                secondary_joints: vec!["hip".to_string()],
                range_min: -30.0,
                range_max: 30.0,
                velocity_threshold: 0.4,
                min_rom_percentage: 0.6,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::AntiRotation,
            },

            _ => ExerciseProfile {
                primary_joint: "".to_string(),
                secondary_joints: vec![],
                range_min: 0.0,
                range_max: 0.0,
                velocity_threshold: 0.0,
                min_rom_percentage: 0.0,
                lockout_angle: None,
                stretch_angle: None,
                movement_pattern: MovementPattern::Static,
            },
        };

        RepCounter {
            count: 0,
            current_phase: MovementPhase::None,
            last_angles: HashMap::new(),
            last_timestamp: 0.0,
            exercise_profile,
            velocity_window: Vec::with_capacity(5),
            rom_window: Vec::with_capacity(3),
        }
    }

    pub fn update(&mut self, angles: &HashMap<String, f32>, timestamp: f32) -> Option<(u32, MovementPhase)> {
        if self.exercise_profile.primary_joint.is_empty() {
            return None;
        }

        // Calculate current ROM percentage
        let rom_percentage = self.calculate_rom_percentage(angles);
        self.rom_window.push(rom_percentage);
        if self.rom_window.len() > 3 {
            self.rom_window.remove(0);
        }
        let avg_rom = self.rom_window.iter().sum::<f32>() / self.rom_window.len() as f32;

        // Calculate movement velocity
        let velocity = self.calculate_velocity(angles, timestamp);
        self.velocity_window.push(velocity);
        if self.velocity_window.len() > 5 {
            self.velocity_window.remove(0);
        }
        let avg_velocity = self.velocity_window.iter().sum::<f32>() / self.velocity_window.len() as f32;

        // Detect phase changes based on movement pattern
        let new_phase = self.detect_phase(avg_velocity, avg_rom);

        // Check for completed rep with pattern-specific logic
        let rep_detected = self.check_rep_completion(&new_phase, avg_rom);

        self.current_phase = new_phase;
        self.last_angles = angles.clone();
        self.last_timestamp = timestamp;

        if rep_detected {
            Some((self.count, self.current_phase))
        } else {
            None
        }
    }

    fn detect_phase(&self, velocity: f32, rom: f32) -> MovementPhase {
        match self.exercise_profile.movement_pattern {
            MovementPattern::Crunch | MovementPattern::SpinalFlexion => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::LegRaise | MovementPattern::HipFlexion => {
                if velocity > self.exercise_profile.velocity_threshold {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold {
                    MovementPhase::Eccentric
                } else if rom < 0.1 {
                    MovementPhase::StaticHold
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::Rotation | MovementPattern::AntiRotation => {
                if velocity.abs() > self.exercise_profile.velocity_threshold {
                    if velocity > 0.0 { MovementPhase::Concentric }
                    else { MovementPhase::Eccentric }
                } else {
                    MovementPhase::None
                }
            },
            MovementPattern::CompoundCore => {
                if velocity > self.exercise_profile.velocity_threshold * 1.5 {
                    MovementPhase::Concentric
                } else if velocity < -self.exercise_profile.velocity_threshold * 1.5 {
                    MovementPhase::Eccentric
                } else {
                    MovementPhase::None
                }
            },
            _ => MovementPhase::None,
        }
    }

    fn check_rep_completion(&mut self, new_phase: &MovementPhase, rom: f32) -> bool {
        let mut completed = false;
        
        match self.exercise_profile.movement_pattern {
            MovementPattern::Crunch | MovementPattern::SpinalFlexion => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::LegRaise | MovementPattern::HipFlexion => {
                if let Some(stretch) = self.exercise_profile.stretch_angle {
                    if let Some(angle) = self.last_angles.get(&self.exercise_profile.primary_joint) {
                        if *angle <= stretch && 
                           *new_phase == MovementPhase::Concentric &&
                           rom >= self.exercise_profile.min_rom_percentage {
                            self.count += 1;
                            completed = true;
                        }
                    }
                }
            },
            MovementPattern::Rotation => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom.abs() >= self.exercise_profile.min_rom_percentage {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::AntiRotation => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom.abs() >= self.exercise_profile.min_rom_percentage * 0.5 {
                    self.count += 1;
                    completed = true;
                }
            },
            MovementPattern::CompoundCore => {
                if self.current_phase == MovementPhase::Concentric && 
                   *new_phase == MovementPhase::Eccentric &&
                   rom >= self.exercise_profile.min_rom_percentage * 0.8 {
                    self.count += 1;
                    completed = true;
                }
            },
            _ => {}
        }
        
        completed
    }

    fn calculate_rom_percentage(&self, angles: &HashMap<String, f32>) -> f32 {
        if let Some(angle) = angles.get(&self.exercise_profile.primary_joint) {
            let normalized = (angle - self.exercise_profile.range_min) / 
                         (self.exercise_profile.range_max - self.exercise_profile.range_min);
            normalized.max(0.0).min(1.0)
        } else {
            0.0
        }
    }

    fn calculate_velocity(&self, angles: &HashMap<String, f32>, timestamp: f32) -> f32 {
        if let (Some(current_angle), Some(last_angle)) = (
            angles.get(&self.exercise_profile.primary_joint),
            self.last_angles.get(&self.exercise_profile.primary_joint)
        ) {
            let dt = timestamp - self.last_timestamp;
            if dt > 0.0 {
                (current_angle - last_angle) / dt
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn current_phase(&self) -> MovementPhase {
        self.current_phase
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.current_phase = MovementPhase::None;
        self.last_angles.clear();
        self.velocity_window.clear();
        self.rom_window.clear();
    }
}
