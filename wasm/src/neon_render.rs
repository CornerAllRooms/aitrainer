// wasm/src/neon_render.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NeonRenderer {
    color: [f32; 3],
    intensity: f32,
}

#[wasm_bindgen]
impl NeonRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        NeonRenderer {
            color: [r, g, b],
            intensity: 1.0,
        }
    }

    #[wasm_bindgen]
    pub fn render(&self, keypoints: &[f32]) -> Vec<f32> {
        keypoints.chunks_exact(3)
            .flat_map(|kp| {
                [kp[0], kp[1], self.color[0], self.color[1], self.color[2], self.intensity * kp[2]]
            })
            .collect()
    }

    #[wasm_bindgen]
    pub fn set_intensity(&mut self, engagement: f32) {
        self.intensity = engagement.clamp(0.3, 1.0);
    }
}
