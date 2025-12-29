//! Resource management for Luna4
//!
//! Manages resource availability based on lunar phases according to Luna4 lore.
//! Each lunar phase enables different sets of basic resources while complex
//! resources are never available on Luna4 (Type D planet limitation).

use std::collections::HashSet;

use super::cycle::LunarPhase;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};

/// Resources available during a specific lunar phase
///
/// This struct defines which basic and complex resources can be generated
/// during each phase of Luna4's lunar cycle.
#[derive(Debug, Clone)]
pub(crate) struct AvailableResources {
    /// Basic resources that can be generated in this phase
    pub(crate) basic: HashSet<BasicResourceType>,
    /// Complex resources that can be generated in this phase (always empty for Luna4)
    pub(crate) complex: HashSet<ComplexResourceType>,
}

impl AvailableResources {
    /// Creates an empty resource set (no resources available)
    ///
    /// # Returns
    /// `AvailableResources` with empty basic and complex sets
    #[allow(dead_code)]
    pub(crate) fn empty() -> Self {
        Self {
            basic: HashSet::new(),
            complex: HashSet::new(),
        }
    }
    
    /// Creates the resource set for New Moon phase
    ///
    /// New Moon enables rare elements only (Silicon and Carbon)
    /// according to the "rare elements in the dark" lore.
    ///
    /// # Returns
    /// `AvailableResources` for New Moon phase
    pub(crate) fn new_moon() -> Self {
        let mut basic = HashSet::new();
        // Rare elements in the dark
        basic.insert(BasicResourceType::Silicon);
        basic.insert(BasicResourceType::Carbon);
        
        Self {
            basic,
            complex: HashSet::new(),
        }
    }
    
    /// Creates the resource set for First Quarter phase
    ///
    /// First Quarter enables common elements only (Oxygen and Hydrogen)
    /// according to the "common ones in the light" lore.
    ///
    /// # Returns
    /// `AvailableResources` for First Quarter phase
    pub(crate) fn first_quarter() -> Self {
        let mut basic = HashSet::new();
        // Common ones in the light
        basic.insert(BasicResourceType::Oxygen);
        basic.insert(BasicResourceType::Hydrogen);
        
        Self {
            basic,
            complex: HashSet::new(),
        }
    }
    
    /// Creates the resource set for Full Moon phase
    ///
    /// Full Moon enables all basic resources (Oxygen, Hydrogen, Carbon, Silicon)
    /// according to the "everything at full moon" lore.
    ///
    /// # Returns
    /// `AvailableResources` for Full Moon phase
    pub(crate) fn full_moon() -> Self {
        let mut basic = HashSet::new();
        // Everything at full moon
        basic.insert(BasicResourceType::Oxygen);
        basic.insert(BasicResourceType::Hydrogen);
        basic.insert(BasicResourceType::Carbon);
        basic.insert(BasicResourceType::Silicon);
        
        Self {
            basic,
            complex: HashSet::new(),
        }
    }
    
    /// Creates the resource set for Last Quarter phase
    ///
    /// Last Quarter is a preparation phase with limited resources
    /// (only Hydrogen as basic fuel).
    ///
    /// # Returns
    /// `AvailableResources` for Last Quarter phase
    pub(crate) fn last_quarter() -> Self {
        let mut basic = HashSet::new();
        // Preparation phase - limited resources
        basic.insert(BasicResourceType::Hydrogen);
        
        Self {
            basic,
            complex: HashSet::new(),
        }
    }
}

/// Manages resource availability based on Luna4's lunar phases
///
/// This struct maps each lunar phase to its corresponding available resources
/// and provides query methods to check what resources can be generated.
#[derive(Debug, Clone)]
pub(crate) struct ResourceManager {
    /// Resource sets indexed by lunar phase
    phase_resources: [AvailableResources; 4],
}

impl ResourceManager {
    /// Creates a new resource manager with Luna4's phase-resource mappings
    ///
    /// # Returns
    /// New `ResourceManager` instance
    pub(crate) fn new() -> Self {
        Self {
            phase_resources: [
                AvailableResources::new_moon(),
                AvailableResources::first_quarter(),
                AvailableResources::full_moon(),
                AvailableResources::last_quarter(),
            ],
        }
    }
    
    /// Retrieves available resources for a specific lunar phase
    ///
    /// # Arguments
    /// * `phase` - Lunar phase to query
    ///
    /// # Returns
    /// Reference to `AvailableResources` for the specified phase
    pub(crate) fn get_available_resources(&self, phase: LunarPhase) -> &AvailableResources {
        match phase {
            LunarPhase::NewMoon => &self.phase_resources[0],
            LunarPhase::FirstQuarter => &self.phase_resources[1],
            LunarPhase::FullMoon => &self.phase_resources[2],
            LunarPhase::LastQuarter => &self.phase_resources[3],
        }
    }
    
    /// Checks if a specific basic resource is available in the current phase
    ///
    /// # Arguments
    /// * `phase` - Current lunar phase
    /// * `resource` - Basic resource type to check
    ///
    /// # Returns
    /// `true` if the resource can be generated in this phase, `false` otherwise
    #[allow(dead_code)]
    pub(crate) fn is_basic_resource_available(
        &self,
        phase: LunarPhase,
        resource: BasicResourceType,
    ) -> bool {
        self.get_available_resources(phase).basic.contains(&resource)
    }
    
    /// Retrieves all available basic resources for a phase
    ///
    /// # Arguments
    /// * `phase` - Lunar phase to query
    ///
    /// # Returns
    /// Clone of the set of available basic resources
    #[allow(dead_code)]
    pub(crate) fn get_available_basic_resources(
        &self,
        phase: LunarPhase,
    ) -> HashSet<BasicResourceType> {
        self.get_available_resources(phase).basic.clone()
    }
    
    /// Generates a human-readable description of a phase with its resources
    ///
    /// # Arguments
    /// * `phase` - Lunar phase to describe
    ///
    /// # Returns
    /// Formatted string describing the phase and available resources
    #[allow(dead_code)]
    pub(crate) fn get_phase_description(&self, phase: LunarPhase) -> String {
        let resources = self.get_available_resources(phase);
        let resource_list: Vec<String> = resources
            .basic
            .iter()
            .map(|r| format!("{:?}", r))
            .collect();
        
        format!(
            "{}: {} | Resources: [{}]",
            phase.name(),
            phase.description(),
            resource_list.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resource_manager_initialization() {
        let manager = ResourceManager::new();
        
        // Check each phase has correct resources
        assert_eq!(
            manager.get_available_basic_resources(LunarPhase::NewMoon),
            HashSet::from([BasicResourceType::Silicon, BasicResourceType::Carbon])
        );
        
        assert_eq!(
            manager.get_available_basic_resources(LunarPhase::FirstQuarter),
            HashSet::from([BasicResourceType::Oxygen, BasicResourceType::Hydrogen])
        );
        
        assert_eq!(
            manager.get_available_basic_resources(LunarPhase::FullMoon).len(),
            4
        );
        
        assert_eq!(
            manager.get_available_basic_resources(LunarPhase::LastQuarter),
            HashSet::from([BasicResourceType::Hydrogen])
        );
    }
    
    #[test]
    fn test_resource_availability_checks() {
        let manager = ResourceManager::new();
        
        // New Moon phase
        assert!(manager.is_basic_resource_available(
            LunarPhase::NewMoon,
            BasicResourceType::Silicon
        ));
        assert!(!manager.is_basic_resource_available(
            LunarPhase::NewMoon,
            BasicResourceType::Oxygen
        ));
        
        // Full Moon phase (everything available)
        assert!(manager.is_basic_resource_available(
            LunarPhase::FullMoon,
            BasicResourceType::Oxygen
        ));
        assert!(manager.is_basic_resource_available(
            LunarPhase::FullMoon,
            BasicResourceType::Silicon
        ));
    }
    
    #[test]
    fn test_no_complex_resources() {
        let manager = ResourceManager::new();
        
        // Verify no complex resources in any phase
        for phase in &[
            LunarPhase::NewMoon,
            LunarPhase::FirstQuarter,
            LunarPhase::FullMoon,
            LunarPhase::LastQuarter,
        ] {
            assert!(manager.get_available_resources(*phase).complex.is_empty());
        }
    }
    
    #[test]
    fn test_phase_descriptions() {
        let manager = ResourceManager::new();
        
        let description = manager.get_phase_description(LunarPhase::NewMoon);
        assert!(description.contains("New Moon"));
        assert!(description.contains("Rare elements in the dark"));
        assert!(description.contains("Silicon"));
        assert!(description.contains("Carbon"));
    }
}