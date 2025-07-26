
// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    triceps_exercises: HashMap<&'static str, TricepsExerciseProfile>,
}

#[derive(Debug, Clone)]
struct TricepsExerciseProfile {
    elbow_extension_range: (f32, f32),   // Optimal extension angles
    shoulder_stabilization: f32,         // 0-1 how much shoulder should stay fixed
    humeral_position: HumeralPosition,   // Upper arm orientation
    lockout_requirement: f32,            // 0-1 how strict full extension is
    compound_factor: f32,                // How much other muscles are involved
}

#[derive(Debug, Clone)]
enum HumeralPosition {
    Overhead,
    Neutral,
    Flexed(f32),  // Specific flexion angle
    Extended(f32), // Specific extension angle
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut triceps_exercises = HashMap::new();

        // 1. Closed-Grip Barbell
        triceps_exercises.insert("closed-grip-barbell", TricepsExerciseProfile {
            elbow_extension_range: (30.0, 180.0),
            shoulder_stabilization: 0.8,
            humeral_position: HumeralPosition::Flexed(45.0),
            lockout_requirement: 0.9,
            compound_factor: 0.7,
        });

        // 2. Lateral Barbell Extensions
        triceps_exercises.insert("lateral-barbell-extensions", TricepsExerciseProfile {
            elbow_extension_range: (90.0, 180.0),
            shoulder_stabilization: 0.9,
            humeral_position: HumeralPosition::Flexed(90.0),
            lockout_requirement: 0.95,
            compound_factor: 0.3,
        });

        // 3. Diamond Push-ups
        triceps_exercises.insert("diamond-push-ups", TricepsExerciseProfile {
            elbow_extension_range: (60.0, 180.0),
            shoulder_stabilization: 0.6,
            humeral_position: HumeralPosition::Neutral,
            lockout_requirement: 0.8,
            compound_factor: 0.5,
        });

        // 4. Dumbbell Dips
        triceps_exercises.insert("dumbbell-dips", TricepsExerciseProfile {
            elbow_extension_range: (90.0, 180.0),
            shoulder_stabilization: 0.7,
            humeral_position: HumeralPosition::Extended(20.0),
            lockout_requirement: 0.85,
            compound_factor: 0.6,
        });

        // 5. Bench Dips
        triceps_exercises.insert("bench-dips", TricepsExerciseProfile {
            elbow_extension_range: (90.0, 180.0),
            shoulder_stabilization: 0.5,
            humeral_position: HumeralPosition::Extended(45.0),
            lockout_requirement: 0.75,
            compound_factor: 0.4,
        });

        // 6. Lateral Dumbbell Raises (Triceps Variation)
        triceps_exercises.insert("lateral-dumbbell-raises", TricepsExerciseProfile {
            elbow_extension_range: (90.0, 180.0),
            shoulder_stabilization: 0.8,
            humeral_position: HumeralPosition::Flexed(90.0),
            lockout_requirement: 0.9,
            compound_factor: 0.3,
        });

        // 7. Hammer Dumbbell Raises
        triceps_exercises.insert("hammer-dumbbell-raises", TricepsExerciseProfile {
            elbow_extension_range: (0.0, 180.0), // Full ROM
            shoulder_stabilization: 0.85,
            humeral_position: HumeralPosition::Overhead,
            lockout_requirement: 0.95,
            compound_factor: 0.2,
        });

        // 8. Forearm Push-ups
        triceps_exercises.insert("forearm-push-ups", TricepsExerciseProfile {
            elbow_extension_range: (60.0, 180.0),
            shoulder_stabilization: 0.9,
            humeral_position: HumeralPosition::Neutral,
            lockout_requirement: 0.7,
            compound_factor: 0.8,
        });

        // 9. Barbell Overhead
        triceps_exercises.insert("barbell-overhead", TricepsExerciseProfile {
            elbow_extension_range: (90.0, 180.0),
            shoulder_stabilization: 0.95,
            humeral_position: HumeralPosition::Overhead,
            lockout_requirement: 1.0,
            compound_factor: 0.5,
        });

        // 10. Tricep Rope Pushdown
        triceps_exercises.insert("tricep-rope-pushdown", TricepsExerciseProfile {
            elbow_extension_range: (90.0, 180.0),
            shoulder_stabilization: 0.75,
            humeral_position: HumeralPosition::Flexed(45.0),
            lockout_requirement: 0.85,
            compound_factor: 0.1,
        });

        PoseAnalyzer { triceps_exercises }
    }

    pub fn calculate_triceps_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 33 { // 11 keypoints (upper body focused)
            // Elbow extension (shoulder-elbow-wrist)
            angles.insert("elbow_extension".to_string(), self.calculate_angle(
                &keypoints[9..12],  // Left shoulder
                &keypoints[11..14], // Left elbow
                &keypoints[13..16]  // Left wrist
            ));
            
            // Shoulder position (hip-shoulder-elbow)
            angles.insert("shoulder_position".to_string(), self.calculate_angle(
                &keypoints[15..18], // Left hip
                &keypoints[9..12],  // Left shoulder
                &keypoints[11..14]  // Left elbow
            ));
            
            // Shoulder stability (shoulder-hip-opposite shoulder)
            angles.insert("shoulder_stability".to_string(), self.calculate_angle(
                &keypoints[9..12],   // Left shoulder
                &keypoints[15..18],  // Left hip
                &keypoints[10..13]   // Right shoulder
            ));
            
            // Wrist angle (elbow-wrist-knuckle)
            angles.insert("wrist_angle".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left elbow
                &keypoints[13..16], // Left wrist
                &keypoints[17..20]  // Left hand
            ));
        }
        
        angles
    }

    pub fn check_triceps_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.triceps_exercises.get(exercise_id) {
            // Check elbow extension range
            if let Some(extension) = angles.get("elbow_extension") {
                if *extension < profile.elbow_extension_range.0 {
                    errors.push(format!(
                        "Insufficient extension ({}° < {}°)",
                        extension.round(), profile.elbow_extension_range.0
                    ));
                }
                if *extension > profile.elbow_extension_range.1 {
                    errors.push(format!(
                        "Over-extension ({}° > {}°)",
                        extension.round(), profile.elbow_extension_range.1
                    ));
                }
                
                // Check lockout requirement
                if *extension < 170.0 && profile.lockout_requirement > 0.8 {
                    errors.push("Incomplete lockout".to_string());
                }
            }
            
            // Check shoulder position
            if let (Some(shoulder_pos), Some(humeral_pos)) = (angles.get("shoulder_position"), profile.humeral_position) {
                match humeral_pos {
                    HumeralPosition::Overhead if *shoulder_pos < 160.0 => {
                        errors.push("Maintain overhead position".to_string());
                    },
                    HumeralPosition::Neutral if (*shoulder_pos - 90.0).abs() > 30.0 => {
                        errors.push("Keep shoulders neutral".to_string());
                    },
                    HumeralPosition::Flexed(target) if (*shoulder_pos - target).abs() > 15.0 => {
                        errors.push(format!("Maintain {}° shoulder flexion", target));
                    },
                    HumeralPosition::Extended(target) if (*shoulder_pos - target).abs() > 15.0 => {
                        errors.push(format!("Maintain {}° shoulder extension", target));
                    },
                    _ => {}
                }
            }
            
            // Check shoulder stability
            if let Some(stability) = angles.get("shoulder_stability") {
                let deviation = (stability - 180.0).abs();
                if deviation > (1.0 - profile.shoulder_stabilization) * 30.0 {
                    errors.push("Excessive shoulder movement".to_string());
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "diamond-push-ups" | "forearm-push-ups" => {
                    if let Some(wrist) = angles.get("wrist_angle") {
                        if *wrist < 150.0 {
                            errors.push("Maintain straight wrist position".to_string());
                        }
                    }
                },
                "tricep-rope-pushdown" => {
                    if let Some(extension) = angles.get("elbow_extension") {
                        if *extension < 100.0 {
                            errors.push("Extend through full range".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_triceps_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.triceps_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Elbow extension component (60% weight)
            if let Some(extension) = angles.get("elbow_extension") {
                let ext_norm = (*extension - profile.elbow_extension_range.0) / 
                              (profile.elbow_extension_range.1 - profile.elbow_extension_range.0);
                score += 0.6 * ext_norm.clamp(0.0, 1.0) * profile.lockout_requirement;
            }
            
            // Shoulder stability component (30% weight)
            if let Some(stability) = angles.get("shoulder_stability") {
                let stab_score = 1.0 - (stability - 180.0).abs() / 30.0;
                score += 0.3 * stab_score.clamp(0.0, 1.0) * profile.shoulder_stabilization;
            }
            
            // Compound factor adjustment (10% bonus)
            score += 0.1 * (1.0 - profile.compound_factor);
            
            score.clamp(0.1, 1.0)
        } else {
            0.5
        }
    }
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    shoulder_exercises: HashMap<&'static str, ShoulderExerciseProfile>,
}

#[derive(Debug, Clone)]
struct ShoulderExerciseProfile {
    plane_of_motion: MovementPlane,
    rotation_type: RotationType,
    rom_requirements: (f32, f32), // Degrees of movement
    scapular_behavior: ScapularSetting,
    stability_factor: f32,
}

#[derive(Debug, Clone)]
enum MovementPlane {
    Sagittal,    // Front raises
    Frontal,     // Lateral raises
    Scapular,    // Scaption
    Transverse,  // Rear delt flyes
}

#[derive(Debug, Clone)]
enum RotationType {
    Internal,
    External,
    Neutral,
    Dynamic,
}

#[derive(Debug, Clone)]
enum ScapularSetting {
    Retracted,
    Protracted,
    Dynamic,
    Elevated,
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut shoulder_exercises = HashMap::new();

        // 1. Lateral Dumbbell Raises
        shoulder_exercises.insert("lateral-dumbbell-raises", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Frontal,
            rotation_type: RotationType::Neutral,
            rom_requirements: (60.0, 120.0),
            scapular_behavior: ScapularSetting::Elevated,
            stability_factor: 1.2,
        });

        // 2. Frontal Dumbbell Raises
        shoulder_exercises.insert("frontal-dumbbell-raises", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Sagittal,
            rotation_type: RotationType::Neutral,
            rom_requirements: (70.0, 130.0),
            scapular_behavior: ScapularSetting::Dynamic,
            stability_factor: 1.1,
        });

        // 3. Dumbbell Shrugs
        shoulder_exercises.insert("dumbbell-shrugs", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Sagittal,
            rotation_type: RotationType::Neutral,
            rom_requirements: (160.0, 190.0), // Near-vertical
            scapular_behavior: ScapularSetting::Elevated,
            stability_factor: 0.9,
        });

        // 4. Bench Dumbbell Raises
        shoulder_exercises.insert("bench-dumbbell-raises", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Scapular,
            rotation_type: RotationType::External,
            rom_requirements: (90.0, 150.0),
            scapular_behavior: ScapularSetting::Retracted,
            stability_factor: 1.3,
        });

        // 5. Exterior Dumbbell Raises
        shoulder_exercises.insert("exterior-dumbbell-raises", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Transverse,
            rotation_type: RotationType::External,
            rom_requirements: (80.0, 140.0),
            scapular_behavior: ScapularSetting::Retracted,
            stability_factor: 1.4,
        });

        // 6. Arnold Press
        shoulder_exercises.insert("arnold-press", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Scapular,
            rotation_type: RotationType::Dynamic,
            rom_requirements: (90.0, 180.0),
            scapular_behavior: ScapularSetting::Dynamic,
            stability_factor: 1.5,
        });

        // 7. Military Press
        shoulder_exercises.insert("military-press", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Sagittal,
            rotation_type: RotationType::Neutral,
            rom_requirements: (90.0, 180.0),
            scapular_behavior: ScapularSetting::Retracted,
            stability_factor: 1.6,
        });

        // 8. Handstand Pushups
        shoulder_exercises.insert("handstand-pushups", ShoulderExerciseProfile {
            plane_of_motion: MovementPlane::Sagittal,
            rotation_type: RotationType::Neutral,
            rom_requirements: (160.0, 220.0), // Hyperextension
            scapular_behavior: ScapularSetting::Protracted,
            stability_factor: 1.8,
        });

        PoseAnalyzer { shoulder_exercises }
    }

    pub fn calculate_shoulder_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 51 { // 17 keypoints
            // Shoulder abduction (hip-shoulder-elbow)
            angles.insert("abduction".to_string(), self.calculate_angle(
                &keypoints[23..26], // Left hip
                &keypoints[11..14], // Left shoulder
                &keypoints[13..16]  // Left elbow
            ));
            
            // Shoulder flexion (spine-shoulder-elbow)
            angles.insert("flexion".to_string(), self.calculate_angle(
                &keypoints[0..3],   // Mid-spine/neck
                &keypoints[11..14], // Left shoulder
                &keypoints[13..16]  // Left elbow
            ));
            
            // Scapular movement (shoulder-hip-opposite shoulder)
            angles.insert("scapular_movement".to_string(), self.calculate_angle(
                &keypoints[11..14],  // Left shoulder
                &keypoints[23..26],  // Left hip
                &keypoints[14..17]   // Right shoulder
            ));
            
            // Rotation (elbow-shoulder-wrist)
            angles.insert("rotation".to_string(), self.calculate_angle(
                &keypoints[13..16], // Left elbow
                &keypoints[11..14], // Left shoulder
                &keypoints[15..18]  // Left wrist
            ));
            
            // Stability (shoulder-elbow-wrist)
            angles.insert("stability".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left shoulder
                &keypoints[13..16], // Left elbow
                &keypoints[15..18]  // Left wrist
            ));
        }
        
        angles
    }

    pub fn check_shoulder_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.shoulder_exercises.get(exercise_id) {
            // Check primary plane of motion
            match profile.plane_of_motion {
                MovementPlane::Frontal => {
                    if let Some(abduction) = angles.get("abduction") {
                        if *abduction < profile.rom_requirements.0 {
                            errors.push(format!(
                                "Insufficient abduction ({}° < {}°)",
                                abduction.round(), profile.rom_requirements.0
                            ));
                        }
                        if *abduction > profile.rom_requirements.1 {
                            errors.push(format!(
                                "Over-abduction ({}° > {}°)",
                                abduction.round(), profile.rom_requirements.1
                            ));
                        }
                    }
                },
                MovementPlane::Sagittal => {
                    if let Some(flexion) = angles.get("flexion") {
                        if *flexion < profile.rom_requirements.0 {
                            errors.push(format!(
                                "Insufficient flexion ({}° < {}°)",
                                flexion.round(), profile.rom_requirements.0
                            ));
                        }
                        if *flexion > profile.rom_requirements.1 {
                            errors.push(format!(
                                "Over-flexion ({}° > {}°)",
                                flexion.round(), profile.rom_requirements.1
                            ));
                        }
                    }
                },
                _ => {} // Other planes checked differently
            }
            
            // Check scapular behavior
            match profile.scapular_behavior {
                ScapularSetting::Retracted => {
                    if let Some(scap) = angles.get("scapular_movement") {
                        if *scap < 160.0 {
                            errors.push("Shoulders should remain retracted".to_string());
                        }
                    }
                },
                ScapularSetting::Protracted => {
                    if let Some(scap) = angles.get("scapular_movement") {
                        if *scap > 200.0 {
                            errors.push("Control scapular protraction".to_string());
                        }
                    }
                },
                ScapularSetting::Elevated => {
                    if let Some(abduction) = angles.get("abduction") {
                        if *abduction < 150.0 {
                            errors.push("Maintain shoulder elevation".to_string());
                        }
                    }
                },
                _ => {}
            }
            
            // Check rotation
            match profile.rotation_type {
                RotationType::Internal => {
                    if let Some(rot) = angles.get("rotation") {
                        if *rot > 45.0 {
                            errors.push("Maintain internal rotation".to_string());
                        }
                    }
                },
                RotationType::External => {
                    if let Some(rot) = angles.get("rotation") {
                        if *rot < 135.0 {
                            errors.push("Maintain external rotation".to_string());
                        }
                    }
                },
                RotationType::Dynamic => {
                    if let Some(rot) = angles.get("rotation") {
                        if *rot < 45.0 || *rot > 135.0 {
                            errors.push("Control rotational movement".to_string());
                        }
                    }
                },
                _ => {}
            }
            
            // Exercise-specific checks
            match exercise_id {
                "arnold-press" => {
                    if let Some(rot) = angles.get("rotation") {
                        if !(45.0..=135.0).contains(rot) {
                            errors.push("Complete rotational movement".to_string());
                        }
                    }
                },
                "handstand-pushups" => {
                    if let Some(flexion) = angles.get("flexion") {
                        if *flexion < 170.0 {
                            errors.push("Press to full extension".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_shoulder_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.shoulder_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Primary movement component (50% weight)
            score += match profile.plane_of_motion {
                MovementPlane::Frontal => {
                    angles.get("abduction")
                        .map_or(0.0, |a| 0.5 * ((*a - profile.rom_requirements.0) / 
                                (profile.rom_requirements.1 - profile.rom_requirements.0)).clamp(0.0, 1.0))
                },
                MovementPlane::Sagittal => {
                    angles.get("flexion")
                        .map_or(0.0, |f| 0.5 * ((*f - profile.rom_requirements.0) / 
                                (profile.rom_requirements.1 - profile.rom_requirements.0)).clamp(0.0, 1.0))
                },
                _ => 0.25 // Other planes get base value
            };
            
            // Scapular control component (30% weight)
            score += match profile.scapular_behavior {
                ScapularSetting::Retracted => {
                    angles.get("scapular_movement")
                        .map_or(0.0, |s| 0.3 * (1.0 - (s - 180.0).abs() / 30.0).clamp(0.0, 1.0))
                },
                ScapularSetting::Protracted => {
                    angles.get("scapular_movement")
                        .map_or(0.0, |s| 0.3 * (1.0 - (s - 190.0).abs() / 20.0).clamp(0.0, 1.0))
                },
                ScapularSetting::Elevated => {
                    angles.get("abduction")
                        .map_or(0.0, |a| 0.3 * ((a - 150.0) / 30.0).clamp(0.0, 1.0))
                },
                _ => 0.15 // Dynamic gets base value
            };
            
            // Rotation component (20% weight)
            score += match profile.rotation_type {
                RotationType::Internal => {
                    angles.get("rotation")
                        .map_or(0.0, |r| 0.2 * (1.0 - r / 45.0).clamp(0.0, 1.0))
                },
                RotationType::External => {
                    angles.get("rotation")
                        .map_or(0.0, |r| 0.2 * ((r - 90.0) / 45.0).clamp(0.0, 1.0))
                },
                RotationType::Dynamic => {
                    angles.get("rotation")
                        .map_or(0.0, |r| 0.2 * (1.0 - (r - 90.0).abs() / 45.0).clamp(0.0, 1.0))
                },
                _ => 0.1 // Neutral gets base value
            };
            
            (score * profile.stability_factor).clamp(0.1, 1.0)
        } else {
            0.5
        }
    }
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    quad_exercises: HashMap<&'static str, QuadExerciseProfile>,
}

#[derive(Debug, Clone)]
struct QuadExerciseProfile {
    knee_flexion_range: (f32, f32),      // Optimal knee angles
    hip_depth_range: (f32, f32),         // Hip descent depth
    torso_lean_range: (f32, f32),        // Acceptable torso lean
    stance_width_factor: f32,            // 0=narrow, 1=wide stance
    emphasis: QuadEmphasis,
}

#[derive(Debug, Clone)]
enum QuadEmphasis {
    VastusLateralis,
    VastusMedialis,
    RectusFemoris,
    All,
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut quad_exercises = HashMap::new();

        // 1. Bulgarian Split Squats
        quad_exercises.insert("bulgarian-splits", QuadExerciseProfile {
            knee_flexion_range: (60.0, 120.0),
            hip_depth_range: (80.0, 130.0),
            torso_lean_range: (70.0, 90.0), // More upright
            stance_width_factor: 0.3,
            emphasis: QuadEmphasis::VastusMedialis,
        });

        // 2. Bodyweight Lunges
        quad_exercises.insert("bodyweight-lunges", QuadExerciseProfile {
            knee_flexion_range: (80.0, 140.0),
            hip_depth_range: (90.0, 150.0),
            torso_lean_range: (60.0, 80.0),
            stance_width_factor: 0.5,
            emphasis: QuadEmphasis::RectusFemoris,
        });

        // 3. Goblet Squats
        quad_exercises.insert("goblet-squats", QuadExerciseProfile {
            knee_flexion_range: (90.0, 150.0),
            hip_depth_range: (100.0, 160.0),
            torso_lean_range: (65.0, 85.0),
            stance_width_factor: 0.7,
            emphasis: QuadEmphasis::All,
        });

        // 4. Weighted Lunges
        quad_exercises.insert("weighted-lunges", QuadExerciseProfile {
            knee_flexion_range: (85.0, 135.0),
            hip_depth_range: (95.0, 145.0),
            torso_lean_range: (65.0, 85.0),
            stance_width_factor: 0.5,
            emphasis: QuadEmphasis::VastusLateralis,
        });

        // 5. Side Squats
        quad_exercises.insert("side-squats", QuadExerciseProfile {
            knee_flexion_range: (70.0, 110.0),
            hip_depth_range: (90.0, 140.0),
            torso_lean_range: (75.0, 95.0), // More upright
            stance_width_factor: 0.9,
            emphasis: QuadEmphasis::VastusMedialis,
        });

        // 6. Obstacle Overstep Touches
        quad_exercises.insert("obstacle-overstep-touches", QuadExerciseProfile {
            knee_flexion_range: (100.0, 160.0),
            hip_depth_range: (120.0, 180.0),
            torso_lean_range: (50.0, 70.0), // More forward lean
            stance_width_factor: 0.8,
            emphasis: QuadEmphasis::RectusFemoris,
        });

        // 7. Stand-ups
        quad_exercises.insert("stand-ups", QuadExerciseProfile {
            knee_flexion_range: (30.0, 180.0), // Full ROM
            hip_depth_range: (60.0, 180.0),
            torso_lean_range: (60.0, 90.0),
            stance_width_factor: 0.4,
            emphasis: QuadEmphasis::All,
        });

        // 8. Squat Steps
        quad_exercises.insert("squat-steps", QuadExerciseProfile {
            knee_flexion_range: (90.0, 130.0),
            hip_depth_range: (100.0, 150.0),
            torso_lean_range: (70.0, 90.0),
            stance_width_factor: 0.6,
            emphasis: QuadEmphasis::VastusLateralis,
        });

        // 9. Front Squats
        quad_exercises.insert("front-squats", QuadExerciseProfile {
            knee_flexion_range: (95.0, 155.0),
            hip_depth_range: (110.0, 170.0),
            torso_lean_range: (80.0, 100.0), // Very upright
            stance_width_factor: 0.5,
            emphasis: QuadEmphasis::All,
        });

        // 10. Leg Extensions
        quad_exercises.insert("leg-extensions", QuadExerciseProfile {
            knee_flexion_range: (30.0, 180.0), // Full ROM
            hip_depth_range: (170.0, 180.0),   // Minimal hip movement
            torso_lean_range: (85.0, 95.0),    // Fully upright
            stance_width_factor: 0.1,
            emphasis: QuadEmphasis::VastusMedialis,
        });

        PoseAnalyzer { quad_exercises }
    }

    pub fn calculate_quad_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 51 { // 17 keypoints
            // Knee flexion (hip-knee-ankle)
            angles.insert("knee_flexion".to_string(), self.calculate_angle(
                &keypoints[23..26], // Left hip
                &keypoints[25..28], // Left knee
                &keypoints[27..30]  // Left ankle
            ));
            
            // Hip depth (shoulder-hip-knee)
            angles.insert("hip_depth".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left shoulder
                &keypoints[23..26], // Left hip
                &keypoints[25..28]  // Left knee
            ));
            
            // Torso lean (hip-shoulder-neck)
            angles.insert("torso_lean".to_string(), self.calculate_angle(
                &keypoints[23..26], // Left hip
                &keypoints[11..14], // Left shoulder
                &keypoints[0..3]    // Nose/neck
            ));
            
            // Stance width (hip-knee-ankle horizontal distance)
            angles.insert("stance_width".to_string(), 
                (keypoints[24] - keypoints[23]).abs() // Right hip X - Left hip X
            );
            
            // Knee alignment (hip-knee-ankle frontal plane)
            angles.insert("knee_alignment".to_string(), self.calculate_frontal_angle(
                &keypoints[23..26], // Left hip
                &keypoints[25..28], // Left knee
                &keypoints[27..30]  // Left ankle
            ));
        }
        
        angles
    }

    pub fn check_quad_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.quad_exercises.get(exercise_id) {
            // Check knee flexion range
            if let Some(knee) = angles.get("knee_flexion") {
                if *knee < profile.knee_flexion_range.0 {
                    errors.push(format!(
                        "Insufficient knee bend ({}° < {}°)",
                        knee.round(), profile.knee_flexion_range.0
                    ));
                }
                if *knee > profile.knee_flexion_range.1 {
                    errors.push(format!(
                        "Excessive knee flexion ({}° > {}°)",
                        knee.round(), profile.knee_flexion_range.1
                    ));
                }
            }
            
            // Check hip depth
            if let Some(hip) = angles.get("hip_depth") {
                if *hip < profile.hip_depth_range.0 {
                    errors.push(format!(
                        "Insufficient depth ({}° < {}°)",
                        hip.round(), profile.hip_depth_range.0
                    ));
                }
                if *hip > profile.hip_depth_range.1 {
                    errors.push(format!(
                        "Over-depth ({}° > {}°)",
                        hip.round(), profile.hip_depth_range.1
                    ));
                }
            }
            
            // Check torso lean
            if let Some(torso) = angles.get("torso_lean") {
                if *torso < profile.torso_lean_range.0 {
                    errors.push(format!(
                        "Excessive forward lean ({}° < {}°)",
                        torso.round(), profile.torso_lean_range.0
                    ));
                }
                if *torso > profile.torso_lean_range.1 {
                    errors.push(format!(
                        "Overly upright ({}° > {}°)",
                        torso.round(), profile.torso_lean_range.1
                    ));
                }
            }
            
            // Check knee alignment
            if let Some(alignment) = angles.get("knee_alignment") {
                if alignment.abs() > 15.0 {
                    errors.push("Knee valgus/varus detected".to_string());
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "bulgarian-splits" => {
                    if let Some(stance) = angles.get("stance_width") {
                        if *stance < 0.2 {
                            errors.push("Maintain proper split stance width".to_string());
                        }
                    }
                },
                "front-squats" => {
                    if let Some(torso) = angles.get("torso_lean") {
                        if *torso < 75.0 {
                            errors.push("Maintain upright torso position".to_string());
                        }
                    }
                },
                "side-squats" => {
                    if let Some(alignment) = angles.get("knee_alignment") {
                        if alignment.abs() < 10.0 {
                            errors.push("Intentional lateral movement required".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_quad_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.quad_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Knee flexion component (40% weight)
            if let Some(knee) = angles.get("knee_flexion") {
                let knee_norm = (*knee - profile.knee_flexion_range.0) / 
                               (profile.knee_flexion_range.1 - profile.knee_flexion_range.0);
                score += 0.4 * knee_norm.clamp(0.0, 1.0);
            }
            
            // Hip depth component (30% weight)
            if let Some(hip) = angles.get("hip_depth") {
                let hip_norm = (*hip - profile.hip_depth_range.0) / 
                              (profile.hip_depth_range.1 - profile.hip_depth_range.0);
                score += 0.3 * hip_norm.clamp(0.0, 1.0);
            }
            
            // Muscle emphasis component (20% weight)
            score += match profile.emphasis {
                QuadEmphasis::VastusLateralis => {
                    if let Some(stance) = angles.get("stance_width") {
                        0.2 * (stance * profile.stance_width_factor).clamp(0.0, 1.0)
                    } else { 0.0 }
                },
                QuadEmphasis::VastusMedialis => {
                    if let Some(alignment) = angles.get("knee_alignment") {
                        0.2 * (1.0 - (alignment.abs() / 30.0)).clamp(0.0, 1.0)
                    } else { 0.0 }
                },
                QuadEmphasis::RectusFemoris => {
                    if let Some(torso) = angles.get("torso_lean") {
                        0.2 * ((torso - 60.0) / 30.0).clamp(0.0, 1.0)
                    } else { 0.0 }
                },
                QuadEmphasis::All => 0.2
            };
            
            // Stance width component (10% weight)
            if let Some(stance) = angles.get("stance_width") {
                let stance_dev = (stance - profile.stance_width_factor).abs();
                score += 0.1 * (1.0 - stance_dev).clamp(0.0, 1.0);
            }
            
            score.clamp(0.1, 1.0)
        } else {
            0.5
        }
    }

    fn calculate_frontal_angle(&self, a: &[f32], b: &[f32], c: &[f32]) -> f32 {
        // Calculates knee valgus/varus in frontal plane (X-Z coordinates)
        if a.len() < 3 || b.len() < 3 || c.len() < 3 {
            return 0.0;
        }
        
        let ab = (b[0] - a[0], b[2] - a[2]); // Hip to knee vector
        let cb = (b[0] - c[0], b[2] - c[2]); // Ankle to knee vector
        
        let dot = ab.0 * cb.0 + ab.1 * cb.1;
        let cross = ab.0 * cb.1 - ab.1 * cb.0;
        
        cross.atan2(dot).to_degrees()
    }
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    hamstring_exercises: HashMap<&'static str, HamstringExerciseProfile>,
}

#[derive(Debug, Clone)]
struct HamstringExerciseProfile {
    knee_flexion_range: (f32, f32),      // Optimal knee angles
    hip_hinge_range: (f32, f32),         // Hip flexion/extension
    lumbar_stability_threshold: f32,     // Lower back control
    eccentric_emphasis: f32,             // 0-1 how important lowering phase is
    muscle_balance: HamstringBalance,    // Which part of hamstrings is emphasized
}

#[derive(Debug, Clone)]
enum HamstringBalance {
    BicepsFemoris,
    Semitendinosus,
    MedialLateral,
    All,
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut hamstring_exercises = HashMap::new();

        // 1. Romanian Deadlifts
        hamstring_exercises.insert("romanian-deadlifts", HamstringExerciseProfile {
            knee_flexion_range: (160.0, 180.0), // Slightly bent
            hip_hinge_range: (60.0, 120.0),
            lumbar_stability_threshold: 0.95,
            eccentric_emphasis: 0.8,
            muscle_balance: HamstringBalance::All,
        });

        // 2. Hamstring Curls
        hamstring_exercises.insert("hamstring-curls", HamstringExerciseProfile {
            knee_flexion_range: (30.0, 150.0), // Full ROM
            hip_hinge_range: (170.0, 180.0),   // Minimal hip movement
            lumbar_stability_threshold: 0.85,
            eccentric_emphasis: 0.7,
            muscle_balance: HamstringBalance::MedialLateral,
        });

        // 3. Kettlebell Good Morning
        hamstring_exercises.insert("kettlebell-good-morning", HamstringExerciseProfile {
            knee_flexion_range: (170.0, 180.0),
            hip_hinge_range: (70.0, 130.0),
            lumbar_stability_threshold: 0.9,
            eccentric_emphasis: 0.6,
            muscle_balance: HamstringBalance::BicepsFemoris,
        });

        // 4. Glute-Ham Raises
        hamstring_exercises.insert("glute-ham-raises", HamstringExerciseProfile {
            knee_flexion_range: (90.0, 180.0),
            hip_hinge_range: (120.0, 180.0),
            lumbar_stability_threshold: 0.8,
            eccentric_emphasis: 0.9,
            muscle_balance: HamstringBalance::All,
        });

        // 5. Single-Leg Deadlifts
        hamstring_exercises.insert("single-leg-deadlifts", HamstringExerciseProfile {
            knee_flexion_range: (170.0, 180.0),
            hip_hinge_range: (80.0, 140.0),
            lumbar_stability_threshold: 0.85,
            eccentric_emphasis: 0.7,
            muscle_balance: HamstringBalance::Semitendinosus,
        });

        // 6. Seated Leg Curls
        hamstring_exercises.insert("seated-leg-curls", HamstringExerciseProfile {
            knee_flexion_range: (45.0, 135.0),
            hip_hinge_range: (175.0, 180.0),
            lumbar_stability_threshold: 0.75,
            eccentric_emphasis: 0.5,
            muscle_balance: HamstringBalance::MedialLateral,
        });

        // 7. Nordic Hamstring Curls
        hamstring_exercises.insert("nordic-hamstring-curls", HamstringExerciseProfile {
            knee_flexion_range: (120.0, 180.0),
            hip_hinge_range: (160.0, 180.0),
            lumbar_stability_threshold: 0.7,
            eccentric_emphasis: 1.0,
            muscle_balance: HamstringBalance::BicepsFemoris,
        });

        // 8. Stiff-Leg Deadlifts
        hamstring_exercises.insert("stiff-leg-deadlifts", HamstringExerciseProfile {
            knee_flexion_range: (175.0, 180.0),
            hip_hinge_range: (60.0, 120.0),
            lumbar_stability_threshold: 0.9,
            eccentric_emphasis: 0.8,
            muscle_balance: HamstringBalance::All,
        });

        // 9. Swiss Ball Hamstring Curls
        hamstring_exercises.insert("swiss-ball-hamstring-curls", HamstringExerciseProfile {
            knee_flexion_range: (90.0, 160.0),
            hip_hinge_range: (150.0, 180.0),
            lumbar_stability_threshold: 0.8,
            eccentric_emphasis: 0.6,
            muscle_balance: HamstringBalance::Semitendinosus,
        });

        // 10. Reverse Hyperextensions
        hamstring_exercises.insert("reverse-hyperextensions", HamstringExerciseProfile {
            knee_flexion_range: (150.0, 180.0),
            hip_hinge_range: (160.0, 200.0), // Hyperextension
            lumbar_stability_threshold: 0.85,
            eccentric_emphasis: 0.5,
            muscle_balance: HamstringBalance::BicepsFemoris,
        });

        PoseAnalyzer { hamstring_exercises }
    }

    pub fn calculate_hamstring_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 51 { // 17 keypoints
            // Knee flexion (hip-knee-ankle)
            angles.insert("knee_flexion".to_string(), self.calculate_angle(
                &keypoints[23..26], // Left hip
                &keypoints[25..28], // Left knee
                &keypoints[27..30]  // Left ankle
            ));
            
            // Hip hinge (shoulder-hip-knee)
            angles.insert("hip_hinge".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left shoulder
                &keypoints[23..26], // Left hip
                &keypoints[25..28]  // Left knee
            ));
            
            // Lumbar stability (shoulder-hip-opposite hip)
            angles.insert("lumbar_stability".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left shoulder
                &keypoints[23..26], // Left hip
                &keypoints[24..27]  // Right hip
            ));
            
            // Muscle balance (knee-ankle-foot angle)
            angles.insert("muscle_balance".to_string(), self.calculate_angle(
                &keypoints[25..28], // Left knee
                &keypoints[27..30], // Left ankle
                &keypoints[29..32]  // Left foot
            ));
        }
        
        angles
    }

    pub fn check_hamstring_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.hamstring_exercises.get(exercise_id) {
            // Check knee flexion range
            if let Some(knee) = angles.get("knee_flexion") {
                if *knee < profile.knee_flexion_range.0 {
                    errors.push(format!(
                        "Insufficient knee flexion ({}° < {}°)",
                        knee.round(), profile.knee_flexion_range.0
                    ));
                }
                if *knee > profile.knee_flexion_range.1 {
                    errors.push(format!(
                        "Excessive knee flexion ({}° > {}°)",
                        knee.round(), profile.knee_flexion_range.1
                    ));
                }
            }
            
            // Check hip hinge range
            if let Some(hip) = angles.get("hip_hinge") {
                if *hip < profile.hip_hinge_range.0 {
                    errors.push(format!(
                        "Insufficient hip hinge ({}° < {}°)",
                        hip.round(), profile.hip_hinge_range.0
                    ));
                }
                if *hip > profile.hip_hinge_range.1 {
                    errors.push(format!(
                        "Over-extension ({}° > {}°)",
                        hip.round(), profile.hip_hinge_range.1
                    ));
                }
            }
            
            // Check lumbar stability
            if let Some(lumbar) = angles.get("lumbar_stability") {
                let deviation = (lumbar - 180.0).abs();
                if deviation > (1.0 - profile.lumbar_stability_threshold) * 30.0 {
                    errors.push("Lumbar instability detected".to_string());
                }
            }
            
            // Check muscle balance
            if let Some(balance) = angles.get("muscle_balance") {
                match profile.muscle_balance {
                    HamstringBalance::BicepsFemoris if *balance < 160.0 => {
                        errors.push("Focus on lateral hamstring engagement".to_string());
                    },
                    HamstringBalance::Semitendinosus if *balance > 200.0 => {
                        errors.push("Focus on medial hamstring engagement".to_string());
                    },
                    HamstringBalance::MedialLateral if *balance < 170.0 || *balance > 190.0 => {
                        errors.push("Maintain balanced hamstring activation".to_string());
                    },
                    _ => {}
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "nordic-hamstring-curls" => {
                    if let Some(knee) = angles.get("knee_flexion") {
                        if *knee < 150.0 {
                            errors.push("Control eccentric phase".to_string());
                        }
                    }
                },
                "stiff-leg-deadlifts" => {
                    if let Some(hip) = angles.get("hip_hinge") {
                        if *hip < 170.0 {
                            errors.push("Maintain straighter legs".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_hamstring_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>, is_eccentric: bool) -> f32 {
        if let Some(profile) = self.hamstring_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Knee flexion component (40% weight)
            if let Some(knee) = angles.get("knee_flexion") {
                let knee_norm = (*knee - profile.knee_flexion_range.0) / 
                               (profile.knee_flexion_range.1 - profile.knee_flexion_range.0);
                score += 0.4 * knee_norm.clamp(0.0, 1.0);
            }
            
            // Hip hinge component (30% weight)
            if let Some(hip) = angles.get("hip_hinge") {
                let hip_norm = (*hip - profile.hip_hinge_range.0) / 
                              (profile.hip_hinge_range.1 - profile.hip_hinge_range.0);
                score += 0.3 * hip_norm.clamp(0.0, 1.0);
            }
            
            // Lumbar stability component (20% weight)
            if let Some(lumbar) = angles.get("lumbar_stability") {
                let lumbar_score = 1.0 - (lumbar - 180.0).abs() / 30.0;
                score += 0.2 * lumbar_score.clamp(0.0, 1.0) * profile.lumbar_stability_threshold;
            }
            
            // Eccentric emphasis bonus (10% weight)
            if is_eccentric {
                score += 0.1 * profile.eccentric_emphasis;
            }
            
            score.clamp(0.1, 1.0)
        } else {
            0.5
        }
    }
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    abs_exercises: HashMap<&'static str, AbExerciseProfile>,
}

#[derive(Debug, Clone)]
struct AbExerciseProfile {
    primary_joints: Vec<&'static str>,
    target_angles: HashMap<&'static str, (f32, f32)>,
    common_errors: Vec<&'static str>,
    engagement_factor: f32,
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    back_exercises: HashMap<&'static str, BackExerciseProfile>,
}

#[derive(Debug, Clone)]
struct BackExerciseProfile {
    primary_muscles: Vec<&'static str>,
    joint_ranges: HashMap<&'static str, (f32, f32)>,
    strictness_factor: f32,
    common_mistakes: Vec<&'static str>,
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    biceps_exercises: HashMap<&'static str, BicepsExerciseProfile>,
}

#[derive(Debug, Clone)]
struct BicepsExerciseProfile {
    curl_angle_range: (f32, f32),
    shoulder_stabilization: f32, // 0-1 how much shoulder should stay fixed
    elbow_travel: (f32, f32),   // Expected elbow movement range
    strictness: f32,
    common_faults: Vec<&'static str>,
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    calf_exercises: HashMap<&'static str, CalfExerciseProfile>,
}

#[derive(Debug, Clone)]
struct CalfExerciseProfile {
    plantar_flexion_range: (f32, f32), // Expected ankle angle range
    knee_angle_range: (f32, f32),      // Knee position during exercise
    stability_threshold: f32,          // How much hip/knee movement allowed
    emphasis: CalfEmphasis,            // Which head of calf is emphasized
    strictness: f32,
}

#[derive(Debug, Clone)]
enum CalfEmphasis {
    Gastrocnemius,
    Soleus,
    Both,
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    chest_exercises: HashMap<&'static str, ChestExerciseProfile>,
}

#[derive(Debug, Clone)]
struct ChestExerciseProfile {
    press_angle_range: (f32, f32),      // Shoulder flexion range
    elbow_path: ElbowPath,              // Ideal elbow trajectory
    scapular_behavior: ScapularSetting, // Proper scapular movement
    depth_requirement: f32,             // 0-1 how deep the rep should be
    stability_factor: f32,
}

#[derive(Debug, Clone)]
enum ElbowPath {
    Flared(f32),  // Degrees from torso
    Tucked(f32),  // Degrees from torso
    Variable(f32, f32) // Min-max degrees
}

#[derive(Debug, Clone)]
enum ScapularSetting {
    Retracted,
    Protracted,
    Dynamic
}

// wasm/src/pose_detection.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct PoseAnalyzer {
    glute_exercises: HashMap<&'static str, GluteExerciseProfile>,
}

#[derive(Debug, Clone)]
struct GluteExerciseProfile {
    hip_extension_range: (f32, f32),   // Optimal hip extension angles
    knee_angle_range: (f32, f32),      // Companion knee angles
    lumbar_stability_threshold: f32,   // Lower back stability
    unilateral_factor: f32,            // 0=bilateral, 1=unilateral
    activation_emphasis: GluteActivation,
}

#[derive(Debug, Clone)]
enum GluteActivation {
    Maximus,
    Medius,
    Both,
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut glute_exercises = HashMap::new();

        // 1. Superman
        glute_exercises.insert("superman", GluteExerciseProfile {
            hip_extension_range: (10.0, 30.0), // Isometric hold range
            knee_angle_range: (170.0, 180.0),
            lumbar_stability_threshold: 0.9,
            unilateral_factor: 0.3, // Slightly unilateral
            activation_emphasis: GluteActivation::Maximus,
        });

        // 2. Good Morning
        glute_exercises.insert("good-morning", GluteExerciseProfile {
            hip_extension_range: (60.0, 180.0), // Full ROM
            knee_angle_range: (170.0, 180.0),   // Near-locked
            lumbar_stability_threshold: 0.95,
            unilateral_factor: 0.0,
            activation_emphasis: GluteActivation::Maximus,
        });

        // 3. Yoga Ball Glute Raises
        glute_exercises.insert("yoga-ball-glute-raises", GluteExerciseProfile {
            hip_extension_range: (160.0, 200.0), // Hyperextension
            knee_angle_range: (90.0, 120.0),     // Bent knees
            lumbar_stability_threshold: 0.85,
            unilateral_factor: 0.0,
            activation_emphasis: GluteActivation::Both,
        });

        // 4. Donkey Kick
        glute_exercises.insert("donkey-kick", GluteExerciseProfile {
            hip_extension_range: (120.0, 180.0),
            knee_angle_range: (90.0, 120.0),
            lumbar_stability_threshold: 0.8,
            unilateral_factor: 1.0,
            activation_emphasis: GluteActivation::Medius,
        });

        // 5. Inverse Kick Back
        glute_exercises.insert("inverse-kick-back", GluteExerciseProfile {
            hip_extension_range: (140.0, 190.0),
            knee_angle_range: (160.0, 180.0),
            lumbar_stability_threshold: 0.75,
            unilateral_factor: 1.0,
            activation_emphasis: GluteActivation::Maximus,
        });

        // 6. Barbell Bench Touches
        glute_exercises.insert("barbell-bench-touches", GluteExerciseProfile {
            hip_extension_range: (70.0, 120.0),
            knee_angle_range: (160.0, 180.0),
            lumbar_stability_threshold: 0.7,
            unilateral_factor: 0.5,
            activation_emphasis: GluteActivation::Medius,
        });

        // 7. Barbell Hip Thrust
        glute_exercises.insert("barbell-hip-thrust", GluteExerciseProfile {
            hip_extension_range: (160.0, 190.0),
            knee_angle_range: (90.0, 100.0),
            lumbar_stability_threshold: 0.9,
            unilateral_factor: 0.0,
            activation_emphasis: GluteActivation::Maximus,
        });

        // 8. Curtsy Lunges
        glute_exercises.insert("curtsy-lunges", GluteExerciseProfile {
            hip_extension_range: (80.0, 140.0),
            knee_angle_range: (90.0, 130.0),
            lumbar_stability_threshold: 0.85,
            unilateral_factor: 0.8,
            activation_emphasis: GluteActivation::Medius,
        });

        // 9. Cable Pull Through
        glute_exercises.insert("cable-pull-through", GluteExerciseProfile {
            hip_extension_range: (100.0, 170.0),
            knee_angle_range: (120.0, 160.0),
            lumbar_stability_threshold: 0.8,
            unilateral_factor: 0.0,
            activation_emphasis: GluteActivation::Both,
        });

        // 10. Frog Pumps
        glute_exercises.insert("frog-pumps", GluteExerciseProfile {
            hip_extension_range: (150.0, 190.0),
            knee_angle_range: (45.0, 90.0), // Wide stance
            lumbar_stability_threshold: 0.7,
            unilateral_factor: 0.0,
            activation_emphasis: GluteActivation::Medius,
        });

        PoseAnalyzer { glute_exercises }
    }

    pub fn calculate_glute_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 51 { // 17 keypoints
            // Hip extension (shoulder-hip-knee)
            angles.insert("hip_extension".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left shoulder
                &keypoints[23..26], // Left hip
                &keypoints[25..28]  // Left knee
            ));
            
            // Knee angle (hip-knee-ankle)
            angles.insert("knee_angle".to_string(), self.calculate_angle(
                &keypoints[23..26], // Left hip
                &keypoints[25..28], // Left knee
                &keypoints[27..30]  // Left ankle
            ));
            
            // Lumbar stability (shoulder-hip-opposite hip)
            angles.insert("lumbar_stability".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left shoulder
                &keypoints[23..26], // Left hip
                &keypoints[24..27]  // Right hip
            ));
            
            // Unilateral loading (hip height difference)
            angles.insert("unilateral_loading".to_string(), 
                (keypoints[24] - keypoints[23]).abs() // Right hip Y - Left hip Y
            );
        }
        
        angles
    }

    pub fn check_glute_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.glute_exercises.get(exercise_id) {
            // Check hip extension range
            if let Some(hip_ext) = angles.get("hip_extension") {
                if *hip_ext < profile.hip_extension_range.0 {
                    errors.push(format!(
                        "Insufficient hip extension ({}° < {}°)",
                        hip_ext.round(), profile.hip_extension_range.0
                    ));
                }
                if *hip_ext > profile.hip_extension_range.1 {
                    errors.push(format!(
                        "Over-extension ({}° > {}°)",
                        hip_ext.round(), profile.hip_extension_range.1
                    ));
                }
            }
            
            // Check knee angle
            if let Some(knee) = angles.get("knee_angle") {
                if *knee < profile.knee_angle_range.0 {
                    errors.push(format!(
                        "Knees too bent ({}° < {}°)",
                        knee.round(), profile.knee_angle_range.0
                    ));
                }
                if *knee > profile.knee_angle_range.1 {
                    errors.push(format!(
                        "Knees too straight ({}° > {}°)",
                        knee.round(), profile.knee_angle_range.1
                    ));
                }
            }
            
            // Check lumbar stability
            if let Some(lumbar) = angles.get("lumbar_stability") {
                let deviation = (lumbar - 180.0).abs();
                if deviation > (1.0 - profile.lumbar_stability_threshold) * 30.0 {
                    errors.push("Lumbar instability detected".to_string());
                }
            }
            
            // Check unilateral loading
            if let Some(unilateral) = angles.get("unilateral_loading") {
                let expected_asymmetry = profile.unilateral_factor * 0.2; // Normalized
                if (unilateral - expected_asymmetry).abs() > 0.1 {
                    errors.push("Improper weight distribution".to_string());
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "frog-pumps" => {
                    if let Some(knee) = angles.get("knee_angle") {
                        if *knee > 100.0 {
                            errors.push("Maintain wider stance".to_string());
                        }
                    }
                },
                "good-morning" => {
                    if let Some(hip_ext) = angles.get("hip_extension") {
                        if *hip_ext < 170.0 {
                            errors.push("Incomplete hip extension".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_glute_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.glute_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Hip extension component (50% weight)
            if let Some(hip_ext) = angles.get("hip_extension") {
                let ext_norm = (*hip_ext - profile.hip_extension_range.0) / 
                              (profile.hip_extension_range.1 - profile.hip_extension_range.0);
                score += 0.5 * ext_norm.clamp(0.0, 1.0);
            }
            
            // Knee position component (20% weight)
            if let Some(knee) = angles.get("knee_angle") {
                let knee_dev = match profile.activation_emphasis {
                    GluteActivation::Maximus => 1.0 - (knee - 90.0).abs() / 45.0,
                    GluteActivation::Medius => 1.0 - (knee - 120.0).abs() / 60.0,
                    GluteActivation::Both => 1.0 - (knee - 105.0).abs() / 50.0,
                };
                score += 0.2 * knee_dev.clamp(0.0, 1.0);
            }
            
            // Lumbar stability component (20% weight)
            if let Some(lumbar) = angles.get("lumbar_stability") {
                let lumbar_score = 1.0 - (lumbar - 180.0).abs() / 30.0;
                score += 0.2 * lumbar_score.clamp(0.0, 1.0) * profile.lumbar_stability_threshold;
            }
            
            // Unilateral component (10% weight)
            if let Some(unilateral) = angles.get("unilateral_loading") {
                let uni_score = 1.0 - (unilateral - profile.unilateral_factor * 0.2).abs() / 0.1;
                score += 0.1 * uni_score.clamp(0.0, 1.0);
            }
            
            score.clamp(0.1, 1.0)
        } else {
            0.5
        }
    }
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut chest_exercises = HashMap::new();

        // 1. Inner Push-ups
        chest_exercises.insert("inner-push-ups", ChestExerciseProfile {
            press_angle_range: (70.0, 120.0),
            elbow_path: ElbowPath::Tucked(30.0),
            scapular_behavior: ScapularSetting::Dynamic,
            depth_requirement: 0.9,
            stability_factor: 1.2,
        });

        // 2. Superman Push-ups
        chest_exercises.insert("superman-push-ups", ChestExerciseProfile {
            press_angle_range: (100.0, 150.0),
            elbow_path: ElbowPath::Flared(60.0),
            scapular_behavior: ScapularSetting::Protracted,
            depth_requirement: 0.8,
            stability_factor: 1.5,
        });

        // 3. Butterfly (Pec Deck)
        chest_exercises.insert("butterfly", ChestExerciseProfile {
            press_angle_range: (120.0, 180.0),
            elbow_path: ElbowPath::Flared(75.0),
            scapular_behavior: ScapularSetting::Retracted,
            depth_requirement: 0.95,
            stability_factor: 1.1,
        });

        // 4. Dumbbell Overhead
        chest_exercises.insert("dumbbell-overhead", ChestExerciseProfile {
            press_angle_range: (150.0, 210.0),
            elbow_path: ElbowPath::Variable(45.0, 75.0),
            scapular_behavior: ScapularSetting::Dynamic,
            depth_requirement: 0.85,
            stability_factor: 1.4,
        });

        // 5. Military Press
        chest_exercises.insert("military-press", ChestExerciseProfile {
            press_angle_range: (160.0, 200.0),
            elbow_path: ElbowPath::Tucked(45.0),
            scapular_behavior: ScapularSetting::Retracted,
            depth_requirement: 0.8,
            stability_factor: 1.3,
        });

        // 6. Dumbbell Bench Press
        chest_exercises.insert("bench-press-dumbbell", ChestExerciseProfile {
            press_angle_range: (75.0, 135.0),
            elbow_path: ElbowPath::Variable(45.0, 60.0),
            scapular_behavior: ScapularSetting::Retracted,
            depth_requirement: 0.9,
            stability_factor: 1.2,
        });

        // 7. Barbell Bench Press
        chest_exercises.insert("bench-press-barbell", ChestExerciseProfile {
            press_angle_range: (80.0, 140.0),
            elbow_path: ElbowPath::Variable(50.0, 70.0),
            scapular_behavior: ScapularSetting::Retracted,
            depth_requirement: 0.95,
            stability_factor: 1.3,
        });

        // 8. Bench Butterfly
        chest_exercises.insert("bench-butterfly", ChestExerciseProfile {
            press_angle_range: (100.0, 160.0),
            elbow_path: ElbowPath::Flared(80.0),
            scapular_behavior: ScapularSetting::Dynamic,
            depth_requirement: 0.85,
            stability_factor: 1.1,
        });

        // 9. Dumbbell Rows (for chest crossover)
        chest_exercises.insert("dumbbell-rows", ChestExerciseProfile {
            press_angle_range: (60.0, 120.0),
            elbow_path: ElbowPath::Variable(30.0, 60.0),
            scapular_behavior: ScapularSetting::Dynamic,
            depth_requirement: 0.7,
            stability_factor: 1.0,
        });

        // 10. Open Butterfly
        chest_exercises.insert("open-butterfly", ChestExerciseProfile {
            press_angle_range: (90.0, 180.0),
            elbow_path: ElbowPath::Flared(90.0),
            scapular_behavior: ScapularSetting::Protracted,
            depth_requirement: 0.75,
            stability_factor: 1.2,
        });

        PoseAnalyzer { chest_exercises }
    }

    pub fn calculate_chest_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 51 { // 17 keypoints
            // Shoulder flexion (hip-shoulder-elbow)
            angles.insert("shoulder_flexion".to_string(), self.calculate_angle(
                &keypoints[23..26], // Left hip
                &keypoints[11..14], // Left shoulder
                &keypoints[13..16]  // Left elbow
            ));
            
            // Elbow path (hip-shoulder-elbow projected angle)
            angles.insert("elbow_path".to_string(), self.calculate_projected_angle(
                &keypoints[23..26], // Hip
                &keypoints[11..14], // Shoulder
                &keypoints[13..16]  // Elbow
            ));
            
            // Scapular movement (shoulder-hip-opposite shoulder)
            angles.insert("scapular_movement".to_string(), self.calculate_angle(
                &keypoints[11..14],  // Left shoulder
                &keypoints[23..26],   // Left hip
                &keypoints[14..17]    // Right shoulder
            ));
            
            // Press depth (wrist-shoulder-hip)
            angles.insert("press_depth".to_string(), self.calculate_angle(
                &keypoints[15..18],  // Left wrist
                &keypoints[11..14],  // Left shoulder
                &keypoints[23..26]   // Left hip
            ));
        }
        
        angles
    }

    pub fn check_chest_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.chest_exercises.get(exercise_id) {
            // Check shoulder flexion range
            if let Some(flexion) = angles.get("shoulder_flexion") {
                if *flexion < profile.press_angle_range.0 {
                    errors.push(format!(
                        "Insufficient press depth ({}° < {}°)",
                        flexion.round(), profile.press_angle_range.0
                    ));
                }
                if *flexion > profile.press_angle_range.1 {
                    errors.push(format!(
                        "Over-extension ({}° > {}°)",
                        flexion.round(), profile.press_angle_range.1
                    ));
                }
            }
            
            // Check elbow path
            if let (Some(elbow_path), Some(path_type)) = (angles.get("elbow_path"), profile.elbow_path) {
                let (min, max) = match path_type {
                    ElbowPath::Flared(angle) => (angle - 15.0, angle + 15.0),
                    ElbowPath::Tucked(angle) => (angle - 15.0, angle + 15.0),
                    ElbowPath::Variable(min, max) => (*min, *max),
                };
                
                if *elbow_path < min {
                    errors.push(format!(
                        "Elbows too tucked ({}° < {}°)",
                        elbow_path.round(), min
                    ));
                }
                if *elbow_path > max {
                    errors.push(format!(
                        "Elbows too flared ({}° > {}°)",
                        elbow_path.round(), max
                    ));
                }
            }
            
            // Check scapular behavior
            if let (Some(scap_movement), Some(scap_setting)) = (angles.get("scapular_movement"), profile.scapular_behavior) {
                match scap_setting {
                    ScapularSetting::Retracted if *scap_movement < 160.0 => {
                        errors.push("Maintain retracted scapulae".to_string());
                    },
                    ScapularSetting::Protracted if *scap_movement > 200.0 => {
                        errors.push("Control scapular protraction".to_string());
                    },
                    ScapularSetting::Dynamic => {
                        if *scap_movement < 150.0 || *scap_movement > 210.0 {
                            errors.push("Abnormal scapular movement".to_string());
                        }
                    },
                    _ => {}
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "superman-push-ups" => {
                    if let Some(depth) = angles.get("press_depth") {
                        if *depth < 60.0 {
                            errors.push("Insufficient chest-to-floor distance".to_string());
                        }
                    }
                },
                "military-press" => {
                    if let Some(flexion) = angles.get("shoulder_flexion") {
                        if *flexion < 170.0 {
                            errors.push("Press should go fully overhead".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_chest_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.chest_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Press range component (40% weight)
            if let Some(flexion) = angles.get("shoulder_flexion") {
                let flex_norm = (*flexion - profile.press_angle_range.0) / 
                                (profile.press_angle_range.1 - profile.press_angle_range.0);
                score += 0.4 * flex_norm.clamp(0.0, 1.0);
            }
            
            // Elbow path component (30% weight)
            if let (Some(elbow_path), Some(path_type)) = (angles.get("elbow_path"), profile.elbow_path) {
                let path_score = match path_type {
                    ElbowPath::Flared(target) => 1.0 - (elbow_path - target).abs() / 30.0,
                    ElbowPath::Tucked(target) => 1.0 - (elbow_path - target).abs() / 30.0,
                    ElbowPath::Variable(min, max) => {
                        if elbow_path < min {
                            1.0 - (min - elbow_path) / 30.0
                        } else if elbow_path > max {
                            1.0 - (elbow_path - max) / 30.0
                        } else {
                            1.0
                        }
                    }
                };
                score += 0.3 * path_score.clamp(0.0, 1.0);
            }
            
            // Scapular control component (20% weight)
            if let Some(scap_movement) = angles.get("scapular_movement") {
                let scap_score = match profile.scapular_behavior {
                    ScapularSetting::Retracted => 1.0 - (scap_movement - 180.0).abs() / 30.0,
                    ScapularSetting::Protracted => 1.0 - (scap_movement - 190.0).abs() / 20.0,
                    ScapularSetting::Dynamic => 1.0 - (scap_movement - 180.0).abs() / 45.0,
                };
                score += 0.2 * scap_score.clamp(0.0, 1.0);
            }
            
            // Depth component (10% weight)
            if let Some(depth) = angles.get("press_depth") {
                score += 0.1 * (*depth / 180.0).clamp(0.0, 1.0) * profile.depth_requirement;
            }
            
            (score * profile.stability_factor).clamp(0.1, 1.0)
        } else {
            0.5
        }
    }

    fn calculate_projected_angle(&self, a: &[f32], b: &[f32], c: &[f32]) -> f32 {
        // Projects elbow angle onto torso plane
        if a.len() < 2 || b.len() < 2 || c.len() < 2 {
            return 0.0;
        }
        
        let torso_vector = (a[0] - b[0], a[1] - b[1]);
        let arm_vector = (c[0] - b[0], c[1] - b[1]);
        
        let dot = torso_vector.0 * arm_vector.0 + torso_vector.1 * arm_vector.1;
        let det = torso_vector.0 * arm_vector.1 - torso_vector.1 * arm_vector.0;
        
        det.atan2(dot).to_degrees().abs()
    }
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut calf_exercises = HashMap::new();

        // 1. Bench Calf Raises
        calf_exercises.insert("bench-calf-raises", CalfExerciseProfile {
            plantar_flexion_range: (90.0, 150.0),
            knee_angle_range: (175.0, 180.0), // Nearly straight
            stability_threshold: 0.9,
            emphasis: CalfEmphasis::Gastrocnemius,
            strictness: 1.2,
        });

        // 2. Plate Raises
        calf_exercises.insert("plate-raises", CalfExerciseProfile {
            plantar_flexion_range: (100.0, 160.0),
            knee_angle_range: (160.0, 180.0),
            stability_threshold: 0.8,
            emphasis: CalfEmphasis::Both,
            strictness: 1.1,
        });

        // 3. Bulgarian Raises
        calf_exercises.insert("bulgarian-raises", CalfExerciseProfile {
            plantar_flexion_range: (80.0, 140.0),
            knee_angle_range: (120.0, 150.0), // Bent knee position
            stability_threshold: 0.7,
            emphasis: CalfEmphasis::Soleus,
            strictness: 1.4,
        });

        // 4. Barbell Raises
        calf_exercises.insert("barbell-raises", CalfExerciseProfile {
            plantar_flexion_range: (95.0, 155.0),
            knee_angle_range: (170.0, 180.0),
            stability_threshold: 0.85,
            emphasis: CalfEmphasis::Gastrocnemius,
            strictness: 1.3,
        });

        // 5. Jump Rope
        calf_exercises.insert("jump-rope", CalfExerciseProfile {
            plantar_flexion_range: (120.0, 170.0),
            knee_angle_range: (150.0, 180.0),
            stability_threshold: 0.6,
            emphasis: CalfEmphasis::Both,
            strictness: 0.9,
        });

        // 6. Donkey Calf Raises
        calf_exercises.insert("donkey-calf-raises", CalfExerciseProfile {
            plantar_flexion_range: (85.0, 145.0),
            knee_angle_range: (175.0, 180.0),
            stability_threshold: 0.95,
            emphasis: CalfEmphasis::Gastrocnemius,
            strictness: 1.5,
        });

        // 7. Seated Calf Raises
        calf_exercises.insert("seated-calf-raises", CalfExerciseProfile {
            plantar_flexion_range: (70.0, 130.0),
            knee_angle_range: (90.0, 100.0), // Fixed bent position
            stability_threshold: 1.0,
            emphasis: CalfEmphasis::Soleus,
            strictness: 1.6,
        });

        // 8. Stair Calf Raises
        calf_exercises.insert("stair-calf-raises", CalfExerciseProfile {
            plantar_flexion_range: (60.0, 150.0), // Extra range
            knee_angle_range: (175.0, 180.0),
            stability_threshold: 0.75,
            emphasis: CalfEmphasis::Both,
            strictness: 1.2,
        });

        // 9. Farmer Walk on Toes
        calf_exercises.insert("farmer-walk-on-toes", CalfExerciseProfile {
            plantar_flexion_range: (140.0, 170.0), // Maintained contraction
            knee_angle_range: (175.0, 180.0),
            stability_threshold: 0.5,
            emphasis: CalfEmphasis::Gastrocnemius,
            strictness: 1.3,
        });

        // 10. Pogo Jumps
        calf_exercises.insert("pogo-jumps", CalfExerciseProfile {
            plantar_flexion_range: (130.0, 180.0), // Explosive movement
            knee_angle_range: (160.0, 180.0),
            stability_threshold: 0.4,
            emphasis: CalfEmphasis::Both,
            strictness: 1.1,
        });

        PoseAnalyzer { calf_exercises }
    }

    pub fn calculate_calf_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 33 { // 11 keypoints (full body)
            // Ankle angle (knee-ankle-foot)
            angles.insert("plantar_flexion".to_string(), self.calculate_angle(
                &keypoints[25..28], // Left knee
                &keypoints[27..30], // Left ankle
                &keypoints[29..32]  // Left foot
            ));
            
            // Knee angle (hip-knee-ankle)
            angles.insert("knee_flexion".to_string(), self.calculate_angle(
                &keypoints[23..26], // Left hip
                &keypoints[25..28], // Left knee
                &keypoints[27..30]  // Left ankle
            ));
            
            // Stability metric (shoulder-hip-ankle)
            angles.insert("body_stability".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left shoulder
                &keypoints[23..26], // Left hip
                &keypoints[27..30]  // Left ankle
            ));
            
            // Range of motion tracking
            angles.insert("rom_achieved".to_string(), 
                angles["plantar_flexion"] - angles["knee_flexion"]
            );
        }
        
        angles
    }

    pub fn check_calf_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.calf_exercises.get(exercise_id) {
            // Check plantar flexion range
            if let Some(pf) = angles.get("plantar_flexion") {
                if *pf < profile.plantar_flexion_range.0 {
                    errors.push(format!(
                        "Insufficient plantar flexion ({}° < {}°)",
                        pf.round(), profile.plantar_flexion_range.0
                    ));
                }
                if *pf > profile.plantar_flexion_range.1 {
                    errors.push(format!(
                        "Over-extension ({}° > {}°)",
                        pf.round(), profile.plantar_flexion_range.1
                    ));
                }
            }
            
            // Check knee position
            if let Some(knee) = angles.get("knee_flexion") {
                match profile.emphasis {
                    CalfEmphasis::Gastrocnemius if *knee < 170.0 => {
                        errors.push("Keep knees straighter for gastrocnemius emphasis".to_string());
                    },
                    CalfEmphasis::Soleus if *knee > 110.0 => {
                        errors.push("Maintain proper knee bend for soleus emphasis".to_string());
                    },
                    _ => {}
                }
                
                if *knee < profile.knee_angle_range.0 {
                    errors.push(format!(
                        "Knees too bent ({}° < {}°)",
                        knee.round(), profile.knee_angle_range.0
                    ));
                }
                if *knee > profile.knee_angle_range.1 {
                    errors.push(format!(
                        "Knees too straight ({}° > {}°)",
                        knee.round(), profile.knee_angle_range.1
                    ));
                }
            }
            
            // Check body stability
            if let Some(stability) = angles.get("body_stability") {
                let deviation = (stability - 180.0).abs();
                if deviation > (1.0 - profile.stability_threshold) * 30.0 {
                    errors.push("Excessive body movement".to_string());
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "pogo-jumps" | "jump-rope" => {
                    if let Some(rom) = angles.get("rom_achieved") {
                        if *rom < 40.0 {
                            errors.push("Insufficient explosive range".to_string());
                        }
                    }
                },
                "seated-calf-raises" => {
                    if let Some(knee) = angles.get("knee_flexion") {
                        if (knee - 95.0).abs() > 5.0 {
                            errors.push("Maintain consistent knee angle".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_calf_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.calf_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Plantar flexion component (50% weight)
            if let Some(pf) = angles.get("plantar_flexion") {
                let pf_norm = (*pf - profile.plantar_flexion_range.0) / 
                             (profile.plantar_flexion_range.1 - profile.plantar_flexion_range.0);
                score += 0.5 * pf_norm.clamp(0.0, 1.0);
            }
            
            // Knee position component (30% weight)
            if let Some(knee) = angles.get("knee_flexion") {
                let knee_dev = match profile.emphasis {
                    CalfEmphasis::Gastrocnemius => 1.0 - (knee - 180.0).abs() / 30.0,
                    CalfEmphasis::Soleus => 1.0 - (knee - 90.0).abs() / 30.0,
                    CalfEmphasis::Both => 1.0 - (knee - 135.0).abs() / 45.0,
                };
                score += 0.3 * knee_dev.clamp(0.0, 1.0);
            }
            
            // Stability component (20% weight)
            if let Some(stability) = angles.get("body_stability") {
                let stab_dev = 1.0 - (stability - 180.0).abs() / 30.0;
                score += 0.2 * stab_dev.clamp(0.0, 1.0) * profile.stability_threshold;
            }
            
            (score * profile.strictness).clamp(0.1, 1.0)
        } else {
            0.5
        }
    }
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut biceps_exercises = HashMap::new();

        // 1. Isolated Dumbbell Curls
        biceps_exercises.insert("isolated-dumbbell-curls", BicepsExerciseProfile {
            curl_angle_range: (30.0, 160.0),
            shoulder_stabilization: 0.9,
            elbow_travel: (5.0, 15.0), // Minimal elbow movement
            strictness: 1.3,
            common_faults: vec!["Shoulder involvement", "Body English"],
        });

        // 2. Barbell Curls
        biceps_exercises.insert("barbell-curls", BicepsExerciseProfile {
            curl_angle_range: (45.0, 150.0),
            shoulder_stabilization: 0.7,
            elbow_travel: (10.0, 30.0),
            strictness: 1.2,
            common_faults: vec!["Elbow drift", "Wrist flexion"],
        });

        // 3. Dumbbell Curls
        biceps_exercises.insert("dumbbell-curls", BicepsExerciseProfile {
            curl_angle_range: (30.0, 160.0),
            shoulder_stabilization: 0.6,
            elbow_travel: (15.0, 40.0),
            strictness: 1.1,
            common_faults: vec!["Alternating unevenly", "Momentum use"],
        });

        // 4. Open-Grip Pull-ups
        biceps_exercises.insert("open-grip-pull-ups", BicepsExerciseProfile {
            curl_angle_range: (60.0, 180.0),
            shoulder_stabilization: 0.3,
            elbow_travel: (50.0, 100.0),
            strictness: 1.4,
            common_faults: vec!["Partial range", "Kipping"],
        });

        // 5. Lateral Push-ups
        biceps_exercises.insert("lateral-push-ups", BicepsExerciseProfile {
            curl_angle_range: (90.0, 150.0),
            shoulder_stabilization: 0.8,
            elbow_travel: (20.0, 60.0),
            strictness: 1.5,
            common_faults: vec!["Elbow flare", "Shoulder roll"],
        });

        // 6. Half-Rep Curls
        biceps_exercises.insert("half-rep-curls", BicepsExerciseProfile {
            curl_angle_range: (90.0, 120.0),
            shoulder_stabilization: 0.95,
            elbow_travel: (2.0, 10.0),
            strictness: 1.6,
            common_faults: vec!["Breaking form", "Overloading"],
        });

        // 7. Resistance Band Pulls
        biceps_exercises.insert("resistance-bands-pull", BicepsExerciseProfile {
            curl_angle_range: (45.0, 135.0),
            shoulder_stabilization: 0.5,
            elbow_travel: (30.0, 60.0),
            strictness: 1.0,
            common_faults: vec!["Inconsistent tension", "Body lean"],
        });

        // 8. Outward Dumbbell Curls
        biceps_exercises.insert("outward-dumbbell-curls", BicepsExerciseProfile {
            curl_angle_range: (40.0, 140.0),
            shoulder_stabilization: 0.7,
            elbow_travel: (10.0, 25.0),
            strictness: 1.3,
            common_faults: vec!["Wrist rotation", "Shoulder elevation"],
        });

        // 9. Concentration Curls
        biceps_exercises.insert("concentration-curls", BicepsExerciseProfile {
            curl_angle_range: (30.0, 150.0),
            shoulder_stabilization: 1.0,
            elbow_travel: (0.0, 5.0), // Elbow should stay fixed
            strictness: 1.7,
            common_faults: vec!["Elbow movement", "Body sway"],
        });

        // 10. Zottman Curls
        biceps_exercises.insert("zottman-curls", BicepsExerciseProfile {
            curl_angle_range: (45.0, 135.0),
            shoulder_stabilization: 0.8,
            elbow_travel: (5.0, 15.0),
            strictness: 1.4,
            common_faults: vec!["Grip inconsistency", "Tempo variation"],
        });

        PoseAnalyzer { biceps_exercises }
    }

    pub fn calculate_biceps_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 33 { // 11 keypoints (simplified upper body)
            // Elbow angle (shoulder-elbow-wrist)
            angles.insert("elbow_flexion".to_string(), self.calculate_angle(
                &keypoints[9..12],  // Left shoulder
                &keypoints[11..14], // Left elbow
                &keypoints[13..16]  // Left wrist
            ));
            
            // Shoulder stabilization (hip-shoulder-elbow)
            angles.insert("shoulder_stability".to_string(), self.calculate_angle(
                &keypoints[15..18], // Left hip
                &keypoints[9..12],  // Left shoulder
                &keypoints[11..14]  // Left elbow
            ));
            
            // Elbow travel (distance from starting position)
            angles.insert("elbow_travel".to_string(), self.calculate_distance(
                &keypoints[11..14], // Current elbow position
                &keypoints[17..20]  // Reference position (hip)
            ));
            
            // Wrist angle for Zottman curls
            angles.insert("wrist_pronation".to_string(), self.calculate_angle(
                &keypoints[11..14], // Elbow
                &keypoints[13..16], // Wrist
                &keypoints[19..22]  // Reference point
            ));
        }
        
        angles
    }

    pub fn check_biceps_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.biceps_exercises.get(exercise_id) {
            // Check curl range
            if let Some(curl_angle) = angles.get("elbow_flexion") {
                if *curl_angle < profile.curl_angle_range.0 {
                    errors.push(format!("Incomplete extension ({}° < {}°)", 
                        curl_angle.round(), profile.curl_angle_range.0));
                }
                if *curl_angle > profile.curl_angle_range.1 {
                    errors.push(format!("Over-flexion ({}° > {}°)", 
                        curl_angle.round(), profile.curl_angle_range.1));
                }
            }
            
            // Check shoulder stability
            if let Some(shoulder_angle) = angles.get("shoulder_stability") {
                let deviation = (shoulder_angle - 180.0).abs();
                if deviation > (1.0 - profile.shoulder_stabilization) * 30.0 {
                    errors.push("Excessive shoulder movement".to_string());
                }
            }
            
            // Check elbow travel
            if let Some(travel) = angles.get("elbow_travel") {
                if *travel < profile.elbow_travel.0 {
                    errors.push("Insufficient range of motion".to_string());
                }
                if *travel > profile.elbow_travel.1 {
                    errors.push("Excessive elbow drift".to_string());
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "zottman-curls" => {
                    if let Some(pronation) = angles.get("wrist_pronation") {
                        if pronation.abs() < 45.0 {
                            errors.push("Incomplete wrist rotation".to_string());
                        }
                    }
                },
                "concentration-curls" => {
                    if let Some(travel) = angles.get("elbow_travel") {
                        if *travel > 5.0 {
                            errors.push("Elbow should remain fixed".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_biceps_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.biceps_exercises.get(exercise_id) {
            let mut score = 0.0;
            
            // Curl range component (40% weight)
            if let Some(curl) = angles.get("elbow_flexion") {
                let curl_norm = (*curl - profile.curl_angle_range.0) / 
                               (profile.curl_angle_range.1 - profile.curl_angle_range.0);
                score += 0.4 * curl_norm.clamp(0.0, 1.0);
            }
            
            // Shoulder stability component (30% weight)
            if let Some(shoulder) = angles.get("shoulder_stability") {
                let shoulder_dev = 1.0 - ((shoulder - 180.0).abs() / 30.0).clamp(0.0, 1.0);
                score += 0.3 * shoulder_dev * profile.shoulder_stabilization;
            }
            
            // Elbow travel component (20% weight)
            if let Some(travel) = angles.get("elbow_travel") {
                let travel_norm = 1.0 - ((*travel - profile.elbow_travel.0) / 
                                        (profile.elbow_travel.1 - profile.elbow_travel.0)).clamp(0.0, 1.0);
                score += 0.2 * travel_norm;
            }
            
            // Exercise-specific components (10% weight)
            score += match exercise_id {
                "zottman-curls" => {
                    angles.get("wrist_pronation")
                        .map_or(0.0, |p| 0.1 * (p.abs() / 90.0).clamp(0.0, 1.0))
                },
                "concentration-curls" => {
                    angles.get("elbow_travel")
                        .map_or(0.0, |t| 0.1 * (1.0 - (t / 5.0).clamp(0.0, 1.0)))
                },
                _ => 0.1
            };
            
            (score * profile.strictness).clamp(0.1, 1.0)
        } else {
            0.5
        }
    }

    fn calculate_distance(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() < 2 || b.len() < 2 {
            return 0.0;
        }
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }

    // Existing calculate_angle method...
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut back_exercises = HashMap::new();

        // 1. Dumbbell Rows
        back_exercises.insert("dumbbell-rows", BackExerciseProfile {
            primary_muscles: vec!["lats", "rhomboids", "rear-delts"],
            joint_ranges: HashMap::from([
                ("shoulder_flexion", (30.0, 60.0)),
                ("elbow", (75.0, 90.0)),
                ("torso", (15.0, 30.0)), // Torso angle from horizontal
            ]),
            strictness_factor: 1.2,
            common_mistakes: vec!["Using momentum", "Shrugging shoulders"],
        });

        // 2. Barbell Rows
        back_exercises.insert("barbell-rows", BackExerciseProfile {
            primary_muscles: vec!["mid-traps", "lats", "erectors"],
            joint_ranges: HashMap::from([
                ("shoulder_extension", (45.0, 75.0)),
                ("elbow", (90.0, 120.0)),
                ("torso", (30.0, 45.0)),
            ]),
            strictness_factor: 1.3,
            common_mistakes: vec!["Rounding lower back", "Partial range"],
        });

        // 3. Seated Dumbbell Rows
        back_exercises.insert("seated-dumbbell-rows", BackExerciseProfile {
            primary_muscles: vec!["lower-lats", "biceps"],
            joint_ranges: HashMap::from([
                ("shoulder_extension", (60.0, 90.0)),
                ("elbow", (90.0, 120.0)),
                ("torso", (0.0, 15.0)), // More upright
            ]),
            strictness_factor: 1.1,
            common_mistakes: vec!["Overextending shoulders", "Using legs"],
        });

        // 4. Chin-up/Pull-up
        back_exercises.insert("chin-up-pull-ups", BackExerciseProfile {
            primary_muscles: vec!["upper-lats", "biceps"],
            joint_ranges: HashMap::from([
                ("shoulder_flexion", (180.0, 210.0)), // Full range
                ("elbow", (30.0, 180.0)), // Wide range
            ]),
            strictness_factor: 1.4,
            common_mistakes: vec!["Partial reps", "Kipping"],
        });

        // 5. Deadlifts
        back_exercises.insert("deadlifts", BackExerciseProfile {
            primary_muscles: vec!["erectors", "glutes", "hamstrings"],
            joint_ranges: HashMap::from([
                ("hip", (60.0, 90.0)), // Starting position
                ("knee", (90.0, 120.0)),
                ("torso", (45.0, 60.0)),
            ]),
            strictness_factor: 1.5,
            common_mistakes: vec!["Rounded back", "Hips rising first"],
        });

        // 6. Pull-ups
        back_exercises.insert("pull-ups", BackExerciseProfile {
            primary_muscles: vec!["lats", "teres-major"],
            joint_ranges: HashMap::from([
                ("shoulder_adduction", (170.0, 220.0)),
                ("elbow", (30.0, 180.0)),
            ]),
            strictness_factor: 1.4,
            common_mistakes: vec!["Incomplete extension", "Elbow flaring"],
        });

        // 7. Face Pulls
        back_exercises.insert("face-pulls", BackExerciseProfile {
            primary_muscles: vec!["rear-delts", "rotator-cuff"],
            joint_ranges: HashMap::from([
                ("shoulder_horizontal", (90.0, 120.0)),
                ("elbow", (90.0, 120.0)),
            ]),
            strictness_factor: 1.0,
            common_mistakes: vec!["Using too much weight", "Shrugging"],
        });

        // 8. T-bar Rows
        back_exercises.insert("t-bar-rows", BackExerciseProfile {
            primary_muscles: vec!["mid-back", "lats"],
            joint_ranges: HashMap::from([
                ("shoulder_extension", (60.0, 90.0)),
                ("torso_rotation", (0.0, 15.0)), // Minimal rotation
            ]),
            strictness_factor: 1.2,
            common_mistakes: vec!["Twisting torso", "Partial contraction"],
        });

        // 9. Open Butterfly (Rear Delt Fly)
        back_exercises.insert("open-butterfly", BackExerciseProfile {
            primary_muscles: vec!["rear-delts", "traps"],
            joint_ranges: HashMap::from([
                ("shoulder_horizontal", (120.0, 150.0)),
                ("elbow", (10.0, 30.0)), // Slight bend
            ]),
            strictness_factor: 0.9,
            common_mistakes: vec!["Using arms instead of back", "Overextending"],
        });

        // 10. Lateral Russian Roulette
        back_exercises.insert("lateral-russian-roulette", BackExerciseProfile {
            primary_muscles: vec!["obliques", "erectors"],
            joint_ranges: HashMap::from([
                ("torso_rotation", (45.0, 60.0)),
                ("hip", (90.0, 120.0)),
            ]),
            strictness_factor: 1.1,
            common_mistakes: vec!["Using arms only", "Over-rotating"],
        });

        PoseAnalyzer { back_exercises }
    }

    pub fn calculate_back_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        if keypoints.len() >= 51 { // 17 keypoints
            // Shoulder angles
            angles.insert("shoulder_flexion".to_string(), self.calculate_angle(
                &keypoints[11..14], // Left hip
                &keypoints[5..8],    // Left shoulder
                &keypoints[7..10]    // Left elbow
            ));
            
            angles.insert("shoulder_extension".to_string(), 180.0 - angles["shoulder_flexion"]);
            
            angles.insert("shoulder_horizontal".to_string(), self.calculate_angle(
                &keypoints[5..8],    // Left shoulder
                &keypoints[11..14],  // Left hip
                &keypoints[6..9]     // Right shoulder
            ));
            
            // Elbow angle
            angles.insert("elbow".to_string(), self.calculate_angle(
                &keypoints[5..8],    // Left shoulder
                &keypoints[7..10],   // Left elbow
                &keypoints[9..12]    // Left wrist
            ));
            
            // Torso angles
            angles.insert("torso".to_string(), self.calculate_angle(
                &keypoints[11..14],  // Left hip
                &keypoints[0..3],    // Nose
                &keypoints[23..26]   // Left knee
            ));
            
            angles.insert("torso_rotation".to_string(), self.calculate_angle(
                &keypoints[5..8],    // Left shoulder
                &keypoints[0..3],    // Nose
                &keypoints[6..9]     // Right shoulder
            ));
            
            // Hip angle
            angles.insert("hip".to_string(), self.calculate_angle(
                &keypoints[5..8],    // Left shoulder
                &keypoints[11..14],  // Left hip
                &keypoints[13..16]   // Left knee
            ));
        }
        
        angles
    }

    pub fn check_back_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.back_exercises.get(exercise_id) {
            // Check joint ranges
            for (joint, &(min, max)) in &profile.joint_ranges {
                if let Some(angle) = angles.get(*joint) {
                    if *angle < min {
                        errors.push(format!("{} too closed ({}° < {}°)", joint, angle.round(), min));
                    } else if *angle > max {
                        errors.push(format!("{} too open ({}° > {}°)", joint, angle.round(), max));
                    }
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "deadlifts" => {
                    if let (Some(hip), Some(knee)) = (angles.get("hip"), angles.get("knee")) {
                        if (hip - knee).abs() > 30.0 {
                            errors.push("Hip-knee synchronization off".to_string());
                        }
                    }
                },
                "pull-ups" => {
                    if let Some(shoulder) = angles.get("shoulder_adduction") {
                        if *shoulder < 170.0 {
                            errors.push("Incomplete shoulder extension".to_string());
                        }
                    }
                },
                "face-pulls" => {
                    if let Some(elbow) = angles.get("elbow") {
                        if *elbow > 120.0 {
                            errors.push("Elbow angle too wide - focus on rear delts".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_back_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        if let Some(profile) = self.back_exercises.get(exercise_id) {
            let mut score = 0.0;
            let mut valid_angles = 0;
            
            for (joint, &(min, max)) in &profile.joint_ranges {
                if let Some(angle) = angles.get(*joint) {
                    let normalized = (angle - min) / (max - min);
                    score += normalized.clamp(0.0, 1.0);
                    valid_angles += 1;
                }
            }
            
            if valid_angles > 0 {
                (score / valid_angles as f32) * profile.strictness_factor
            } else {
                0.5
            }
        } else {
            0.5
        }.clamp(0.1, 1.0)
    }

    // Existing calculate_angle method...
}

impl PoseAnalyzer {
    pub fn new() -> Self {
        let mut abs_exercises = HashMap::new();

        // 1. Jack Knife
        abs_exercises.insert("jack-knife", AbExerciseProfile {
            primary_joints: vec!["hip", "shoulder"],
            target_angles: HashMap::from([
                ("hip", (45.0, 90.0)),
                ("shoulder", (30.0, 60.0)),
            ]),
            common_errors: vec!["Overarching lower back", "Using momentum"],
            engagement_factor: 1.2,
        });

        // 2. Hanging Leg Raises
        abs_exercises.insert("hanging-leg-raises", AbExerciseProfile {
            primary_joints: vec!["hip", "shoulder"],
            target_angles: HashMap::from([
                ("hip", (70.0, 120.0)),
                ("shoulder", (160.0, 180.0)), // Should maintain near-full extension
            ]),
            common_errors: vec!["Swinging body", "Partial range of motion"],
            engagement_factor: 1.5,
        });

        // 3. Russian Twist
        abs_exercises.insert("russian-roulette", AbExerciseProfile { // Note: Using your ID "russian-roulette"
            primary_joints: vec!["torso_rotation", "hip"],
            target_angles: HashMap::from([
                ("torso_rotation", (45.0, 60.0)), // Rotation angle
                ("hip", (100.0, 130.0)), // Hip flexion
            ]),
            common_errors: vec!["Rotating from arms only", "Rounding shoulders"],
            engagement_factor: 1.3,
        });

        // 4. Ab Wheel Rollout
        abs_exercises.insert("ab-wheel-rollout", AbExerciseProfile {
            primary_joints: vec!["shoulder", "hip"],
            target_angles: HashMap::from([
                ("shoulder", (150.0, 180.0)),
                ("hip", (170.0, 180.0)), // Near full extension at peak
            ]),
            common_errors: vec!["Dropping hips", "Overextending lower back"],
            engagement_factor: 1.4,
        });

        // 5. Reverse Crunch
        abs_exercises.insert("reverse-crunch", AbExerciseProfile {
            primary_joints: vec!["hip", "knee"],
            target_angles: HashMap::from([
                ("hip", (90.0, 120.0)),
                ("knee", (90.0, 120.0)),
            ]),
            common_errors: vec!["Using hip flexors only", "Neck strain"],
            engagement_factor: 1.1,
        });

        // 6. Scissor Kicks
        abs_exercises.insert("scissor-kicks", AbExerciseProfile {
            primary_joints: vec!["hip", "leg_angle"],
            target_angles: HashMap::from([
                ("hip", (150.0, 180.0)), // Near-flat on ground
                ("leg_angle", (30.0, 60.0)), // Scissor angle
            ]),
            common_errors: vec!["Arching back", "Moving too quickly"],
            engagement_factor: 1.0,
        });

        // 7. Plank Hip Dips
        abs_exercises.insert("plank-hip-dips", AbExerciseProfile {
            primary_joints: vec!["shoulder", "hip_lateral"],
            target_angles: HashMap::from([
                ("shoulder", (170.0, 180.0)), // Stable shoulders
                ("hip_lateral", (10.0, 20.0)), // Lateral flexion range
            ]),
            common_errors: vec!["Sagging hips", "Over-rotating"],
            engagement_factor: 1.2,
        });

        // 8. Back Arch (for core stability)
        abs_exercises.insert("back_arch", AbExerciseProfile {
            primary_joints: vec!["spine_extension", "shoulder"],
            target_angles: HashMap::from([
                ("spine_extension", (15.0, 30.0)), // Controlled extension
                ("shoulder", (160.0, 180.0)),
            ]),
            common_errors: vec!["Over-arching", "Neck strain"],
            engagement_factor: 0.9,
        });

        // 9. Lateral Leg Raises
        abs_exercises.insert("lateral-leg-raises", AbExerciseProfile {
            primary_joints: vec!["hip_abduction", "torso_lateral"],
            target_angles: HashMap::from([
                ("hip_abduction", (30.0, 60.0)),
                ("torso_lateral", (0.0, 10.0)), // Minimal torso lean
            ]),
            common_errors: vec!["Using momentum", "Leaning torso"],
            engagement_factor: 1.1,
        });

        // 10. Dumbbell Leg Raises
        abs_exercises.insert("dumbbell-leg-raises", AbExerciseProfile {
            primary_joints: vec!["hip", "shoulder"],
            target_angles: HashMap::from([
                ("hip", (60.0, 90.0)),
                ("shoulder", (150.0, 180.0)), // Stable shoulders
            ]),
            common_errors: vec!["Swinging weights", "Partial range"],
            engagement_factor: 1.3,
        });

        PoseAnalyzer { abs_exercises }
    }

    pub fn calculate_abs_angles(&self, keypoints: &[f32]) -> HashMap<String, f32> {
        let mut angles = HashMap::new();
        
        // Core angle calculations (using 17-keypoint COCO model format)
        if keypoints.len() >= 51 { // 17 points * 3 values
            // Hip angle (nose - hip - knee)
            angles.insert("hip".to_string(), self.calculate_angle(
                &keypoints[0..3],   // nose
                &keypoints[11..14], // left hip
                &keypoints[13..16]  // left knee
            ));
            
            // Shoulder stability (hip - shoulder - elbow)
            angles.insert("shoulder".to_string(), self.calculate_angle(
                &keypoints[11..14], // left hip
                &keypoints[5..8],   // left shoulder
                &keypoints[7..10]   // left elbow
            ));
            
            // Torso rotation (left shoulder - nose - right shoulder)
            angles.insert("torso_rotation".to_string(), self.calculate_angle(
                &keypoints[5..8],   // left shoulder
                &keypoints[0..3],   // nose
                &keypoints[6..9]    // right shoulder
            ));
            
            // Lateral flexion (left hip - nose - right hip)
            angles.insert("torso_lateral".to_string(), self.calculate_angle(
                &keypoints[11..14], // left hip
                &keypoints[0..3],   // nose
                &keypoints[12..15]  // right hip
            ));
            
            // Hip abduction (for lateral movements)
            angles.insert("hip_abduction".to_string(), self.calculate_angle(
                &keypoints[11..14], // left hip
                &keypoints[13..16], // left knee
                &keypoints[12..15]  // right hip
            ));
        }
        
        angles
    }

    pub fn check_abs_form(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(profile) = self.abs_exercises.get(exercise_id) {
            // Check target angles
            for (joint, &(min, max)) in &profile.target_angles {
                if let Some(angle) = angles.get(*joint) {
                    if *angle < min {
                        errors.push(format!("{} angle too small ({}° < {}°)", joint, angle.round(), min));
                    } else if *angle > max {
                        errors.push(format!("{} angle too large ({}° > {}°)", joint, angle.round(), max));
                    }
                }
            }
            
            // Exercise-specific checks
            match exercise_id {
                "hanging-leg-raises" => {
                    if let Some(shoulder_angle) = angles.get("shoulder") {
                        if *shoulder_angle < 160.0 {
                            errors.push("Maintain straight arm position".to_string());
                        }
                    }
                },
                "plank-hip-dips" => {
                    if let Some(hip_lateral) = angles.get("hip_lateral") {
                        if *hip_lateral > 25.0 {
                            errors.push("Control lateral movement - reduce range".to_string());
                        }
                    }
                },
                _ => {}
            }
        }
        
        errors
    }

    pub fn calculate_abs_engagement(&self, exercise_id: &str, angles: &HashMap<String, f32>) -> f32 {
        let base_engagement = if let Some(profile) = self.abs_exercises.get(exercise_id) {
            // Calculate based on primary joints
            let mut total = 0.0;
            for joint in &profile.primary_joints {
                if let Some(angle) = angles.get(*joint) {
                    // Normalize angle to 0-1 range based on target
                    if let Some((min, max)) = profile.target_angles.get(*joint) {
                        let normalized = (angle - min) / (max - min);
                        total += normalized.clamp(0.0, 1.0);
                    }
                }
            }
            (total / profile.primary_joints.len() as f32) * profile.engagement_factor
        } else {
            0.5 // Default for unknown exercises
        };
        
        base_engagement.clamp(0.1, 1.0)
    }

    fn calculate_angle(&self, a: &[f32], b: &[f32], c: &[f32]) -> f32 {
        if a.len() < 3 || b.len() < 3 || c.len() < 3 || a[2] < 0.1 || b[2] < 0.1 || c[2] < 0.1 {
            return 0.0;
        }

        let ab = (b[0] - a[0], b[1] - a[1]);
        let cb = (b[0] - c[0], b[1] - c[1]);
        
        let dot = ab.0 * cb.0 + ab.1 * cb.1;
        let cross = ab.0 * cb.1 - ab.1 * cb.0;
        
        let angle = cross.atan2(dot).to_degrees().abs();
        
        if angle > 180.0 { 360.0 - angle } else { angle }
    }
}