//! Luna4 planet module
//!
//! A lunar-inspired planet with cyclic resource availability
//! and unbounded generation capabilities.

mod cycle;
mod energy;
mod errors;
mod luna4;
mod resources;

// Re-export only the minimal public API
pub use luna4::{Luna4, Luna4Id};

// Internal modules remain private
pub(crate) mod state;

// Core components (crate-internal only)
pub(crate) use cycle::LunarPhase;
pub(crate) use energy::{EnergyManager, EnergyStatus};
pub(crate) use errors::Luna4Error;
pub(crate) use resources::{ResourceManager, AvailableResources};
pub(crate) use state::{Luna4State, OperationalStats};

// Re-export common types for convenience
pub use common_game::components::planet::Planet;
pub use common_game::components::resource::BasicResourceType;
pub use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
pub use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
pub use common_game::components::planet::PlanetAI;
pub use common_game::components::planet::PlanetType;