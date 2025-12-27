//! Structured logging utilities for Luna4 planet operations
//!
//! This module provides standardized logging functions that follow the
//! common_game logging protocol while adding Luna4-specific context.
//! All logs are emitted as structured `LogEvent` instances with
//! appropriate metadata for monitoring and debugging.

use common_game::logging::{LogEvent, Participant, ActorType, EventType, Channel, Payload};
use crate::planet::{Luna4Id, LunarPhase};

/// Logs a structured event from a Luna4 planet
///
/// This is the core logging function that creates and emits a `LogEvent`
/// with Luna4-specific formatting and metadata. All other logging functions
/// in this module delegate to this function.
///
/// # Arguments
/// * `planet_id` - Identifier of the Luna4 planet
/// * `event_type` - Type of event being logged
/// * `channel` - Logging channel/severity level
/// * `message` - Human-readable message describing the event
/// * `additional_data` - Optional key-value pairs for structured data
pub fn log_planet_event(
    planet_id: Luna4Id,
    event_type: EventType,
    channel: Channel,
    message: impl AsRef<str>,
    additional_data: Option<impl IntoIterator<Item = (&'static str, String)>>,
) {
    let participant = Participant::new(ActorType::Planet, planet_id.as_u32());
    
    let mut payload = Payload::new();
    payload.insert("message".to_string(), message.as_ref().to_string());
    
    if let Some(data) = additional_data {
        for (key, value) in data {
            payload.insert(key.to_string(), value);
        }
    }
    
    let event = LogEvent::broadcast(participant, event_type, channel, payload);
    event.emit();
}

/// Logs a resource generation attempt with structured data
///
/// Use this function to log attempts to generate resources on Luna4.
/// The log includes whether the attempt succeeded and the current lunar phase.
///
/// # Arguments
/// * `planet_id` - Identifier of the Luna4 planet
/// * `resource_type` - Type of resource being generated
/// * `success` - Whether the generation succeeded
/// * `phase` - Current lunar phase during generation
pub fn log_resource_generation_attempt(
    planet_id: Luna4Id,
    resource_type: impl AsRef<str>,
    success: bool,
    phase: LunarPhase,
) {
    let message = if success {
        format!("Successfully generated {}", resource_type.as_ref())
    } else {
        format!("Failed to generate {}", resource_type.as_ref())
    };
    
    let data = [
        ("resource", resource_type.as_ref().to_string()),
        ("success", success.to_string()),
        ("lunar_phase", format!("{:?}", phase)),
    ];
    
    let channel = if success {
        Channel::Info
    } else {
        Channel::Warning
    };
    
    log_planet_event(
        planet_id,
        EventType::InternalPlanetAction,
        channel,
        message,
        Some(data),
    );
}

/// Logs a lunar phase transition with timing information
///
/// Call this function when Luna4 transitions between lunar phases.
/// The log includes both the old and new phases along with timing data.
///
/// # Arguments
/// * `planet_id` - Identifier of the Luna4 planet
/// * `from_phase` - Phase being transitioned from
/// * `to_phase` - Phase being transitioned to
/// * `duration_in_phase` - Time spent in the previous phase
pub fn log_lunar_phase_transition(
    planet_id: Luna4Id,
    from_phase: LunarPhase,
    to_phase: LunarPhase,
    duration_in_phase: std::time::Duration,
) {
    let message = format!("Phase transition: {} → {}", from_phase.name(), to_phase.name());
    
    let data = [
        ("from_phase", format!("{:?}", from_phase)),
        ("to_phase", format!("{:?}", to_phase)),
        ("duration_seconds", duration_in_phase.as_secs().to_string()),
    ];
    
    log_planet_event(
        planet_id,
        EventType::InternalPlanetAction,
        Channel::Info,
        message,
        Some(data),
    );
}

/// Logs an explorer interaction event
///
/// Use this function to log interactions between Luna4 and explorers.
/// This includes resource requests, arrivals, departures, etc.
///
/// # Arguments
/// * `planet_id` - Identifier of the Luna4 planet
/// * `explorer_id` - Identifier of the interacting explorer
/// * `action` - Type of interaction performed
/// * `success` - Whether the interaction succeeded
pub fn log_explorer_interaction(
    planet_id: Luna4Id,
    explorer_id: u32,
    action: impl AsRef<str>,
    success: bool,
) {
    let message = format!(
        "Explorer {} {}: {}",
        explorer_id,
        if success { "successfully" } else { "failed to" },
        action.as_ref()
    );
    
    let data = [
        ("explorer_id", explorer_id.to_string()),
        ("action", action.as_ref().to_string()),
        ("success", success.to_string()),
    ];
    
    let channel = if success {
        Channel::Debug
    } else {
        Channel::Warning
    };
    
    log_planet_event(
        planet_id,
        EventType::MessagePlanetToExplorer,
        channel,
        message,
        Some(data),
    );
}

/// Logs operational statistics for monitoring
///
/// Call this function periodically to log Luna4's operational metrics.
/// This provides visibility into resource generation, explorer traffic,
/// and energy status for monitoring purposes.
///
/// # Arguments
/// * `planet_id` - Identifier of the Luna4 planet
/// * `stats` - Operational statistics to log
/// * `energy_charged` - Number of charged energy cells
/// * `energy_total` - Total number of energy cells
pub fn log_operational_statistics(
    planet_id: Luna4Id,
    stats: &crate::planet::OperationalStats,  // CORRECT: using re-export from mod.rs
    energy_charged: usize,
    energy_total: usize,
) {
    let energy_percentage = if energy_total > 0 {
        (energy_charged as f32 / energy_total as f32) * 100.0
    } else {
        0.0
    };

    let data = [
        ("total_resources", stats.total_resources_generated.to_string()),
        ("phase_transitions", stats.phase_transitions.to_string()),
        ("explorer_arrivals", stats.explorer_arrivals.to_string()),
        ("energy_percentage", format!("{:.1}%", energy_percentage)),
    ];

    log_planet_event(
        planet_id,
        EventType::InternalPlanetAction,
        Channel::Info,
        "Operational statistics",
        Some(data),
    );
}


#[cfg(test)]
mod tests {
    use super::*;
    use common_game::logging::{Level, Log, Metadata, Record};
    use std::sync::{Mutex, Once};
    
    static LOGGER: TestLogger = TestLogger {
        messages: Mutex::new(Vec::new()),
    };
    static LOGGER_INIT: Once = Once::new();
    
    struct TestLogger {
        messages: Mutex<Vec<(Level, String)>>,
    }
    
    impl Log for TestLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }
    
        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let mut guard = self.messages.lock().expect("logger mutex poisoned");
                guard.push((record.level(), format!("{}", record.args())));
            }
        }
    
        fn flush(&self) {}
    }
    
    fn init_logger() {
        LOGGER_INIT.call_once(|| {
            log::set_logger(&LOGGER).expect("failed to install test logger");
            log::set_max_level(log::LevelFilter::Trace);
        });
    
        LOGGER
            .messages
            .lock()
            .expect("logger mutex poisoned")
            .clear();
    }
    
    #[test]
    fn test_log_planet_event() {
        init_logger();
        
        let planet_id = Luna4Id::new(42);
        
        log_planet_event(
            planet_id,
            EventType::InternalPlanetAction,
            Channel::Info,
            "Test message",
            Some([("key1", "value1".to_string()), ("key2", "value2".to_string())]),
        );
        
        let guard = LOGGER.messages.lock().expect("logger mutex poisoned");
        assert!(!guard.is_empty());
        
        let (level, message) = guard.last().unwrap();
        assert_eq!(*level, Level::Info);
        assert!(message.contains("Test message"));
        assert!(message.contains("key1"));
        assert!(message.contains("value1"));
    }
    
    #[test]
    fn test_log_resource_generation() {
        init_logger();
        
        let planet_id = Luna4Id::new(1);
        
        // Test successful generation
        log_resource_generation_attempt(
            planet_id,
            "Oxygen",
            true,
            LunarPhase::FullMoon,
        );
        
        let guard = LOGGER.messages.lock().expect("logger mutex poisoned");
        let (level, message) = guard.last().unwrap();
        assert_eq!(*level, Level::Info);
        assert!(message.contains("Successfully generated Oxygen"));
        assert!(message.contains("lunar_phase"));
    }
    
    #[test]
    fn test_log_explorer_interaction() {
        init_logger();
        
        let planet_id = Luna4Id::new(2);
        
        log_explorer_interaction(
            planet_id,
            100,
            "requested resources",
            true,
        );
        
        let guard = LOGGER.messages.lock().expect("logger mutex poisoned");
        let (level, message) = guard.last().unwrap();
        assert_eq!(*level, Level::Debug);
        assert!(message.contains("Explorer 100"));
        assert!(message.contains("requested resources"));
    }
}