//! Resource management for Luna4
//!
//! Manages resource availability based on lunar phases according to Luna4 lore.
//! Each lunar phase enables different sets of basic resources while complex
//! resources are never available on Luna4 (Type D planet limitation).

use std::collections::HashSet;

use super::cycle::LunarPhase;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};

/// Resources available during a specific lunar phase
#[derive(Debug, Clone)]
pub struct PhaseResources {
    /// Basic resources that can be generated in this phase
    pub basic: HashSet<BasicResourceType>,
    /// Complex resources that can be generated in this phase (always empty for Luna4)
    pub complex: HashSet<ComplexResourceType>,
}

impl PhaseResources {
    /// Creates a resource set for a phase
    pub fn basic(resources: Vec<BasicResourceType>) -> Self {
        Self {
            basic: resources.into_iter().collect(),
            complex: HashSet::new(),
        }
    }
}

/// Manages resource availability based on Luna4's lunar phases
#[derive(Debug, Clone)]
pub struct ResourceManager;

impl ResourceManager {
    /// Creates a new resource manager with Luna4's phase-resource mappings
    pub fn new() -> Self {
        Self
    }

    /// Retrieves available resources for a specific lunar phase
    pub fn get_available_resources(&self, phase: LunarPhase) -> PhaseResources {
        match phase {
            LunarPhase::NewMoon => {
                // Rare elements in the dark: Carbon only
                PhaseResources::basic(vec![BasicResourceType::Carbon])
            }
            LunarPhase::FirstQuarter => {
                // Common ones in the light: Oxygen and Hydrogen
                PhaseResources::basic(vec![
                    BasicResourceType::Oxygen,
                    BasicResourceType::Hydrogen,
                ])
            }
            LunarPhase::FullMoon => {
                // Everything at full moon: All basic resources
                PhaseResources::basic(vec![
                    BasicResourceType::Oxygen,
                    BasicResourceType::Hydrogen,
                    BasicResourceType::Carbon,
                    BasicResourceType::Silicon,
                ])
            }
            LunarPhase::LastQuarter => {
                // Preparation phase: Oxygen and Silicon
                PhaseResources::basic(vec![
                    BasicResourceType::Oxygen,
                    BasicResourceType::Silicon,
                ])
            }
        }
    }
}
