use crate::*;

#[derive(Default, Debug, Clone)]
pub enum ProcessingMode {
  #[default]
  SingleFrame,
  ChunkedSequential,
  ChunkedConcurrent,
  StreamingLowMemory,
}

impl Config {
  pub fn with_mode(mut self, mode: ProcessingMode) -> Self {
    self.mode = mode;
    self
  }

  pub fn with_mode_frame(mut self) -> Self {
    self.mode = ProcessingMode::SingleFrame;
    self
  }
  pub fn with_mode_sequential(mut self) -> Self {
    self.mode = ProcessingMode::ChunkedSequential;
    self
  }
  pub fn with_mode_concurrent(mut self) -> Self {
    self.mode = ProcessingMode::ChunkedConcurrent;
    self
  }
  pub fn with_mode_low_memory(mut self) -> Self {
    self.mode = ProcessingMode::StreamingLowMemory;
    self
  }
}
