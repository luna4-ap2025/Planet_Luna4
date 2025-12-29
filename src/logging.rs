//! Structured logging utilities for Luna4 planet operations
//!
//! Provides standardized logging functions following the
//! `common_game` logging protocol with Luna4-specific context.

use common_game::logging::{LogEvent, Participant, ActorType, EventType, Channel, Payload};
use crate::planet::{Luna4Id, LunarPhase, OperationalStats};

/// Logs a structured event from a Luna4 planet.
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

/// Logs a resource generation attempt with structured data.
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

    let channel = if success { Channel::Info } else { Channel::Warning };

    log_planet_event(
        planet_id,
        EventType::InternalPlanetAction,
        channel,
        message,
        Some(data),
    );
}

/// Logs a lunar phase transition with timing information.
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

/// Logs an explorer interaction event.
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

    let channel = if success { Channel::Debug } else { Channel::Warning };

    log_planet_event(
        planet_id,
        EventType::MessagePlanetToExplorer,
        channel,
        message,
        Some(data),
    );
}

/// Logs operational statistics for monitoring.
#[allow(dead_code)]
pub(crate) fn log_operational_statistics(
    planet_id: Luna4Id,
    stats: &OperationalStats,
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
