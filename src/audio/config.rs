use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioEngineConfig {
    pub sample_rate: u32,
    pub buffer_size: u32,
}

impl Default for AudioEngineConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            buffer_size: 128,
        }
    }
}

impl AudioEngineConfig {
    pub fn latency_ms(&self) -> f32 {
        (self.buffer_size as f32 / self.sample_rate as f32) * 1000.0
    }
}
