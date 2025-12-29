//! Statistics tracking for Luna4
//!
//! This module tracks various operational metrics for Luna4 planets
//! that are used for monitoring and debugging purposes.

/// Statistics tracking for Luna4 operations
///
/// This struct aggregates various metrics about Luna4's operation
/// that can be used for monitoring, debugging, or performance analysis.

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Luna4Stats {
    /// Number of successful resource generations
    pub(crate) successful_generations: usize,
    /// Number of failed resource generations
    pub(crate) failed_generations: usize,
    /// Number of sunrays received
    pub(crate) sunrays_received: usize,
    /// Number of explorer messages processed
    pub(crate) explorer_messages_processed: usize,
    /// Total processing time for operations (in microseconds)
    pub(crate) total_processing_time_us: u64,
}

impl Luna4Stats {
    /// Creates new statistics with all counts zero
    ///
    /// # Returns
    /// New `Luna4Stats` instance
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self::default()
    }
    
    /// Records a successful resource generation
    #[allow(dead_code)]
    pub(crate) fn record_successful_generation(&mut self) {
        self.successful_generations += 1;
    }
    
    /// Records a failed resource generation
    #[allow(dead_code)]
    pub(crate) fn record_failed_generation(&mut self) {
        self.failed_generations += 1;
    }
    
    /// Records reception of a sunray
    #[allow(dead_code)]
    pub(crate) fn record_sunray_received(&mut self) {
        self.sunrays_received += 1;
    }
    
    /// Records processing of an explorer message
    #[allow(dead_code)]
    pub(crate) fn record_explorer_message_processed(&mut self) {
        self.explorer_messages_processed += 1;
    }
    
    /// Adds processing time to the total
    ///
    /// # Arguments
    /// * `microseconds` - Processing time to add in microseconds
    #[allow(dead_code)]
    pub(crate) fn add_processing_time(&mut self, microseconds: u64) {
        self.total_processing_time_us += microseconds;
    }
    
    /// Calculates the success rate for resource generation
    ///
    /// # Returns
    /// Success rate as a percentage (0.0 to 100.0), or 0.0 if no attempts
    #[allow(dead_code)]
    pub(crate) fn generation_success_rate(&self) -> f32 {
        let total_attempts = self.successful_generations + self.failed_generations;
        if total_attempts == 0 {
            return 0.0;
        }
        (self.successful_generations as f32 / total_attempts as f32) * 100.0
    }
    
    /// Creates a display-friendly summary of the statistics
    ///
    /// # Returns
    /// Formatted string summarizing the statistics
    #[allow(dead_code)]
    pub(crate) fn display_summary(&self) -> String {
        format!(
            "Stats: {} successful / {} failed generations ({:.1}% success), {} sunrays, {} explorer messages",
            self.successful_generations,
            self.failed_generations,
            self.generation_success_rate(),
            self.sunrays_received,
            self.explorer_messages_processed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stats_initialization() {
        let stats = Luna4Stats::new();
        
        assert_eq!(stats.successful_generations, 0);
        assert_eq!(stats.failed_generations, 0);
        assert_eq!(stats.sunrays_received, 0);
        assert_eq!(stats.explorer_messages_processed, 0);
        assert_eq!(stats.total_processing_time_us, 0);
    }
    
    #[test]
    fn test_stat_recording() {
        let mut stats = Luna4Stats::new();
        
        stats.record_successful_generation();
        stats.record_successful_generation();
        stats.record_failed_generation();
        stats.record_sunray_received();
        stats.record_explorer_message_processed();
        stats.add_processing_time(150);
        
        assert_eq!(stats.successful_generations, 2);
        assert_eq!(stats.failed_generations, 1);
        assert_eq!(stats.sunrays_received, 1);
        assert_eq!(stats.explorer_messages_processed, 1);
        assert_eq!(stats.total_processing_time_us, 150);
    }
    
    #[test]
    fn test_success_rate_calculation() {
        let mut stats = Luna4Stats::new();
        
        // No attempts
        assert!((stats.generation_success_rate() - 0.0).abs() < 0.001);
        
        // Some successes, no failures
        stats.record_successful_generation();
        stats.record_successful_generation();
        assert!((stats.generation_success_rate() - 100.0).abs() < 0.001);
        
        // Mixed results
        stats.record_failed_generation();
        stats.record_failed_generation();
        // 2 successes, 2 failures = 50% success rate
        assert!((stats.generation_success_rate() - 50.0).abs() < 0.001);
    }
    
    #[test]
    fn test_display_summary() {
        let mut stats = Luna4Stats::new();
        
        stats.record_successful_generation();
        stats.record_failed_generation();
        stats.record_sunray_received();
        stats.record_explorer_message_processed();
        
        let summary = stats.display_summary();
        
        assert!(summary.contains("1 successful"));
        assert!(summary.contains("1 failed"));
        assert!(summary.contains("50.0% success"));
        assert!(summary.contains("1 sunrays"));
        assert!(summary.contains("1 explorer messages"));
    }
}