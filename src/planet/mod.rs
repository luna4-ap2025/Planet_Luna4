//! Luna4 planet module
//!
//! A lunar-inspired planet with cyclic resource availability
//! and unbounded generation capabilities.

mod cycle;
mod energy;
mod errors;
mod resources;
mod state;

// Internal modules
mod luna4;

// Re-export only the public API
pub use luna4::{Luna4, Luna4Id};

// Internal re-exports (used within crate only)
pub(crate) use state::{Luna4State, OperationalStats};
pub(crate) use cycle::LunarPhase;
pub(crate) use energy::{EnergyManager, EnergyStatus};
pub(crate) use errors::Luna4Error;
pub(crate) use resources::{ResourceManager, AvailableResources};

// Re-export common types that might be needed by users of this crate
pub use common_game::components::planet::{Planet, PlanetAI, PlanetType};
pub use common_game::components::resource::BasicResourceType;
pub use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
pub use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};