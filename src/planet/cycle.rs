//! Lunar cycle and phase management for Luna4
//!
//! Implements the 7-minute lunar cycle with 4 distinct phases
//! that determine resource availability according to Luna4 lore.

use std::time::Duration;

/// Lunar phases that determine resource availability on Luna4
///
/// Luna4 follows a predictable 7-minute cycle divided into four equal phases.
/// Each phase enables different sets of basic resources according to the lore:
/// - **New Moon**: Rare elements in the dark
/// - **First Quarter**: Common ones in the light  
/// - **Full Moon**: Everything available
/// - **Last Quarter**: Preparation phase with limited resources
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LunarPhase {
    /// New Moon phase - rare elements only (Silicon, Carbon)
    NewMoon,
    /// First Quarter phase - common elements only (Oxygen, Hydrogen)
    FirstQuarter,
    /// Full Moon phase - all basic resources available
    FullMoon,
    /// Last Quarter phase - preparation phase, limited resources
    LastQuarter,
}

impl LunarPhase {
    /// Returns the human-readable name of this lunar phase
    ///
    /// # Returns
    /// String slice containing the phase name
    pub(crate) fn name(&self) -> &'static str {
        match self {
            LunarPhase::NewMoon => "New Moon",
            LunarPhase::FirstQuarter => "First Quarter",
            LunarPhase::FullMoon => "Full Moon",
            LunarPhase::LastQuarter => "Last Quarter",
        }
    }
    
    /// Returns the lore description of what this phase represents
    ///
    /// # Returns
    /// String slice with the phase description

    #[allow(dead_code)]
    pub(crate) fn description(&self) -> &'static str {
        match self {
            LunarPhase::NewMoon => "Rare elements in the dark",
            LunarPhase::FirstQuarter => "Common ones in the light",
            LunarPhase::FullMoon => "Everything at full moon",
            LunarPhase::LastQuarter => "Preparation for next cycle",
        }
    }
    
    /// Returns the duration of this phase in seconds
    ///
    /// # Returns
    /// Number of seconds this phase lasts
    ///
    /// # Note
    /// Luna4 has a 7-minute (420 second) total cycle divided equally among 4 phases.
    /// Each phase therefore lasts 105 seconds (1 minute 45 seconds).
    #[allow(dead_code)]
    pub(crate) fn duration_seconds(&self) -> u64 {
        105 // 420 seconds total / 4 phases = 105 seconds per phase
    }
    
    /// Returns the duration of this phase as a `Duration` type
    ///
    /// # Returns
    /// `Duration` representing how long this phase lasts
    #[allow(dead_code)]
    pub(crate) fn duration(&self) -> Duration {
        Duration::from_secs(self.duration_seconds())
    }
}

/// Manages the timing and progression of Luna4's lunar cycle
///
/// This struct handles the deterministic 7-minute cycle that governs
/// resource availability on Luna4. The cycle is divided into four equal
/// phases of 105 seconds each.
#[derive(Debug, Clone)]
pub(crate) struct LunarCycle {
    /// Total cycle duration in seconds (always 420 for Luna4)
    total_cycle_seconds: u64,
    /// Order of phases in the cycle
    phase_order: Vec<LunarPhase>,
}

impl Default for LunarCycle {
    /// Creates a default lunar cycle with Luna4's standard 7-minute duration
    fn default() -> Self {
        Self {
            total_cycle_seconds: 420, // 7 minutes
            phase_order: vec![
                LunarPhase::NewMoon,
                LunarPhase::FirstQuarter,
                LunarPhase::FullMoon,
                LunarPhase::LastQuarter,
            ],
        }
    }
}

impl LunarCycle {
    /// Creates a new lunar cycle with the specified total duration
    ///
    /// # Arguments
    /// * `total_seconds` - Total cycle duration in seconds
    ///
    /// # Returns
    /// New `LunarCycle` instance
    #[allow(dead_code)]
    pub(crate) fn new(total_seconds: u64) -> Self {
        Self {
            total_cycle_seconds: total_seconds,
            phase_order: vec![
                LunarPhase::NewMoon,
                LunarPhase::FirstQuarter,
                LunarPhase::FullMoon,
                LunarPhase::LastQuarter,
            ],
        }
    }
    
    /// Determines the next phase in the cycle
    ///
    /// # Arguments
    /// * `current` - Current lunar phase
    ///
    /// # Returns
    /// The phase that follows `current` in the cycle
    pub(crate) fn next_phase(&self, current: LunarPhase) -> LunarPhase {
        let current_idx = self.phase_order
            .iter()
            .position(|&p| p == current)
            .unwrap_or(0);
        
        let next_idx = (current_idx + 1) % self.phase_order.len();
        self.phase_order[next_idx]
    }
    
    /// Calculates the duration of a specific phase based on total cycle time
    ///
    /// # Arguments
    /// * `phase` - Phase to get duration for
    ///
    /// # Returns
    /// `Duration` representing how long this phase lasts
    pub(crate) fn phase_duration(&self, _phase: LunarPhase) -> Duration {
        let seconds_per_phase = self.total_cycle_seconds / self.phase_order.len() as u64;
        Duration::from_secs(seconds_per_phase)
    }
    
    /// Determines which phase should be active at a given elapsed time
    ///
    /// # Arguments
    /// * `elapsed` - Time elapsed since cycle start
    ///
    /// # Returns
    /// The lunar phase that should be active at this time
    #[allow(dead_code)]
    pub(crate) fn phase_at_time(&self, elapsed: Duration) -> LunarPhase {
        let total_secs = elapsed.as_secs();
        let seconds_per_phase = self.total_cycle_seconds / self.phase_order.len() as u64;
        let phase_index = (total_secs / seconds_per_phase) as usize % self.phase_order.len();
        
        self.phase_order[phase_index]
    }
    
    /// Calculates time remaining until the next phase transition
    ///
    /// # Arguments
    /// * `elapsed_in_current_phase` - Time already spent in current phase
    ///
    /// # Returns
    /// `Duration` until the next phase begins
    #[allow(dead_code)]
    pub(crate) fn time_until_next_phase(&self, elapsed_in_current_phase: Duration) -> Duration {
        let seconds_per_phase = self.total_cycle_seconds / self.phase_order.len() as u64;
        let current_phase_duration = Duration::from_secs(seconds_per_phase);
        
        if elapsed_in_current_phase >= current_phase_duration {
            Duration::ZERO
        } else {
            current_phase_duration - elapsed_in_current_phase
        }
    }
    
    /// Calculates progress through the current phase as a percentage
    ///
    /// # Arguments
    /// * `elapsed_in_current_phase` - Time already spent in current phase
    ///
    /// # Returns
    /// Progress as a floating-point value between 0.0 and 1.0
    #[allow(dead_code)]
    pub(crate) fn phase_progress(&self, elapsed_in_current_phase: Duration) -> f32 {
        let seconds_per_phase = self.total_cycle_seconds / self.phase_order.len() as u64;
        let elapsed_secs = elapsed_in_current_phase.as_secs_f32();
        
        (elapsed_secs / seconds_per_phase as f32).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_lunar_cycle_default_duration() {
        let cycle = LunarCycle::default();
        assert_eq!(cycle.total_cycle_seconds, 420);
    }
    
    #[test]
    fn test_phase_duration_calculation() {
        let cycle = LunarCycle::default();
        assert_eq!(cycle.phase_duration(LunarPhase::NewMoon), Duration::from_secs(105));
        assert_eq!(cycle.phase_duration(LunarPhase::FullMoon), Duration::from_secs(105));
    }
    
    #[test]
    fn test_phase_transitions() {
        let cycle = LunarCycle::default();
        assert_eq!(cycle.next_phase(LunarPhase::NewMoon), LunarPhase::FirstQuarter);
        assert_eq!(cycle.next_phase(LunarPhase::FirstQuarter), LunarPhase::FullMoon);
        assert_eq!(cycle.next_phase(LunarPhase::FullMoon), LunarPhase::LastQuarter);
        assert_eq!(cycle.next_phase(LunarPhase::LastQuarter), LunarPhase::NewMoon);
    }
    
    #[test]
    fn test_phase_at_time() {
        let cycle = LunarCycle::default();

        assert_eq!(cycle.phase_at_time(Duration::from_secs(0)), LunarPhase::NewMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(100)), LunarPhase::NewMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(110)), LunarPhase::FirstQuarter);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(220)), LunarPhase::FullMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(230)), LunarPhase::FullMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(320)), LunarPhase::LastQuarter);
    }
    
    #[test]
    fn test_time_until_next_phase() {
        let cycle = LunarCycle::default();
        
        // At start of phase
        assert_eq!(cycle.time_until_next_phase(Duration::from_secs(0)), Duration::from_secs(105));
        
        // Middle of phase
        assert_eq!(cycle.time_until_next_phase(Duration::from_secs(50)), Duration::from_secs(55));
        
        // End of phase
        assert_eq!(cycle.time_until_next_phase(Duration::from_secs(105)), Duration::from_secs(0));
        
        // Past end of phase (should clamp to zero)
        assert_eq!(cycle.time_until_next_phase(Duration::from_secs(200)), Duration::from_secs(0));
    }
    
    #[test]
    fn test_phase_progress() {
        let cycle = LunarCycle::default();
        
        assert!((cycle.phase_progress(Duration::from_secs(0)) - 0.0).abs() < 0.001);
        assert!((cycle.phase_progress(Duration::from_secs(52)) - 0.495).abs() < 0.01);
        assert!((cycle.phase_progress(Duration::from_secs(105)) - 1.0).abs() < 0.001);
        assert!((cycle.phase_progress(Duration::from_secs(200)) - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_phase_names_and_descriptions() {
        assert_eq!(LunarPhase::NewMoon.name(), "New Moon");
        assert_eq!(LunarPhase::NewMoon.description(), "Rare elements in the dark");
        
        assert_eq!(LunarPhase::FirstQuarter.name(), "First Quarter");
        assert_eq!(LunarPhase::FullMoon.description(), "Everything at full moon");
    }
}