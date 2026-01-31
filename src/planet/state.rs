//! Luna4 state management
//!
//! Tracks the operational state of a Luna4 planet including
//! lunar phase timing, explorer presence, and statistics.
//! This is internal state that should not be exposed to external users.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use super::cycle::LunarPhase;
use crate::planet::luna4::Luna4Id;
use crate::planet::luna4::stats::Luna4Stats;

/// Tracks the operational state of a Luna4 planet
#[derive(Debug)]
pub struct Luna4State {
    /// Planet identifier
    pub id: u32,
    /// Current lunar phase
    pub current_phase: LunarPhase,
    /// When the current phase started
    pub phase_start_time: Instant,
    /// Explorer presence tracking
    pub present_explorers: HashSet<u32>,
    /// Operational statistics
    pub stats: Luna4Stats,
}

impl Luna4State {
    /// Creates a new Luna4 state
    pub fn new(id: u32) -> Self {
        Self {
            id,
            current_phase: LunarPhase::NewMoon,
            phase_start_time: Instant::now(),
            present_explorers: HashSet::new(),
            stats: Luna4Stats::new(),
        }
    }

    /// Updates the lunar phase based on elapsed time
    pub fn update_phase(&mut self) -> Option<LunarPhase> {
        let elapsed = self.phase_start_time.elapsed();
        let seconds_per_phase = 105; // Luna4: 105 seconds per phase

        if elapsed.as_secs() >= seconds_per_phase {
            let old_phase = self.current_phase;
            self.current_phase = self.current_phase.next();
            self.phase_start_time = Instant::now();
            Some(old_phase)
        } else {
            None
        }
    }

    /// Registers an explorer arrival
    pub fn register_explorer_arrival(&mut self, explorer_id: u32) {
        self.present_explorers.insert(explorer_id);
        self.stats.record_explorer_message_processed();
    }

    /// Registers an explorer departure
    pub fn register_explorer_departure(&mut self, explorer_id: u32) {
        self.present_explorers.remove(&explorer_id);
    }

    /// Gets the number of present explorers
    pub fn explorer_count(&self) -> usize {
        self.present_explorers.len()
    }
}

/// Operational statistics for Luna4
#[derive(Debug, Clone, Default)]
pub struct OperationalStats {
    /// Total resources generated
    pub total_resources_generated: usize,
    /// Phase transitions observed
    pub phase_transitions: usize,
    /// Explorer arrivals
    pub explorer_arrivals: usize,
    /// Explorer departures
    pub explorer_departures: usize,
}

impl OperationalStats {
    /// Creates new operational statistics
    pub fn new() -> Self {
        Self::default()
    }
}