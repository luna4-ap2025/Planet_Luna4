//! Luna4 core implementation module
//!
//! This module contains all Luna4-specific implementations including
//! the main Luna4 struct, unique identifiers, and AI implementation.

//! Luna4 core implementation module

pub(crate) mod ai;
pub(crate) mod stats;

pub(crate) use ai::Luna4AI;
pub(crate) use stats::Luna4Stats;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Luna4Id(ID);

impl Luna4Id {
    /// Creates a new Luna4 identifier
    pub fn new(id: ID) -> Self {
        Self(id)
    }

    /// Returns the underlying numeric ID
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<ID> for Luna4Id {
    fn from(id: ID) -> Self {
        Self::new(id)
    }
}

impl std::fmt::Display for Luna4Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Luna4#{}", self.0)
    }
}

/// Main Luna4 planet implementation
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
    pub fn new(id: ID) -> Result<Self, Luna4Error> {
        let id = Luna4Id::new(id);
        let energy = EnergyManager::new(5)?; // Luna4 has exactly 5 energy cells
        let resources = ResourceManager::new();

        Ok(Self {
            id,
            energy,
            resources,
        })
    }

    /// Creates the common Planet wrapper for orchestrator integration
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
            self.resources,
        ));

        Planet::new(
            self.id.as_u32(), // Planet API expects raw u32 ID
            PlanetType::D, // Type D: 5 energy cells, unbounded generation, no rockets
            ai,
            gen_rules,
            comb_rules,
            (rx_orchestrator, tx_orchestrator),
            rx_explorer,
        ).map_err(|e| Luna4Error::PlanetCreation(e))
    }

    /// Returns the planet identifier
    #[must_use]
    pub fn id(&self) -> Luna4Id {
        self.id
    }
}
