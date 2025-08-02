use crate::{
  LogLevel, OutputFormat, Scope, TimingConfig, configure_timing, time_block,
  timed_scope,
};
use std::time::Duration;
use tracing::info;

fn main() {
  // Configure timing globally for the entire workspace
  configure_timing(TimingConfig {
    print_immediately: true,
    collect_data: true,
    min_duration: Duration::from_micros(10), // Show operations > 10μs
    format: OutputFormat::Hierarchical,
    use_tracing: false,
    log_level: LogLevel::Info,
  });

  // Example 1: Database operations
  database_example();

  // Example 2: Web service operations
  web_service_example();

  // Example 3: Data processing pipeline
  data_processing_example();

  // Example 4: Multi-threaded operations
  multithreaded_example();

  // Example 5: Async operations (if you have async feature)
  // tokio::runtime::Runtime::new().unwrap().block_on(async_example());
}

fn database_example() {
  let mut scope = Scope::new("database", "user_operations")
    .with_description("Complete user lifecycle operations");

  // Simulate database operations
  let users = scope.time_task("load_users", || {
    std::thread::sleep(Duration::from_millis(50));
    vec!["alice", "bob", "charlie"]
  });

  let processed_count = scope.time_task("validate_users", || {
    std::thread::sleep(Duration::from_millis(25));
    users.len()
  });

  scope.time_task("update_cache", || {
    std::thread::sleep(Duration::from_millis(15));
  });

  // Record external timing
  scope.record_task("external_api_call", Duration::from_millis(100));

  // Child scope for nested operations
  let mut child_scope = scope.child("user_permissions");
  child_scope.time_task("load_permissions", || {
    std::thread::sleep(Duration::from_millis(30));
  });
  child_scope.print_summary();

  println!("Processed {processed_count} users");
  // Summary will be printed automatically on drop
}

fn web_service_example() {
  // Override config for this specific scope
  let mut scope = Scope::new("web", "request_handler")
    .with_description("HTTP request processing")
    .with_config(TimingConfig {
      format: OutputFormat::Table,
      min_duration: Duration::from_micros(1),
      ..TimingConfig::default()
    });

  scope.time_task("parse_request", || {
    std::thread::sleep(Duration::from_millis(5));
  });

  scope.time_task("authenticate", || {
    std::thread::sleep(Duration::from_millis(20));
  });

  scope.time_task("business_logic", || {
    std::thread::sleep(Duration::from_millis(35));
  });

  scope.time_task("serialize_response", || {
    std::thread::sleep(Duration::from_millis(8));
  });

  // Export data for monitoring systems
  if let Ok(json_data) = scope.export_json() {
    println!("Exported timing data: {json_data}");
  }
}

fn data_processing_example() {
  // Using macros for convenience
  timed_scope!("data", "etl_pipeline", {
    let mut pipeline = Scope::new("data", "etl_pipeline");

    time_block!(pipeline, "extract", {
      std::thread::sleep(Duration::from_millis(100));
      "raw_data"
    });

    time_block!(pipeline, "transform", {
      std::thread::sleep(Duration::from_millis(200));
      "transformed_data"
    });

    time_block!(pipeline, "load", {
      std::thread::sleep(Duration::from_millis(75));
    });

    "pipeline_complete"
  })
}

fn multithreaded_example() {
  use std::sync::Arc;
  use std::thread;

  let shared_scope = Arc::new(std::sync::Mutex::new(
    Scope::new("concurrent", "worker_pool")
      .with_description("Multi-threaded data processing")
      .make_shared(),
  ));

  let handles: Vec<_> = (0..4)
    .map(|i| {
      let scope = shared_scope.clone();
      thread::spawn(move || {
        // Simulate work
        std::thread::sleep(Duration::from_millis(50 + i * 10));

        // Record timing from this thread
        if let Ok(mut s) = scope.lock() {
          s.record_task(
            format!("worker_{i}"),
            Duration::from_millis(50 + i * 10),
          );
        }
      })
    })
    .collect();

  // Wait for all threads
  for handle in handles {
    handle.join().unwrap();
  }

  // Print summary
  if let Ok(mut scope) = shared_scope.lock() {
    scope.print_summary();
  }
}

// Example async operations (requires tokio feature)
async fn async_example() {
  let mut scope = Scope::new("async", "http_client")
    .with_description("Async HTTP operations");

  // Time async operations
  let response = scope
    .time_async_task("fetch_data", || async {
      tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
      "response_data"
    })
    .await;

  scope
    .time_async_task("process_response", || async {
      tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
      response.len()
    })
    .await;

  scope.print_summary();
}

// Example of integration with tracing
fn tracing_example() {
  use tracing_subscriber;

  // Initialize tracing
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  // Configure to use tracing instead of println
  configure_timing(TimingConfig {
    use_tracing: true,
    log_level: LogLevel::Info,
    ..TimingConfig::default()
  });

  let mut scope = Scope::new("tracing", "example");

  scope.time_task("traced_operation", || {
    std::thread::sleep(Duration::from_millis(25));
    info!("Operation completed");
  });
}

// Example CI/CD integration
fn ci_cd_example() {
  // In CI environments, you might want JSON output
  configure_timing(TimingConfig {
    format: OutputFormat::Structured,
    print_immediately: false, // Only print summaries
    collect_data: true,
    ..TimingConfig::default()
  });

  let mut build_scope = Scope::new("ci", "build_process");

  build_scope.time_task("compile", || {
    std::thread::sleep(Duration::from_millis(200));
  });

  build_scope.time_task("test", || {
    std::thread::sleep(Duration::from_millis(150));
  });

  build_scope.time_task("package", || {
    std::thread::sleep(Duration::from_millis(50));
  });

  // Export for CI metrics
  if let Ok(json) = build_scope.export_json() {
    // In real CI, you'd send this to your metrics system
    println!("BUILD_METRICS={json}");
  }
}

// Benchmark-style usage
fn benchmark_example() {
  configure_timing(TimingConfig {
    format: OutputFormat::Table,
    min_duration: Duration::from_nanos(1), // Show everything
    ..TimingConfig::default()
  });

  let mut bench_scope = Scope::new("benchmark", "algorithm_comparison");

  // Compare different algorithms
  for algorithm in &["quicksort", "mergesort", "heapsort"] {
    bench_scope.time_task(format!("sort_1000_{algorithm}"), || {
      // Simulate sorting
      let sleep_time = match *algorithm {
        "quicksort" => 20,
        "mergesort" => 25,
        "heapsort" => 30,
        _ => 50,
      };
      std::thread::sleep(Duration::from_millis(sleep_time));
    });
  }

  bench_scope.print_summary();
}

// Production monitoring example
fn production_monitoring_example() {
  // In production, you might want minimal overhead
  configure_timing(TimingConfig {
    print_immediately: false,
    collect_data: true,
    min_duration: Duration::from_millis(1), // Only significant operations
    format: OutputFormat::Structured,
    use_tracing: true,
    log_level: LogLevel::Debug,
  });

  let mut request_scope = Scope::new("prod", "api_request");

  // These would be real operations in production
  request_scope.time_task("auth_check", || {
    std::thread::sleep(Duration::from_millis(10));
  });

  request_scope.time_task("db_query", || {
    std::thread::sleep(Duration::from_millis(45));
  });

  request_scope.time_task("render_response", || {
    std::thread::sleep(Duration::from_millis(15));
  });

  // Only log if request took too long
  if request_scope.scope_duration() > Duration::from_millis(100) {
    request_scope.print_summary();
  }
}
