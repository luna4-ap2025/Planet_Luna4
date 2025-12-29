//! Luna4 core implementation module
//!
//! This module contains all Luna4-specific implementations including
//! the main Luna4 struct, unique identifiers, and AI implementation.

mod ai;
mod stats;

pub(crate) use ai::Luna4AI;

use crossbeam_channel::{Receiver, Sender};

use crate::planet::energy::EnergyManager;
use crate::planet::errors::Luna4Error;
use crate::planet::resources::ResourceManager;

use common_game::components::planet::{Planet, PlanetType};
use common_game::components::resource::BasicResourceType;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::ExplorerToPlanet;
use common_game::utils::ID;

/// Unique identifier for Luna4 planets
///
/// This new type wraps the raw `ID` (u32) to provide type safety
/// and prevent confusion with other entity identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Luna4Id(ID);

impl Luna4Id {
    /// Creates a new Luna4 identifier
    ///
    /// # Arguments
    /// * `id` - Raw identifier value
    ///
    /// # Returns
    /// New `Luna4Id` instance
    pub fn new(id: ID) -> Self {
        Self(id)
    }

    /// Returns the underlying numeric ID
    ///
    /// # Returns
    /// Raw `u32` identifier
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<ID> for Luna4Id {
    /// Converts a raw `ID` into a `Luna4Id`
    fn from(id: ID) -> Self {
        Self::new(id)
    }
}

impl std::fmt::Display for Luna4Id {
    /// Formats the Luna4 identifier for display
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Luna4#{}", self.0)
    }
}

/// Main Luna4 planet implementation
///
/// This struct serves as the factory for creating Luna4 planets.
/// It manages the integration of all Luna4-specific systems
/// (energy, resources, lunar cycles) into a single cohesive unit.
#[derive(Debug)]
pub struct Luna4 {
    /// Planet identifier
    id: Luna4Id,
    /// Energy management system
    energy: EnergyManager,
    /// Resource management system
    resources: ResourceManager,
}

impl Luna4 {
    /// Creates a new Luna4 planet instance
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this Luna4 planet
    ///
    /// # Returns
    /// `Result<Self, Luna4Error>` - The constructed Luna4 instance or an error
    ///
    /// # Errors
    /// Returns `Luna4Error::EnergyError` if energy configuration fails
    pub fn new(id: ID) -> Result<Self, Luna4Error> {
        let id = Luna4Id::new(id);
        let energy = EnergyManager::new(5)?; // Luna4 has exactly 5 energy cells
        let resources = ResourceManager::new();

        Ok(Self {
            id,
            energy,
            resources
        })
    }

    /// Creates the common Planet wrapper for orchestrator integration
    ///
    /// This method constructs a `Planet` instance that wraps Luna4's
    /// AI and integrates it with the game's messaging protocols.
    ///
    /// # Arguments
    /// * `rx_orchestrator` - Receiver for orchestrator messages
    /// * `tx_orchestrator` - Sender for planet-to-orchestrator messages
    /// * `rx_explorer` - Receiver for explorer messages
    ///
    /// # Returns
    /// `Result<Planet, Luna4Error>` - The wrapped planet instance
    ///
    /// # Errors
    /// Returns `Luna4Error::PlanetCreation` if planet construction fails
    pub fn create_planet(
        self,
        rx_orchestrator: Receiver<OrchestratorToPlanet>,
        tx_orchestrator: Sender<PlanetToOrchestrator>,
        rx_explorer: Receiver<ExplorerToPlanet>,
    ) -> Result<Planet, Luna4Error> {
        // Luna4 can generate all basic resources (unbounded generation)
        let gen_rules = vec![
            BasicResourceType::Oxygen,
            BasicResourceType::Hydrogen,
            BasicResourceType::Carbon,
            BasicResourceType::Silicon,
        ];

        // Luna4 cannot create complex resources (no combination rules)
        let comb_rules = Vec::new();

        // Create AI implementation
        let ai = Box::new(Luna4AI::new(
            self.id,
            self.energy,
            self.resources
        ));

        Planet::new(
            self.id.as_u32(),
            PlanetType::D, // Type D: 5 energy cells, unbounded generation, no rockets
            ai,
            gen_rules,
            comb_rules,
            (rx_orchestrator, tx_orchestrator),
            rx_explorer,
        ).map_err(Luna4Error::PlanetCreation)
    }

    /// Returns the planet identifier
    ///
    /// # Returns
    /// The `Luna4Id` for this planet
    #[must_use]
    pub fn id(&self) -> Luna4Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luna4_id_creation() {
        let id = Luna4Id::new(42);
        assert_eq!(id.as_u32(), 42);
    }

    #[test]
    fn test_luna4_id_display() {
        let id = Luna4Id::new(42);
        assert_eq!(format!("{}", id), "Luna4#42");
    }

    #[test]
    fn test_luna4_id_from_conversion() {
        let id: Luna4Id = 42.into();
        assert_eq!(id.as_u32(), 42);
    }

    #[test]
    fn test_luna4_constructor() {
        let luna4 = Luna4::new(1);
        assert!(luna4.is_ok());

        let luna4 = luna4.unwrap();
        assert_eq!(luna4.id().as_u32(), 1);
    }
}