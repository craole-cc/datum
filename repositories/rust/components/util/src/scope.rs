use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TaskResult {
  pub name: String,
  pub duration: Duration,
}

#[derive(Debug)]
pub struct Scope {
  pub domain: String,
  pub name: String,
  pub task: Option<String>,
  measure: bool,
  duration: Duration,
  // New fields for tracking
  tasks: Vec<TaskResult>,
  start_time: Option<Instant>,
}

impl Default for Scope {
  fn default() -> Self {
    Self {
      domain: String::new(),
      name: String::new(),
      task: Some(String::from("Execution")),
      measure: true,
      duration: Duration::from_secs(0),
      tasks: Vec::new(),
      start_time: None,
    }
  }
}

impl Scope {
  pub fn new(domain: impl Into<String>, name: impl Into<String>) -> Self {
    Self {
      domain: domain.into(),
      name: name.into(),
      start_time: Some(Instant::now()),
      ..Default::default()
    }
  }

  pub fn with_task(mut self, task: impl Into<String>) -> Self {
    self.task = Some(task.into());
    self
  }

  pub fn without_timer(mut self) -> Self {
    self.measure = false;
    self
  }

  /// Time a task and immediately print the result
  pub fn time_task<F, R>(&mut self, task_name: impl Into<String>, f: F) -> R
  where
    F: FnOnce() -> R,
  {
    let task_name = task_name.into();
    let start = Instant::now();

    let result = f();

    let duration = start.elapsed();

    if self.measure {
      // Print immediately
      println!(
        "[>>-{}-|-{}-|-{}-<<] took {}",
        self.domain,
        self.name,
        task_name,
        format_duration(duration)
      );

      // Store for later
      self.tasks.push(TaskResult {
        name: task_name,
        duration,
      });
    }

    result
  }

  /// Get all completed tasks
  pub fn tasks(&self) -> &[TaskResult] {
    &self.tasks
  }

  /// Get total duration of all tasks
  pub fn total_task_duration(&self) -> Duration {
    self.tasks.iter().map(|t| t.duration).sum()
  }

  /// Print a summary of all tasks
  pub fn print_summary(&self) {
    if !self.measure || self.tasks.is_empty() {
      return;
    }

    println!("\n=== {} :: {} Summary ===", self.domain, self.name);
    for task in &self.tasks {
      println!("  {} : {}", task.name, format_duration(task.duration));
    }
    println!("  Total: {}", format_duration(self.total_task_duration()));

    if let Some(start) = self.start_time {
      let total_scope_time = start.elapsed();
      println!("  Scope Total: {}", format_duration(total_scope_time));
    }
    println!("========================\n");
  }

  pub fn init(&self) -> Option<ScopedTimer> {
    let domain = &self.domain;
    let scope = &self.name;

    if !&self.measure {
      return None;
    };

    if let Some(task) = self.task.as_ref() {
      Some(ScopedTimer::new(format!(
        ">>-{domain}-|-{scope}-|-{task}-<<"
      )))
    } else {
      Some(ScopedTimer::new(format!(">>-{domain}-|-{scope}-<<")))
    }
  }
}

// Your existing timer code
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
}

impl ScopedTimer {
  pub fn new(label: impl Into<String>) -> Self {
    Self {
      label: label.into(),
      start: Instant::now(),
    }
  }
}

impl Drop for ScopedTimer {
  fn drop(&mut self) {
    let elapsed = Instant::now().duration_since(self.start);
    println!("[{}] took {}", self.label, format_duration(elapsed));
  }
}

pub fn format_duration(d: Duration) -> String {
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

// Usage example
fn example_usage() {
  let mut scope = Scope::new("database", "user_operations");

  // Time individual tasks - prints immediately
  let users = scope.time_task("load_users", || {
    std::thread::sleep(std::time::Duration::from_millis(100));
    vec!["alice", "bob"]
  });

  let _processed = scope.time_task("process_users", || {
    std::thread::sleep(std::time::Duration::from_millis(50));
    users.len()
  });

  scope.time_task("save_results", || {
    std::thread::sleep(std::time::Duration::from_millis(75));
  });

  // Print final summary
  scope.print_summary();
}
