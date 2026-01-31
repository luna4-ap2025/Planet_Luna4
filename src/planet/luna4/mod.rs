//! Luna4 core implementation module
//!
//! This module contains all Luna4-specific implementations including
//! the main Luna4 struct, unique identifiers, and AI implementation.

pub(crate) mod ai;
pub(crate) mod stats;

pub(crate) use ai::Luna4AI;
pub(crate) use stats::Luna4Stats;

use crossbeam_channel::{Receiver, Sender};

use crate::planet::energy::EnergyManager;
use crate::planet::errors::Luna4Error;
use crate::planet::resources::ResourceManager;

use common_game::components::planet::{Planet, PlanetAI, PlanetState, PlanetType, DummyPlanetState};
use common_game::components::resource::{BasicResourceType, Combinator, Generator};
use common_game::components::rocket::Rocket;
use common_game::components::sunray::Sunray;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use common_game::utils::ID;

use std::sync::{Arc, RwLock};

/// Unique identifier for Luna4 planets
///
/// Wraps a numeric ID to provide type safety and Luna4-specific formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Luna4Id(ID);

impl Luna4Id {
    /// Creates a new Luna4 identifier from a raw numeric ID
    pub fn new(id: ID) -> Self {
        Self(id)
    }

    /// Returns the underlying numeric ID as a u32
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
///
/// This struct represents a Luna4 planet instance with its unique identifier,
/// energy management system, resource management system, and AI.
/// It provides methods to create the planet and query its current state.
pub struct Luna4 {
    /// Unique identifier for this Luna4 instance
    id: Luna4Id,
    /// Energy management system (handles 5 energy cells)
    energy: EnergyManager,
    /// Resource management system (phase-based resource availability)
    resources: ResourceManager,
    /// AI instance wrapped for visualizer access (shared, thread-safe reference)
    ai_for_visualizer: Option<Arc<RwLock<Luna4AI>>>,
}

impl Luna4 {
    /// Creates a new Luna4 planet instance
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the new Luna4 planet
    ///
    /// # Returns
    /// `Result<Self, Luna4Error>` - The new Luna4 instance or an error
    ///
    /// # Errors
    /// Returns `Luna4Error::EnergyError` if energy cell configuration is invalid
    pub fn new(id: ID) -> Result<Self, Luna4Error> {
        let id = Luna4Id::new(id);
        let energy = EnergyManager::new(5)?;
        let resources = ResourceManager::new();

        Ok(Self {
            id,
            energy,
            resources,
            ai_for_visualizer: None,
        })
    }

    /// Creates the common `Planet` wrapper for orchestrator integration
    ///
    /// This method transforms the Luna4 instance into a game-compatible `Planet`
    /// that can be managed by the orchestrator. It sets up communication channels
    /// and wraps the AI for both game logic and visualizer access.
    ///
    /// # Arguments
    /// * `rx_orchestrator` - Receiver for orchestrator messages
    /// * `tx_orchestrator` - Sender for messages to the orchestrator
    /// * `rx_explorer` - Receiver for explorer messages
    ///
    /// # Returns
    /// `Result<Planet, Luna4Error>` - The wrapped planet or an error
    ///
    /// # Note
    /// After calling this method, the Luna4 instance can still be used
    /// to query current state via `get_current_resources()`
    pub fn create_planet(
        self,
        rx_orchestrator: Receiver<OrchestratorToPlanet>,
        tx_orchestrator: Sender<PlanetToOrchestrator>,
        rx_explorer: Receiver<ExplorerToPlanet>,
    ) -> Result<Planet, Luna4Error> {
        let mut this = self;

        // Luna4 can generate all basic resources (unbounded generation)
        let gen_rules = vec![
            BasicResourceType::Oxygen,
            BasicResourceType::Hydrogen,
            BasicResourceType::Carbon,
            BasicResourceType::Silicon,
        ];

        // Type D planets do not support resource combination
        let comb_rules = Vec::new();

        // Create the AI instance
        let ai = Luna4AI::new(this.id, this.energy, this.resources);
        let ai_arc = Arc::new(RwLock::new(ai));

        // Store reference for visualizer access
        this.ai_for_visualizer = Some(ai_arc.clone());

        /// Wrapper type that implements `PlanetAI` while delegating to the shared Luna4AI
        ///
        /// This allows the same AI instance to be used by both the game engine
        /// (via the `PlanetAI` trait) and the visualizer (via direct access).
        struct Luna4AIWrapper(Arc<RwLock<Luna4AI>>);

        impl PlanetAI for Luna4AIWrapper {
            fn handle_sunray(
                &mut self,
                state: &mut PlanetState,
                generator: &Generator,
                combinator: &Combinator,
                sunray: Sunray,
            ) {
                self.0.write().unwrap().handle_sunray(state, generator, combinator, sunray)
            }

            fn handle_asteroid(
                &mut self,
                state: &mut PlanetState,
                generator: &Generator,
                combinator: &Combinator,
            ) -> Option<Rocket> {
                self.0.write().unwrap().handle_asteroid(state, generator, combinator)
            }

            fn handle_internal_state_req(
                &mut self,
                state: &mut PlanetState,
                generator: &Generator,
                combinator: &Combinator,
            ) -> DummyPlanetState {
                self.0.write().unwrap().handle_internal_state_req(state, generator, combinator)
            }

            fn handle_explorer_msg(
                &mut self,
                state: &mut PlanetState,
                generator: &Generator,
                combinator: &Combinator,
                msg: ExplorerToPlanet,
            ) -> Option<PlanetToExplorer> {
                self.0.write().unwrap().handle_explorer_msg(state, generator, combinator, msg)
            }

            fn on_explorer_arrival(
                &mut self,
                state: &mut PlanetState,
                generator: &Generator,
                combinator: &Combinator,
                explorer_id: u32,
            ) {
                self.0.write().unwrap().on_explorer_arrival(state, generator, combinator, explorer_id)
            }

            fn on_explorer_departure(
                &mut self,
                state: &mut PlanetState,
                generator: &Generator,
                combinator: &Combinator,
                explorer_id: u32,
            ) {
                self.0.write().unwrap().on_explorer_departure(state, generator, combinator, explorer_id)
            }

            fn on_start(
                &mut self,
                state: &PlanetState,
                generator: &Generator,
                combinator: &Combinator,
            ) {
                self.0.write().unwrap().on_start(state, generator, combinator)
            }

            fn on_stop(
                &mut self,
                state: &PlanetState,
                generator: &Generator,
                combinator: &Combinator,
            ) {
                self.0.write().unwrap().on_stop(state, generator, combinator)
            }
        }

        // Create the boxed AI for the Planet wrapper
        let ai: Box<dyn PlanetAI> = Box::new(Luna4AIWrapper(ai_arc));

        // Create the final Planet instance
        Planet::new(
            this.id.as_u32(),
            PlanetType::D,
            ai,
            gen_rules,
            comb_rules,
            (rx_orchestrator, tx_orchestrator),
            rx_explorer,
        )
            .map_err(Luna4Error::PlanetCreation)
    }

    /// Returns the planet's unique identifier
    #[must_use]
    pub fn id(&self) -> Luna4Id {
        self.id
    }
}

// Import the visualizer-related types
use crate::planet::{AvailableResources, PlanetVisualizer};

impl Luna4 {
    /// Gets current resource availability information for the visualizer
    ///
    /// This method provides real-time access to the current lunar phase
    /// and which resources are available in that phase. It's thread-safe
    /// and can be called at any time after `create_planet()`.
    ///
    /// # Returns
    /// `Option<AvailableResources>` - Current resource information if available,
    /// or `None` if the planet hasn't been initialized yet.
    ///
    /// # Example
    /// ```no_run
    /// let luna4 = Luna4::new(1).unwrap();
    /// // ... create planet and run it ...
    /// if let Some(resources) = luna4.get_current_resources() {
    ///     println!("Current phase: {:?}", resources.current_phase);
    ///     println!("Available resources: {:?}", resources.basic_resources);
    /// }
    /// ```
    pub fn get_current_resources(&self) -> Option<AvailableResources> {
        self.ai_for_visualizer
            .as_ref()
            .and_then(|ai| ai.read().ok())
            .map(|ai| ai.get_current_resources())
    }
}