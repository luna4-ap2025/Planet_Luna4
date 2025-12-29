//! Luna4 - The quiet, essential resource moon
//!
//! A planetary implementation with lunar-inspired resource cycles
//! that provides unbounded resource generation and support capabilities.
//! This crate implements a Type D planet with 5 energy cells,
//! no rockets, and phase-based resource availability according to
//! Luna4's 7-minute lunar cycle.

pub mod planet;
pub mod logging;

// Re-export the main entry point
pub use planet::{Luna4, Luna4Id};

/// Main entry point for Luna4 planet
///
/// This function is called by the orchestrator to spawn a Luna4 planet.
/// It creates a new Luna4 instance, wraps it in the common `Planet` type,
/// and returns it ready for execution.
///
/// # Arguments
/// * `id` - Unique identifier for the new planet
/// * `rx_orchestrator` - Receiver for messages from the orchestrator
/// * `tx_orchestrator` - Sender for messages to the orchestrator
/// * `rx_explorer` - Receiver for messages from explorers
///
/// # Returns
/// `Result<Planet, String>` - The constructed planet or an error message
///
/// # Errors
/// Returns an error string if planet creation fails at any step
pub fn create_planet(
    id: u32,
    rx_orchestrator: crossbeam_channel::Receiver<common_game::protocols::orchestrator_planet::OrchestratorToPlanet>,
    tx_orchestrator: crossbeam_channel::Sender<common_game::protocols::orchestrator_planet::PlanetToOrchestrator>,
    rx_explorer: crossbeam_channel::Receiver<common_game::protocols::planet_explorer::ExplorerToPlanet>,
) -> Result<planet::Planet, String> {
    use logging::log_planet_event;
    use common_game::logging::{EventType, Channel};
    
    let planet_id = Luna4Id::new(id);
    
    // Log initialization using structured logging
log_planet_event(
    planet_id,
    EventType::InternalPlanetAction,
    Channel::Info,
    "Luna4 initializing",
    None::<[(&str, String); 0]>,  // Explicit type annotation
);
    
    // Create Luna4 instance
    let luna4 = match Luna4::new(id) {
        Ok(luna) => luna,
        Err(e) => return Err(format!("Failed to create Luna4: {e}")),
    };
    
    // Create the common Planet wrapper
    let planet = match luna4.create_planet(rx_orchestrator, tx_orchestrator, rx_explorer) {
        Ok(p) => p,
        Err(e) => return Err(format!("Failed to create planet: {e}")),
    };
    
    // Log successful initialization
log_planet_event(
    planet_id,
    EventType::InternalPlanetAction,
    Channel::Info,
    "Luna4 initialization complete",
    None::<[(&str, String); 0]>,  // Explicit type annotation
);
    
    Ok(planet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::unbounded;

    //noinspection ALL
    #[test]
    fn test_create_planet_function_exists() {
        // Just verify the function signature compiles
        let (_tx_orch, rx_orch) = unbounded::<common_game::protocols::orchestrator_planet::OrchestratorToPlanet>();
        let (tx_planet, _rx_planet) = unbounded::<common_game::protocols::orchestrator_planet::PlanetToOrchestrator>();
        let (_tx_expl, rx_expl) = unbounded::<common_game::protocols::planet_explorer::ExplorerToPlanet>();
        
        // Should compile - actual execution would require proper setup
        let _result = create_planet(1, rx_orch, tx_planet, rx_expl);
    }
}