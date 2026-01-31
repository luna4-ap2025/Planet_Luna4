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
    /// New Moon phase - rare elements only (Carbon)
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
    pub fn name(&self) -> &'static str {
        match self {
            LunarPhase::NewMoon => "New Moon",
            LunarPhase::FirstQuarter => "First Quarter",
            LunarPhase::FullMoon => "Full Moon",
            LunarPhase::LastQuarter => "Last Quarter",
        }
    }

    /// Returns the lore description of what this phase represents
    pub fn description(&self) -> &'static str {
        match self {
            LunarPhase::NewMoon => "Rare elements in the dark",
            LunarPhase::FirstQuarter => "Common ones in the light",
            LunarPhase::FullMoon => "Everything at full moon",
            LunarPhase::LastQuarter => "Preparation for next cycle",
        }
    }

    /// Returns the duration of this phase in seconds
    pub fn duration_seconds(&self) -> u64 {
        105 // 420 seconds total / 4 phases = 105 seconds per phase
    }

    /// Returns the next phase in the cycle
    pub fn next(&self) -> Self {
        match self {
            LunarPhase::NewMoon => LunarPhase::FirstQuarter,
            LunarPhase::FirstQuarter => LunarPhase::FullMoon,
            LunarPhase::FullMoon => LunarPhase::LastQuarter,
            LunarPhase::LastQuarter => LunarPhase::NewMoon,
        }
    }
}

/// Manages the timing and progression of Luna4's lunar cycle
#[derive(Debug, Clone)]
pub struct LunarCycle {
    /// Total cycle duration in seconds (always 420 for Luna4)
    pub(crate) total_cycle_seconds: u64,
    /// Order of phases in the cycle
    phase_order: Vec<LunarPhase>,
}

impl Default for LunarCycle {
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
    /// Creates a new lunar cycle
    pub fn new(total_seconds: u64) -> Self {
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

    /// Calculates the duration of a specific phase
    pub fn phase_duration(&self, _phase: LunarPhase) -> Duration {
        let seconds_per_phase = self.total_cycle_seconds / self.phase_order.len() as u64;
        Duration::from_secs(seconds_per_phase)
    }

    /// Determines which phase should be active at a given elapsed time
    pub fn phase_at_time(&self, elapsed: Duration) -> LunarPhase {
        let total_secs = elapsed.as_secs();
        let seconds_per_phase = self.total_cycle_seconds / self.phase_order.len() as u64;
        let phase_index = (total_secs / seconds_per_phase) as usize % self.phase_order.len();

        self.phase_order[phase_index]
    }
}

