use wasm_bindgen::prelude::*;
use web_sys::console;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Debug => "DEBUG",
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn init() {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    }

    pub fn log(level: LogLevel, module: &str, message: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        let log_message = format!("[{}] {} - {}: {}", 
            timestamp,
            level.as_str(),
            module,
            message
        );

        match level {
            LogLevel::Info => console::info_1(&log_message.into()),
            LogLevel::Warn => console::warn_1(&log_message.into()),
            LogLevel::Error => console::error_1(&log_message.into()),
            LogLevel::Debug => console::debug_1(&log_message.into()),
        }
    }

    pub fn info(module: &str, message: &str) {
        Self::log(LogLevel::Info, module, message);
    }

    pub fn warn(module: &str, message: &str) {
        Self::log(LogLevel::Warn, module, message);
    }

    pub fn error(module: &str, message: &str) {
        Self::log(LogLevel::Error, module, message);
    }

    pub fn debug(module: &str, message: &str) {
        Self::log(LogLevel::Debug, module, message);
    }
}

// Performance monitoring
pub struct PerformanceMonitor {
    start_time: f64,
    label: String,
}

impl PerformanceMonitor {
    pub fn start(label: &str) -> Self {
        let window = web_sys::window().expect("Should have window");
        let performance = window.performance().expect("Should have performance");
        
        Self {
            start_time: performance.now(),
            label: label.to_string(),
        }
    }

    pub fn end(self) {
        let window = web_sys::window().expect("Should have window");
        let performance = window.performance().expect("Should have performance");
        let end_time = performance.now();
        let duration = end_time - self.start_time;
        
        Logger::info("Performance", &format!("{}: {:.2}ms", self.label, duration));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_logger_initialization() {
        Logger::init();
        Logger::info("test", "Logger initialized");
    }

    #[wasm_bindgen_test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::start("test_operation");
        // Simulate some work
        for _ in 0..1000 {
            let _ = 1 + 1;
        }
        monitor.end();
    }
}
