//! Luna4 state management
//!
//! Tracks the operational state of a Luna4 planet including
//! lunar phase timing, explorer presence, and statistics.
//! This is internal state that should not be exposed to external users.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::cycle::LunarPhase;
use super::Luna4Id;

/// Tracks the operational state of a Luna4 planet
///
/// This struct maintains all runtime state for a Luna4 instance,
/// including timing information, explorer presence, and generation history.
/// It is used internally by the Luna4 AI implementation.
#[derive(Debug)]
pub struct Luna4State {
    /// Planet identifier
    pub(crate) id: Luna4Id,
    /// Current lunar phase
    pub(crate) current_phase: LunarPhase,
    /// When the current phase started
    pub(crate) phase_start_time: Instant,
    /// Resource generation history
    pub(crate) generation_history: Vec<(LunarPhase, common_game::components::resource::BasicResourceType)>,
    /// Explorer presence tracking
    pub(crate) present_explorers: HashMap<u32, Instant>,
    /// Operational statistics
    pub(crate) stats: OperationalStats,
}

impl Luna4State {
    /// Creates a new Luna4 state
    ///
    /// # Arguments
    /// * `id` - Planet identifier
    ///
    /// # Returns
    /// New `Luna4State` instance initialized to New Moon phase
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id: Luna4Id::new(id),
            current_phase: LunarPhase::NewMoon,
            phase_start_time: Instant::now(),
            generation_history: Vec::new(),
            present_explorers: HashMap::new(),
            stats: OperationalStats::new(),
        }
    }
    
    /// Updates the lunar phase based on elapsed time
    ///
    /// Checks if enough time has passed to transition to the next phase.
    /// If a transition occurs, updates statistics and returns the previous phase.
    ///
    /// # Returns
    /// `Some(previous_phase)` if a phase transition occurred, `None` otherwise
    pub(crate) fn update_phase(&mut self) -> Option<LunarPhase> {
        use super::cycle::LunarCycle;
        
        let cycle = LunarCycle::default();
        let elapsed = self.phase_start_time.elapsed();
        
        if elapsed >= cycle.phase_duration(self.current_phase) {
            let old_phase = self.current_phase;
            self.current_phase = cycle.next_phase(self.current_phase);
            self.phase_start_time = Instant::now();
            self.stats.phase_transitions += 1;
            Some(old_phase)
        } else {
            None
        }
    }
    
    /// Records a resource generation event
    ///
    /// # Arguments
    /// * `resource` - Type of resource that was generated
    pub(crate) fn record_generation(
        &mut self,
        resource: common_game::components::resource::BasicResourceType,
    ) {
        self.generation_history.push((self.current_phase, resource));
        self.stats.total_resources_generated += 1;
    }
    
    /// Registers an explorer arrival
    ///
    /// # Arguments
    /// * `explorer_id` - Identifier of arriving explorer
    pub(crate) fn register_explorer_arrival(&mut self, explorer_id: u32) {
        self.present_explorers.insert(explorer_id, Instant::now());
        self.stats.explorer_arrivals += 1;
    }
    
    /// Registers an explorer departure
    ///
    /// # Arguments
    /// * `explorer_id` - Identifier of departing explorer
    pub(crate) fn register_explorer_departure(&mut self, explorer_id: u32) {
        self.present_explorers.remove(&explorer_id);
        self.stats.explorer_departures += 1;
    }
    
    /// Gets the number of present explorers
    ///
    /// # Returns
    /// Count of explorers currently on the planet
    pub(crate) fn explorer_count(&self) -> usize {
        self.present_explorers.len()
    }
    
    /// Gets elapsed time in current phase
    ///
    /// # Returns
    /// `Duration` since current phase started
    pub(crate) fn elapsed_in_current_phase(&self) -> Duration {
        self.phase_start_time.elapsed()
    }
    
    /// Creates a display-friendly summary of the state
    ///
    /// # Returns
    /// Formatted string summarizing current state
    pub(crate) fn display_summary(&self) -> String {
        let phase_progress = self.phase_start_time.elapsed();
        let explorers = self.explorer_count();
        
        format!(
            "Luna4 #{id} | Phase: {phase:?} | Time in phase: {time:.1}s | Explorers: {explorers} | Generated: {gen}",
            id = self.id.as_u32(),
            phase = self.current_phase,
            time = phase_progress.as_secs_f32(),
            explorers = explorers,
            gen = self.stats.total_resources_generated
        )
    }
}

/// Operational statistics for Luna4
///
/// Tracks various metrics about Luna4's operation for monitoring
/// and debugging purposes.
#[derive(Debug, Clone, Default)]
pub(crate) struct OperationalStats {
    /// Total resources generated
    pub(crate) total_resources_generated: usize,
    /// Phase transitions observed
    pub(crate) phase_transitions: usize,
    /// Explorer arrivals
    pub(crate) explorer_arrivals: usize,
    /// Explorer departures
    pub(crate) explorer_departures: usize,
    /// Errors encountered
    pub(crate) errors_encountered: usize,
}

impl OperationalStats {
    /// Creates new operational statistics
    ///
    /// # Returns
    /// New `OperationalStats` instance with all counts zero
    pub(crate) fn new() -> Self {
        Self::default()
    }
    
    /// Records an error occurrence
    pub(crate) fn record_error(&mut self) {
        self.errors_encountered += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_initialization() {
        let state = Luna4State::new(42);
        
        assert_eq!(state.id.as_u32(), 42);
        assert_eq!(state.current_phase, LunarPhase::NewMoon);
        assert_eq!(state.explorer_count(), 0);
        assert_eq!(state.stats.total_resources_generated, 0);
    }
    
    #[test]
    fn test_explorer_tracking() {
        let mut state = Luna4State::new(1);
        
        state.register_explorer_arrival(100);
        assert_eq!(state.explorer_count(), 1);
        assert_eq!(state.stats.explorer_arrivals, 1);
        
        state.register_explorer_arrival(200);
        assert_eq!(state.explorer_count(), 2);
        
        state.register_explorer_departure(100);
        assert_eq!(state.explorer_count(), 1);
        assert_eq!(state.stats.explorer_departures, 1);
    }
    
    #[test]
    fn test_resource_generation_tracking() {
        let mut state = Luna4State::new(1);
        
        state.record_generation(common_game::components::resource::BasicResourceType::Oxygen);
        assert_eq!(state.stats.total_resources_generated, 1);
        assert_eq!(state.generation_history.len(), 1);
        
        state.record_generation(common_game::components::resource::BasicResourceType::Hydrogen);
        assert_eq!(state.stats.total_resources_generated, 2);
    }
    
    #[test]
    fn test_operational_stats_error_tracking() {
        let mut stats = OperationalStats::new();
        
        assert_eq!(stats.errors_encountered, 0);
        
        stats.record_error();
        assert_eq!(stats.errors_encountered, 1);
        
        stats.record_error();
        stats.record_error();
        assert_eq!(stats.errors_encountered, 3);
    }
    
    #[test]
    fn test_display_summary_format() {
        let state = Luna4State::new(99);
        let summary = state.display_summary();
        
        assert!(summary.contains("Luna4 #99"));
        assert!(summary.contains("Phase: NewMoon"));
        assert!(summary.contains("Explorers: 0"));
        assert!(summary.contains("Generated: 0"));
    }
}