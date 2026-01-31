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
pub(crate) use cycle::{LunarPhase, LunarCycle};
pub(crate) use energy::EnergyManager;
pub(crate) use resources::ResourceManager;

// Re-export common types
pub use common_game::components::planet::{Planet, PlanetAI, PlanetType};
pub use common_game::components::resource::BasicResourceType;
pub use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
pub use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};