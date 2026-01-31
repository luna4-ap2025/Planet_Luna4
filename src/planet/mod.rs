//! Luna4 planet module
//!
//! A lunar-inspired planet with cyclic resource availability
//! and unbounded generation capabilities.

pub(crate) mod cycle;
pub(crate) mod energy;
pub(crate) mod errors;
pub(crate) mod resources;
pub(crate) mod state;

// Internal modules
pub(crate) mod luna4;

// Re-export only the public API
pub use luna4::{Luna4, Luna4Id};

// Internal re-exports (used within crate only)
pub(crate) use state::Luna4State;
pub use cycle::{LunarPhase, LunarCycle};
pub(crate) use energy::EnergyManager;
pub(crate) use resources::ResourceManager;

// Re-export common types
pub use common_game::components::planet::{Planet, PlanetAI, PlanetType};
pub use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
pub use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};

use std::collections::HashSet;
pub(crate) use common_game::components::resource::BasicResourceType;

/// Available resources information for visualizer
///
/// This struct provides real-time access to the current lunar phase
/// and resource availability, allowing the visualizer to display
/// accurate, up-to-date information about the planet's state.
#[derive(Debug, Clone)]
pub struct AvailableResources {
    /// Current lunar phase (real-time)
    pub current_phase: LunarPhase,
    /// Basic resources available in current phase
    pub basic_resources: HashSet<BasicResourceType>,
    /// All possible resources Luna4 can generate (unbounded generation list)
    pub all_possible_resources: HashSet<BasicResourceType>,
}

impl AvailableResources {
    /// Creates a new AvailableResources struct
    ///
    /// # Arguments
    /// * `current_phase` - Current lunar phase
    /// * `basic_resources` - Resources available in current phase
    /// * `all_possible_resources` - All resources Luna4 can generate
    pub fn new(
        current_phase: LunarPhase,
        basic_resources: HashSet<BasicResourceType>,
        all_possible_resources: HashSet<BasicResourceType>,
    ) -> Self {
        Self {
            current_phase,
            basic_resources,
            all_possible_resources,
        }
    }
}

/// Trait for visualizer to access planet state in real-time
pub trait PlanetVisualizer {
    /// Get current resource availability information
    ///
    /// Returns real-time information about current lunar phase
    /// and which resources are available right now.
    fn get_current_resources(&self) -> AvailableResources;
}