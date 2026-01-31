//! Luna4 AI implementation
//!
//! This module contains the PlanetAI trait implementation for Luna4.
//! It handles all message processing from orchestrators and explorers
//! according to Luna4's lunar cycle and resource availability rules.

//! Luna4 AI implementation

use crate::planet::state::Luna4State;
use crate::planet::energy::EnergyManager;
use crate::planet::resources::ResourceManager;
use crate::planet::luna4::Luna4Id;

use common_game::components::planet::{PlanetAI, PlanetState, DummyPlanetState};
use common_game::components::resource::{Generator, Combinator};
use common_game::components::rocket::Rocket;
use common_game::components::sunray::Sunray;
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};

/// Luna4 AI implementation
pub struct Luna4AI {
    /// Planet state
    state: Luna4State,
    /// Energy management
    energy: EnergyManager,
    /// Resource management
    resources: ResourceManager,
}

impl Luna4AI {
    /// Creates a new Luna4 AI instance
    pub fn new(
        id: Luna4Id,
        energy: EnergyManager,
        resources: ResourceManager,
    ) -> Self {
        Self {
            state: Luna4State::new(id.as_u32()),
            energy,
            resources,
        }
    }
}

impl PlanetAI for Luna4AI {
    fn handle_sunray(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        sunray: Sunray,
    ) {
        self.state.update_phase();
        self.state.stats.record_sunray_received();

        if let Err(e) = self.energy.charge_cell(sunray, state) {
            log::warn!("Failed to charge cell: {}", e);
            self.state.stats.record_failed_generation();
        }
    }

    fn handle_asteroid(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> Option<Rocket> {
        self.state.update_phase();
        None // Type D: No rockets
    }

    fn handle_internal_state_req(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> DummyPlanetState {
        self.state.update_phase();
        state.to_dummy()
    }

    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        _combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        self.state.update_phase();

        let current_phase = self.state.current_phase;
        let available = self.resources.get_available_resources(current_phase);

        match msg {
            ExplorerToPlanet::SupportedResourceRequest { explorer_id } => {
                self.state.register_explorer_arrival(explorer_id);
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: available.basic.clone(),
                })
            }

            ExplorerToPlanet::SupportedCombinationRequest { explorer_id } => {
                self.state.register_explorer_arrival(explorer_id);
                // Luna4 doesn't support complex resources
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: available.complex.clone(),
                })
            }

            ExplorerToPlanet::GenerateResourceRequest { explorer_id, resource } => {
                self.state.register_explorer_arrival(explorer_id);

                // Check if resource is available in current phase
                if !available.basic.contains(&resource) {
                    self.state.stats.record_failed_generation();
                    return Some(PlanetToExplorer::GenerateResourceResponse {
                        resource: None,
                    });
                }

                // Try to generate the resource
                match self.energy.use_energy_cell(state, |cell| {
                    generator.try_make(resource, cell)
                }) {
                    Ok(generated_resource) => {
                        self.state.stats.record_successful_generation();
                        Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: Some(generated_resource),
                        })
                    }
                    Err(_) => {
                        self.state.stats.record_failed_generation();
                        Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: None,
                        })
                    }
                }
            }

            ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id } => {
                self.state.register_explorer_arrival(explorer_id);
                let available_cells = self.energy.available_charged_cells(state);
                Some(PlanetToExplorer::AvailableEnergyCellResponse {
                    available_cells: available_cells as u32,
                })
            }

            ExplorerToPlanet::CombineResourceRequest { explorer_id, msg } => {
                self.state.register_explorer_arrival(explorer_id);
                self.state.stats.record_failed_generation();

                // Type D: No combination support
                // Extract resources from the request to return them
                use common_game::components::resource::ComplexResourceRequest;

                let (resource1, resource2) = match msg {
                    ComplexResourceRequest::Water(lhs, rhs) => (lhs.to_generic(), rhs.to_generic()),
                    ComplexResourceRequest::Diamond(lhs, rhs) => (lhs.to_generic(), rhs.to_generic()),
                    ComplexResourceRequest::Life(lhs, rhs) => (lhs.to_generic(), rhs.to_generic()),
                    ComplexResourceRequest::Robot(lhs, rhs) => (lhs.to_generic(), rhs.to_generic()),
                    ComplexResourceRequest::Dolphin(lhs, rhs) => (lhs.to_generic(), rhs.to_generic()),
                    ComplexResourceRequest::AIPartner(lhs, rhs) => (lhs.to_generic(), rhs.to_generic()),
                };

                Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: Err((
                        "Luna4 (Type D) does not support resource combination".to_string(),
                        resource1,
                        resource2,
                    )),
                })
            }
        }
    }

    fn on_explorer_arrival(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        explorer_id: u32,
    ) {
        self.state.register_explorer_arrival(explorer_id);
    }

    fn on_explorer_departure(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        explorer_id: u32,
    ) {
        self.state.register_explorer_departure(explorer_id);
    }

    fn on_start(
        &mut self,
        _state: &PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) {
        self.state.phase_start_time = std::time::Instant::now();
        log::info!("Luna4 #{} started operation", self.state.id);
    }

    fn on_stop(
        &mut self,
        _state: &PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) {
        log::info!("Luna4 #{} stopped operation", self.state.id);
    }
}