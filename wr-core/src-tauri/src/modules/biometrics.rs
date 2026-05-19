use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BiometricData {
    pub heart_rate: u32,
    pub blink_rate: u32,
    pub focus_score: f32,
}

pub struct BiometricsMonitor;

impl BiometricsMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn process_frame(&self, frame_data: &[u8]) -> BiometricData {
        // Placeholder for MediaPipe / OpenCV integration
        BiometricData {
            heart_rate: 70,
            blink_rate: 12,
            focus_score: 0.95,
        }
    }
}
