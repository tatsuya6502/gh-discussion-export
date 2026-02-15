use std::io::{IsTerminal, Write, stdout};

/// Progress reporter for long-running operations.
///
/// Provides terminal-aware progress reporting:
/// - TTY: inline updates with carriage return
/// - Non-TTY: line-by-line progress
#[derive(Debug)]
pub struct ProgressReporter {
    /// Total number of items to process
    total: usize,
    /// Current progress count
    current: usize,
    /// Whether stdout is a terminal
    is_tty: bool,
    /// Label for the progress message
    label: String,
}

impl ProgressReporter {
    /// Create a new progress reporter.
    ///
    /// # Arguments
    /// * `total` - Total number of items to process
    /// * `label` - Label for the progress message (e.g., "Fetching comments")
    ///
    /// # Example
    /// ```no_run
    /// use gh_discussion_export::progress::ProgressReporter;
    ///
    /// let reporter = ProgressReporter::new(100, "Downloading assets");
    /// reporter.start();
    /// ```
    pub fn new(total: usize, label: impl Into<String>) -> Self {
        Self {
            total,
            current: 0,
            is_tty: stdout().is_terminal(),
            label: label.into(),
        }
    }

    /// Start progress reporting (displays initial message).
    pub fn start(&self) {
        if self.total == 0 {
            println!("{}: 0/0 (100%)", self.label);
            return;
        }

        let message = self.format_progress();
        self.print_message(&message);
    }

    /// Increment progress by one and update display.
    pub fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
            self.update();
        }
    }

    /// Set current progress and update display.
    pub fn set_progress(&mut self, current: usize) {
        if current <= self.total {
            self.current = current;
            self.update();
        }
    }

    /// Update progress display.
    fn update(&self) {
        let message = self.format_progress();
        self.print_message(&message);
    }

    /// Complete progress reporting (displays final message with newline).
    pub fn complete(&self) {
        let message = self.format_progress();
        self.print_message(&message);

        // Always add final newline for clean output
        if self.is_tty {
            println!();
        }
    }

    /// Format progress message with count and percentage.
    ///
    /// # Arguments
    /// * `current` - Current progress count
    /// * `total` - Total number of items
    ///
    /// # Example
    /// ```
    /// use gh_discussion_export::progress::ProgressReporter;
    ///
    /// let message = ProgressReporter::format_progress_message(5, 10);
    /// assert_eq!(message, "5/10 (50%)");
    /// ```
    pub fn format_progress_message(current: usize, total: usize) -> String {
        if total == 0 {
            return "0/0 (100%)".to_string();
        }

        let percent = (current * 100) / total;
        format!("{}/{} ({}%)", current, total, percent)
    }

    /// Format current progress message.
    fn format_progress(&self) -> String {
        let progress = Self::format_progress_message(self.current, self.total);
        format!("{}: {}", self.label, progress)
    }

    /// Print message with appropriate formatting for TTY/non-TTY.
    fn print_message(&self, message: &str) {
        if self.is_tty {
            // TTY: use carriage return to overwrite current line
            print!("\r{}", message);
            let _ = stdout().flush();
        } else {
            // Non-TTY: print each progress on a new line
            println!("{}", message);
        }
    }

    /// Check if stdout is a terminal.
    ///
    /// # Example
    /// ```
    /// use gh_discussion_export::progress::ProgressReporter;
    ///
    /// let is_tty = ProgressReporter::is_terminal();
    /// ```
    pub fn is_terminal() -> bool {
        stdout().is_terminal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_progress_message_zero() {
        let message = ProgressReporter::format_progress_message(0, 0);
        assert_eq!(message, "0/0 (100%)");
    }

    #[test]
    fn test_format_progress_message_start() {
        let message = ProgressReporter::format_progress_message(0, 10);
        assert_eq!(message, "0/10 (0%)");
    }

    #[test]
    fn test_format_progress_message_half() {
        let message = ProgressReporter::format_progress_message(5, 10);
        assert_eq!(message, "5/10 (50%)");
    }

    #[test]
    fn test_format_progress_message_complete() {
        let message = ProgressReporter::format_progress_message(10, 10);
        assert_eq!(message, "10/10 (100%)");
    }

    #[test]
    fn test_format_progress_message_rounding() {
        // Test rounding behavior
        let message = ProgressReporter::format_progress_message(1, 3);
        assert_eq!(message, "1/3 (33%)"); // Integer division: 33%

        let message = ProgressReporter::format_progress_message(2, 3);
        assert_eq!(message, "2/3 (66%)"); // Integer division: 66%
    }

    #[test]
    fn test_format_progress_message_large_numbers() {
        let message = ProgressReporter::format_progress_message(999, 1000);
        assert_eq!(message, "999/1000 (99%)");
    }

    #[test]
    fn test_reporter_creation() {
        let reporter = ProgressReporter::new(100, "Test");
        assert_eq!(reporter.total, 100);
        assert_eq!(reporter.current, 0);
        assert_eq!(reporter.label, "Test");
    }

    #[test]
    fn test_reporter_increment() {
        let mut reporter = ProgressReporter::new(10, "Test");
        assert_eq!(reporter.current, 0);

        reporter.increment();
        assert_eq!(reporter.current, 1);

        reporter.increment();
        assert_eq!(reporter.current, 2);
    }

    #[test]
    fn test_reporter_set_progress() {
        let mut reporter = ProgressReporter::new(10, "Test");
        reporter.set_progress(5);
        assert_eq!(reporter.current, 5);

        // Set to same value should work
        reporter.set_progress(5);
        assert_eq!(reporter.current, 5);

        // Set beyond total should not change
        reporter.set_progress(15);
        assert_eq!(reporter.current, 5); // Should stay at 5
    }

    #[test]
    fn test_reporter_increment_stops_at_total() {
        let mut reporter = ProgressReporter::new(5, "Test");

        for _ in 0..10 {
            reporter.increment();
        }

        assert_eq!(reporter.current, 5); // Should not exceed total
    }

    #[test]
    fn test_format_progress_includes_label() {
        let reporter = ProgressReporter::new(10, "Downloading");
        let message = reporter.format_progress();
        assert_eq!(message, "Downloading: 0/10 (0%)");
    }

    #[test]
    fn test_format_progress_updates_with_current() {
        let mut reporter = ProgressReporter::new(10, "Downloading");
        reporter.set_progress(5);

        let message = reporter.format_progress();
        assert_eq!(message, "Downloading: 5/10 (50%)");
    }

    #[test]
    fn test_is_terminal_returns_bool() {
        // This will return false in test environment (non-TTY)
        // but we just verify it returns a boolean without panicking
        let is_tty = ProgressReporter::is_terminal();
        let _ = is_tty; // Suppress unused warning
    }
}
