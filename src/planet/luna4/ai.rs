//! Luna4 AI implementation
//!
//! This module contains the PlanetAI trait implementation for Luna4.
//! It handles all message processing from orchestrators and explorers
//! according to Luna4's lunar cycle and resource availability rules.

use std::sync::{Arc, Mutex};

use super::state::Luna4State;
use crate::planet::cycle::LunarCycle;
use crate::planet::energy::EnergyManager;
use crate::planet::resources::ResourceManager;
use crate::planet::Luna4Id;

use common_game::components::planet::{PlanetAI, PlanetState, DummyPlanetState};
use common_game::components::resource::{Generator, Combinator};
use common_game::components::rocket::Rocket;
use common_game::components::sunray::Sunray;
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};

/// Luna4 AI implementation
///
/// This struct implements the PlanetAI trait with Luna4-specific behavior:
/// - Resource generation based on lunar phases
/// - No rocket construction (Type D limitation)
/// - Phase-aware response to explorer requests
pub(crate) struct Luna4AI {
    /// Shared planet state
    state: Arc<Mutex<Luna4State>>,
    /// Energy management
    energy: EnergyManager,
    /// Resource management
    resources: ResourceManager,
    /// Lunar cycle tracking
    cycle: LunarCycle,
}

impl Luna4AI {
    /// Creates a new Luna4 AI instance
    ///
    /// # Arguments
    /// * `id` - Planet identifier
    /// * `energy` - Energy manager
    /// * `resources` - Resource manager
    /// * `cycle` - Lunar cycle tracker
    ///
    /// # Returns
    /// New `Luna4AI` instance
    pub(crate) fn new(
        id: Luna4Id,
        energy: EnergyManager,
        resources: ResourceManager,
        cycle: LunarCycle,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(Luna4State::new(id.as_u32()))),
            energy,
            resources,
            cycle,
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
        // Update lunar phase before processing
        {
            let mut luna_state = self.state.lock().unwrap();
            luna_state.update_phase();
        }
        
        // Charge a cell using Luna4's energy management
        if let Err(e) = self.energy.charge_cell(sunray, state) {
            // Record error but don't fail - sunray might be wasted
            let mut luna_state = self.state.lock().unwrap();
            luna_state.stats.record_error();
            log::warn!("Failed to charge cell: {}", e);
        }
    }
    
    fn handle_asteroid(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> Option<Rocket> {
        // Luna4 is a Type D planet - no rockets allowed
        // Update phase for consistency
        {
            let mut luna_state = self.state.lock().unwrap();
            luna_state.update_phase();
        }
        
        None
    }
    
    fn handle_internal_state_req(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> DummyPlanetState {
        // Update phase before responding
        {
            let mut luna_state = self.state.lock().unwrap();
            luna_state.update_phase();
        }
        
        // Return the dummy state
        state.to_dummy()
    }
    
    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        _combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        // Update lunar phase
        let current_phase = {
            let mut luna_state = self.state.lock().unwrap();
            luna_state.update_phase();
            luna_state.current_phase
        };
        
        // Get available resources for current phase
        let available_resources = self.resources.get_available_resources(current_phase);
        
        match msg {
            ExplorerToPlanet::SupportedResourceRequest { explorer_id } => {
                // Register explorer interaction
                {
                    let mut luna_state = self.state.lock().unwrap();
                    if !luna_state.present_explorers.contains_key(&explorer_id) {
                        luna_state.register_explorer_arrival(explorer_id);
                    }
                }
                
                // Return available resources for current phase
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: available_resources.basic.clone(),
                })
            }
            
            ExplorerToPlanet::SupportedCombinationRequest { explorer_id: _ } => {
                // Luna4 doesn't support complex resources
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: available_resources.complex.clone(),
                })
            }
            
            ExplorerToPlanet::GenerateResourceRequest { explorer_id, resource } => {
                // Check if resource is available in current phase
                if !available_resources.basic.contains(&resource) {
                    return Some(PlanetToExplorer::GenerateResourceResponse {
                        resource: None,
                    });
                }
                
                // Try to generate the resource
                match self.energy.use_energy_cell(state, |cell| {
                    generator.try_make(resource, cell)
                }) {
                    Ok(generated_resource) => {
                        // Record successful generation
                        {
                            let mut luna_state = self.state.lock().unwrap();
                            luna_state.record_generation(resource);
                        }
                        
                        Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: Some(generated_resource),
                        })
                    }
                    Err(_) => {
                        // Record error
                        {
                            let mut luna_state = self.state.lock().unwrap();
                            luna_state.stats.record_error();
                        }
                        
                        Some(PlanetToExplorer::GenerateResourceResponse {
                            resource: None,
                        })
                    }
                }
            }
            
            ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id: _ } => {
                let available_cells = self.energy.available_charged_cells(state);
                Some(PlanetToExplorer::AvailableEnergyCellResponse {
                    available_cells: available_cells as u32,
                })
            }
            
       ExplorerToPlanet::CombineResourceRequest { explorer_id: _, msg } => {
    // Extract the actual resources from the request
    let (resource1, resource2) = match msg {
        ComplexResourceRequest::Water(h, o) => (h.to_generic(), o.to_generic()),
        ComplexResourceRequest::Diamond(c1, c2) => (c1.to_generic(), c2.to_generic()),
        ComplexResourceRequest::Life(w, c) => (w.to_generic(), c.to_generic()),
        ComplexResourceRequest::Robot(s, l) => (s.to_generic(), l.to_generic()),
        ComplexResourceRequest::Dolphin(w, l) => (w.to_generic(), l.to_generic()),
        ComplexResourceRequest::AIPartner(r, d) => (r.to_generic(), d.to_generic()),
    };
    
    Some(PlanetToExplorer::CombineResourceResponse {
        complex_response: Err((
            "Luna4 does not support resource combination".to_string(),
            resource1,
            resource2,
        )),
    })
}
                        common_game::components::resource::GenericResource::BasicResources(
                            common_game::components::resource::BasicResource::Oxygen(
                                common_game::components::resource::Oxygen { _private: () }
                            )
                        ),
                        common_game::components::resource::GenericResource::BasicResources(
                            common_game::components::resource::BasicResource::Hydrogen(
                                common_game::components::resource::Hydrogen { _private: () }
                            )
                        ),
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
        let mut luna_state = self.state.lock().unwrap();
        luna_state.register_explorer_arrival(explorer_id);
        log::info!("🌙 Explorer #{} arrived at Luna4", explorer_id);
    }
    
    fn on_explorer_departure(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        explorer_id: u32,
    ) {
        let mut luna_state = self.state.lock().unwrap();
        luna_state.register_explorer_departure(explorer_id);
        log::info!("🌙 Explorer #{} departed from Luna4", explorer_id);
    }
    
    fn on_start(
        &mut self,
        _state: &PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) {
        let mut luna_state = self.state.lock().unwrap();
        luna_state.phase_start_time = std::time::Instant::now();
        log::info!("🌙 Luna4 #{} started operation", luna_state.id.as_u32());
    }
    
    fn on_stop(
        &mut self,
        _state: &PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) {
        let luna_state = self.state.lock().unwrap();
        log::info!("🌙 Luna4 #{} stopped operation", luna_state.id.as_u32());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_game::components::resource::BasicResourceType;
    
    #[test]
    fn test_ai_creation() {
        let id = Luna4Id::new(1);
        let energy = EnergyManager::new(5).unwrap();
        let resources = ResourceManager::new();
        let cycle = LunarCycle::default();
        
        let ai = Luna4AI::new(id, energy, resources, cycle);
        
        // Should compile and create without panicking
        let _ = ai;
    }
}