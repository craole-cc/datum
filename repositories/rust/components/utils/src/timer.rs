//! Comprehensive timing and profiling utilities for workspace projects.
//!
//! This crate provides hierarchical timing, structured logging integration,
//! and flexible output formats suitable for development, CI/CD, and production use.

use std::{
  collections::HashMap,
  fmt,
  sync::{Arc, Mutex},
  time::{Duration, Instant},
};

// Re-export tracing for convenience
pub use tracing::{debug, error, info, trace, warn};

/// Configuration for timing behavior across the workspace
#[derive(Debug, Clone)]
pub struct TimingConfig {
  /// Whether to print timing info immediately
  pub print_immediately: bool,
  /// Whether to print the summary at the end of execution
  pub print_summary: bool,
  /// Whether to collect timing data for summaries
  pub collect_data: bool,
  /// Minimum duration to print (filters out very fast operations)
  pub min_duration: Duration,
  /// Output format preference
  pub format: OutputFormat,
  /// Whether to use structured logging (tracing) or println
  pub use_tracing: bool,
  /// Log level for tracing output
  pub log_level: LogLevel,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
  /// Simple: "[task] took 123ms"
  Simple,
  /// Hierarchical: ">>-domain-|-scope-|-task-<< took 123ms"
  Hierarchical,
  /// JSON-like structured output
  Structured,
  /// Table format for summaries
  Table,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

impl Default for TimingConfig {
  fn default() -> Self {
    Self {
      print_immediately: true,
      print_summary: false,
      collect_data: true,
      min_duration: Duration::from_micros(100), // Only show operations > 100μs
      format: OutputFormat::Hierarchical,
      use_tracing: true,
      log_level: LogLevel::Info,
    }
  }
}

/// Global timing configuration - can be set once for the entire workspace
static GLOBAL_CONFIG: std::sync::OnceLock<TimingConfig> =
  std::sync::OnceLock::new();

/// Default configuration instance
const DEFAULT_CONFIG: TimingConfig = TimingConfig {
  print_immediately: true,
  print_summary: false,
  collect_data: true,
  min_duration: Duration::from_micros(100),
  format: OutputFormat::Hierarchical,
  use_tracing: true,
  log_level: LogLevel::Info,
};

/// Set global timing configuration for all scopes
pub fn configure_timing(config: TimingConfig) {
  let _ = GLOBAL_CONFIG.set(config);
}

/// Get the current timing configuration
pub fn get_config() -> &'static TimingConfig {
  GLOBAL_CONFIG.get().unwrap_or(&DEFAULT_CONFIG)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskResult {
  pub name: String,
  #[serde(with = "duration_serde")]
  pub duration: Duration,
  #[serde(skip, default = "Instant::now")]
  // Instant can't be serialized, use current time as default
  pub timestamp: Instant,
  pub metadata: HashMap<String, String>,
}

// Helper module for Duration serialization
mod duration_serde {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};
  use std::time::Duration;

  pub fn serialize<S>(
    duration: &Duration,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    duration.as_nanos().serialize(serializer)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
  where
    D: Deserializer<'de>,
  {
    let nanos = u128::deserialize(deserializer)?;
    Ok(Duration::from_nanos(nanos as u64))
  }
}

impl TaskResult {
  pub fn new(name: String, duration: Duration) -> Self {
    Self {
      name,
      duration,
      timestamp: Instant::now(),
      metadata: HashMap::new(),
    }
  }

  pub fn with_metadata(
    mut self,
    key: impl Into<String>,
    value: impl Into<String>,
  ) -> Self {
    self.metadata.insert(key.into(), value.into());
    self
  }
}

/// A hierarchical timing scope that can contain multiple tasks
#[derive(Debug)]
pub struct Scope {
  pub domain: String,
  pub name: String,
  pub description: Option<String>,

  // Timing data
  tasks: Vec<TaskResult>,
  start_time: Instant,
  end_time: Option<Instant>,

  // Hierarchy support
  parent: Option<String>,
  children: Vec<String>,

  // Configuration override
  config_override: Option<TimingConfig>,

  // Thread safety for shared scopes
  shared_data: Option<Arc<Mutex<Vec<TaskResult>>>>,
}

impl Scope {
  /// Create a new timing scope
  pub fn new(domain: impl Into<String>, name: impl Into<String>) -> Self {
    Self {
      domain: domain.into(),
      name: name.into(),
      description: None,
      tasks: Vec::new(),
      start_time: Instant::now(),
      end_time: None,
      parent: None,
      children: Vec::new(),
      config_override: None,
      shared_data: None,
    }
  }

  /// Add a description to this scope
  pub fn with_description(mut self, desc: impl Into<String>) -> Self {
    self.description = Some(desc.into());
    self
  }

  /// Override global config for this scope
  pub fn with_config(mut self, config: TimingConfig) -> Self {
    self.config_override = Some(config);
    self
  }

  /// Make this scope thread-safe for sharing across threads
  pub fn make_shared(mut self) -> Self {
    self.shared_data = Some(Arc::new(Mutex::new(Vec::new())));
    self
  }

  /// Create a child scope
  pub fn child(&self, name: impl Into<String>) -> Self {
    let child_name = name.into();
    let parent_id = format!("{}::{}", self.domain, self.name);

    Self {
      domain: self.domain.clone(),
      name: child_name,
      description: None,
      tasks: Vec::new(),
      start_time: Instant::now(),
      end_time: None,
      parent: Some(parent_id),
      children: Vec::new(),
      config_override: self.config_override.clone(),
      shared_data: self.shared_data.clone(),
    }
  }

  /// Get the effective configuration (override or global)
  fn config(&self) -> &TimingConfig {
    self
      .config_override
      .as_ref()
      .unwrap_or_else(|| get_config())
  }

  /// Time a synchronous task
  pub fn time_task<F, R>(&mut self, task_name: impl Into<String>, f: F) -> R
  where
    F: FnOnce() -> R,
  {
    let task_name = task_name.into();
    let start = Instant::now();

    let result = f();
    let duration = start.elapsed();

    self.record_task(task_name, duration);
    result
  }

  /// Time an async task
  pub async fn time_async_task<T, E, F, Fut>(
    &mut self,
    task_name: impl Into<String>,
    f: F,
  ) -> Result<T, E>
  where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, E>>,
  {
    let task_name = task_name.into();
    let start = Instant::now();

    // Run the future, capturing its Result<T, E>
    let outcome = f().await;
    let duration = start.elapsed();

    // Record timing regardless of success or failure
    self.record_task(task_name, duration);
    outcome
  }
  // pub async fn time_async_task<F, Fut, R>(
  //   &mut self,
  //   task_name: impl Into<String>,
  //   f: F,
  // ) -> R
  // where
  //   F: FnOnce() -> Fut,
  //   Fut: std::future::Future<Output = R>,
  // {
  //   let task_name = task_name.into();
  //   let start = Instant::now();

  //   let result = f().await;
  //   let duration = start.elapsed();

  //   self.record_task(task_name, duration);
  //   result
  // }

  /// Manually record a task duration
  pub fn record_task(
    &mut self,
    task_name: impl Into<String>,
    duration: Duration,
  ) {
    let config = self.config();

    // Skip if below minimum duration
    if duration < config.min_duration {
      return;
    }

    let task_result = TaskResult::new(task_name.into(), duration);

    // Print immediately if configured
    if config.print_immediately {
      self.print_task(&task_result);
    }

    // Store if collecting data
    if config.collect_data || config.print_summary {
      if let Some(shared) = &self.shared_data {
        if let Ok(mut data) = shared.lock() {
          data.push(task_result);
        }
      } else {
        self.tasks.push(task_result);
      }
    }
  }

  /// Print a single task result
  fn print_task(&self, task: &TaskResult) {
    let config = self.config();
    let message = self.format_task_message(task);

    if config.use_tracing {
      match config.log_level {
        LogLevel::Trace => trace!("{}", message),
        LogLevel::Debug => debug!("{}", message),
        LogLevel::Info => info!("{}", message),
        LogLevel::Warn => warn!("{}", message),
        LogLevel::Error => error!("{}", message),
      }
    } else {
      println!("{message}");
    }
  }

  /// Format a task message according to the configured format
  fn format_task_message(&self, task: &TaskResult) -> String {
    let config = self.config();

    match config.format {
      OutputFormat::Simple => {
        format!("[{}] took {}", task.name, format_duration(task.duration))
      }
      OutputFormat::Hierarchical => {
        format!(
          "[>>-{}-|-{}-|-{}-<<] took {}",
          self.domain,
          self.name,
          task.name,
          format_duration(task.duration)
        )
      }
      OutputFormat::Structured => {
        format!(
          "{{\"domain\":\"{}\",\"scope\":\"{}\",\"task\":\"{}\",\"duration_ms\":{},\"duration_str\":\"{}\"}}",
          self.domain,
          self.name,
          task.name,
          task.duration.as_millis(),
          format_duration(task.duration)
        )
      }
      OutputFormat::Table => {
        format!(
          "{:20} | {:15} | {:15} | {}",
          self.domain,
          self.name,
          task.name,
          format_duration(task.duration)
        )
      }
    }
  }

  /// Get all tasks (including from shared data)
  pub fn tasks(&self) -> Vec<TaskResult> {
    if let Some(shared) = &self.shared_data {
      if let Ok(data) = shared.lock() {
        let mut all_tasks = self.tasks.clone();
        all_tasks.extend(data.clone());
        all_tasks
      } else {
        self.tasks.clone()
      }
    } else {
      self.tasks.clone()
    }
  }

  /// Mark scope as completed
  pub fn finish(&mut self) {
    if self.end_time.is_none() {
      self.end_time = Some(Instant::now());
    }
  }

  /// Get total duration of all tasks
  pub fn total_task_duration(&self) -> Duration {
    self.tasks().iter().map(|t| t.duration).sum()
  }

  /// Get scope's total wall-clock time
  pub fn scope_duration(&self) -> Duration {
    self
      .end_time
      .unwrap_or_else(Instant::now)
      .duration_since(self.start_time)
  }

  /// Print comprehensive summary
  pub fn print_summary(&mut self) {
    self.finish();

    let config = self.config();
    if !config.collect_data {
      return;
    }

    let tasks = self.tasks();
    if tasks.is_empty() {
      return;
    }

    match config.format {
      OutputFormat::Table => self.print_table_summary(&tasks),
      OutputFormat::Structured => self.print_json_summary(&tasks),
      _ => self.print_standard_summary(&tasks),
    }
  }

  fn print_standard_summary(&self, tasks: &[TaskResult]) {
    let title = if let Some(desc) = &self.description {
      format!("{} :: {} ({})", self.domain, self.name, desc)
    } else {
      format!("{} :: {}", self.domain, self.name)
    };

    println!("\n=== {title} Summary ===");

    for task in tasks {
      println!("  {:20} : {}", task.name, format_duration(task.duration));
      for (key, value) in &task.metadata {
        println!("    {key} = {value}");
      }
    }

    println!("  {:-<42}", "");
    println!(
      "  {:20} : {}",
      "Task Total",
      format_duration(self.total_task_duration())
    );
    println!(
      "  {:20} : {}",
      "Scope Total",
      format_duration(self.scope_duration())
    );

    let overhead = self
      .scope_duration()
      .saturating_sub(self.total_task_duration());
    if overhead > Duration::from_micros(100) {
      println!("  {:20} : {}", "Overhead", format_duration(overhead));
    }

    println!("{}===={}====\n", "=".repeat(title.len()), "=".repeat(10));
  }

  fn print_table_summary(&self, tasks: &[TaskResult]) {
    println!("\n┌{:─<20}┬{:─<15}┬{:─<15}┬{:─<12}┐", "", "", "", "");
    println!(
      "│{:^20}│{:^15}│{:^15}│{:^12}│",
      "Domain", "Scope", "Task", "Duration"
    );
    println!("├{:─<20}┼{:─<15}┼{:─<15}┼{:─<12}┤", "", "", "", "");

    for task in tasks {
      println!(
        "│{:20}│{:15}│{:15}│{:>12}│",
        self.domain,
        self.name,
        task.name,
        format_duration(task.duration)
      );
    }

    println!("├{:─<20}┼{:─<15}┼{:─<15}┼{:─<12}┤", "", "", "", "");
    println!(
      "│{:20}│{:15}│{:15}│{:>12}│",
      "",
      "",
      "TOTAL",
      format_duration(self.total_task_duration())
    );
    println!("└{:─<20}┴{:─<15}┴{:─<15}┴{:─<12}┘\n", "", "", "", "");
  }

  fn print_json_summary(&self, tasks: &[TaskResult]) {
    let summary = serde_json::json!({
        "domain": self.domain,
        "scope": self.name,
        "description": self.description,
        "tasks": tasks.iter().map(|t| serde_json::json!({
            "name": t.name,
            "duration_ms": t.duration.as_millis(),
            "duration_str": format_duration(t.duration),
            "metadata": t.metadata
        })).collect::<Vec<_>>(),
        "totals": {
            "task_duration_ms": self.total_task_duration().as_millis(),
            "scope_duration_ms": self.scope_duration().as_millis(),
            "task_count": tasks.len()
        }
    });

    println!(
      "{}",
      serde_json::to_string_pretty(&summary)
        .unwrap_or_else(|_| "{}".to_string())
    );
  }

  /// Export timing data as JSON
  pub fn export_json(&self) -> Result<String, Box<dyn std::error::Error>> {
    let tasks = self.tasks();
    let data = serde_json::json!({
        "domain": self.domain,
        "scope": self.name,
        "description": self.description,
        "start_time_nanos": self.start_time.elapsed().as_nanos(),
        "tasks": tasks.iter().map(|t| serde_json::json!({
            "name": t.name,
            "duration_nanos": t.duration.as_nanos(),
            "duration_ms": t.duration.as_millis(),
            "duration_str": format_duration(t.duration),
            "metadata": t.metadata
        })).collect::<Vec<_>>(),
        "totals": {
            "task_duration_nanos": self.total_task_duration().as_nanos(),
            "scope_duration_nanos": self.scope_duration().as_nanos(),
            "task_duration_ms": self.total_task_duration().as_millis(),
            "scope_duration_ms": self.scope_duration().as_millis()
        }
    });

    Ok(serde_json::to_string_pretty(&data)?)
  }
}

// Implement Drop to auto-print summary
impl Drop for Scope {
  fn drop(&mut self) {
    let cfg = get_config();

    // only auto-print end-of-scope summary if configured
    if cfg.collect_data && cfg.print_summary && self.end_time.is_none() {
      self.print_summary();
    }
  }
}

// impl Drop for Scope {
//   fn drop(&mut self) {
//     if get_config().print_immediately && get_config().collect_data {
//       // Only print summary if we haven't manually called it
//       if self.end_time.is_none() {
//         self.print_summary();
//       }
//     }
//   }
// }

/// Convenience macro for timing code blocks
#[macro_export]
macro_rules! time_block {
  ($scope:expr, $task_name:expr, $code:block) => {{ $scope.time_task($task_name, || $code) }};
}

/// Convenience macro for creating and timing in one go
#[macro_export]
macro_rules! timed_scope {
  ($domain:expr, $name:expr, $block:block) => {{
    let mut scope = $crate::Scope::new($domain, $name);
    let result = $block;
    // result
    scope.print_summary()
  }};
}

/// Convenience macro for creating and timing without returning a value
#[macro_export]
macro_rules! timed_scope_void {
  ($domain:expr, $name:expr, $block:block) => {{
    let mut scope = $crate::Scope::new($domain, $name);
    let _result = $block;
    scope.print_summary();
  }};
}

// Your existing timer utilities
#[derive(Debug, Clone, Copy)]
pub struct Timer {
  start: Instant,
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      start: Instant::now(),
    }
  }
}

impl Timer {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn elapsed(&self) -> Duration {
    Instant::now()
      .checked_duration_since(self.start)
      .unwrap_or_default()
  }

  pub fn elapsed_str(&self) -> String {
    format_duration(self.elapsed())
  }
}

#[derive(Debug)]
pub struct ScopedTimer {
  label: String,
  start: Instant,
  config: TimingConfig,
}

impl ScopedTimer {
  pub fn new(label: impl Into<String>) -> Self {
    Self {
      label: label.into(),
      start: Instant::now(),
      config: get_config().clone(),
    }
  }

  pub fn with_config(label: impl Into<String>, config: TimingConfig) -> Self {
    Self {
      label: label.into(),
      start: Instant::now(),
      config,
    }
  }
}

impl Drop for ScopedTimer {
  fn drop(&mut self) {
    let elapsed = Instant::now().duration_since(self.start);

    if elapsed >= self.config.min_duration {
      let message =
        format!("[{}] took {}", self.label, format_duration(elapsed));

      if self.config.use_tracing {
        match self.config.log_level {
          LogLevel::Info => info!("{}", message),
          LogLevel::Debug => debug!("{}", message),
          LogLevel::Trace => trace!("{}", message),
          LogLevel::Warn => warn!("{}", message),
          LogLevel::Error => error!("{}", message),
        }
      } else {
        println!("{message}");
      }
    }
  }
}

/// Enhanced duration formatting with more precision options
pub fn format_duration(d: Duration) -> String {
  let nanos = d.as_nanos();

  if nanos < 1_000 {
    format!("{nanos}ns")
  } else if nanos < 1_000_000 {
    format!("{:.1}μs", nanos as f64 / 1_000.0)
  } else if nanos < 1_000_000_000 {
    format!("{:.1}ms", nanos as f64 / 1_000_000.0)
  } else {
    let secs = d.as_secs();
    let millis = d.subsec_millis();

    if secs >= 60 {
      let mins = secs / 60;
      let rem_secs = secs % 60;
      format!("{mins}m {rem_secs}s")
    } else if secs > 0 {
      format!("{secs}.{millis:03}s")
    } else {
      format!("{millis}ms")
    }
  }
}

/// Helper for timing functions
pub fn time_function<F, R>(name: &str, f: F) -> R
where
  F: FnOnce() -> R,
{
  let _timer = ScopedTimer::new(name);
  f()
}

// Additional dependencies you might want to add to Cargo.toml:
/*
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["time"], optional = true }

[features]
default = []
async = ["tokio"]
json = ["serde", "serde_json"]
tracing = ["tracing", "tracing-subscriber"]
*/
