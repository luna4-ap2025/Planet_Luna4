#[cfg(test)]
mod tests {
    use std::time::Duration;
    use common_game::components::resource::BasicResourceType;
    use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
    use common_game::protocols::planet_explorer::ExplorerToPlanet;
    use super::*;
    use crossbeam_channel::unbounded;
    use crate::{create_planet, Luna4};
    use crate::planet::{EnergyManager, LunarCycle, LunarPhase, ResourceManager};
    use crate::planet::errors::Luna4Error;
    use crate::planet::luna4::Luna4Stats;

    #[test]
    fn test_create_planet_function_exists() {
        // Verify the function signature compiles
        let (_tx_orch, rx_orch) = unbounded::<common_game::protocols::orchestrator_planet::OrchestratorToPlanet>();
        let (tx_planet, _rx_planet) = unbounded::<common_game::protocols::orchestrator_planet::PlanetToOrchestrator>();
        let (_tx_expl, rx_expl) = unbounded::<common_game::protocols::planet_explorer::ExplorerToPlanet>();

        let _result = create_planet(1, rx_orch, tx_planet, rx_expl);
    }
    #[test]
    fn test_phase_transitions() {
        assert_eq!(LunarPhase::NewMoon.next(), LunarPhase::FirstQuarter);
        assert_eq!(LunarPhase::FirstQuarter.next(), LunarPhase::FullMoon);
        assert_eq!(LunarPhase::FullMoon.next(), LunarPhase::LastQuarter);
        assert_eq!(LunarPhase::LastQuarter.next(), LunarPhase::NewMoon);
    }

    #[test]
    fn test_energy_manager_validation() {
        assert!(EnergyManager::new(5).is_ok());
        assert!(EnergyManager::new(4).is_err());
    }

    #[test]
    fn test_error_display() {
        let error = Luna4Error::EnergyError("test error".to_string());
        assert_eq!(format!("{}", error), "Invalid energy configuration: test error");

        let error = Luna4Error::PlanetCreation("creation failed".to_string());
        assert_eq!(format!("{}", error), "Failed to create planet: creation failed");
    }

    #[test]
    fn test_error_debug() {
        let error = Luna4Error::ResourceError("cannot generate".to_string());
        // Should not panic when formatting for debug
        let _ = format!("{:?}", error);
    }

    #[test]
    fn test_resource_manager_phase_resources() {
        let manager = ResourceManager::new();

        // New Moon: Carbon only
        let new_moon = manager.get_available_resources(LunarPhase::NewMoon);
        assert_eq!(new_moon.basic.len(), 1);
        assert!(new_moon.basic.contains(&BasicResourceType::Carbon));

        // Full Moon: All resources
        let full_moon = manager.get_available_resources(LunarPhase::FullMoon);
        assert_eq!(full_moon.basic.len(), 4);

        // No complex resources in any phase
        for phase in &[LunarPhase::NewMoon, LunarPhase::FirstQuarter, LunarPhase::FullMoon, LunarPhase::LastQuarter] {
            let resources = manager.get_available_resources(*phase);
            assert!(resources.complex.is_empty());
        }
    }

    #[test]
    fn test_luna4_constructor() {
        let luna4 = Luna4::new(1);
        assert!(luna4.is_ok());
        let luna4 = luna4.unwrap();
        assert_eq!(luna4.id().as_u32(), 1);
    }

    #[test]
    fn test_stats_recording() {
        let mut stats = Luna4Stats::new();

        stats.record_successful_generation();
        stats.record_sunray_received();
        stats.record_explorer_message_processed();

        assert_eq!(stats.successful_generations, 1);
        assert_eq!(stats.sunrays_received, 1);
        assert_eq!(stats.explorer_messages_processed, 1);
    }

    #[test]
    fn test_lunar_phases() {
        // Test phase names and descriptions
        assert_eq!(LunarPhase::NewMoon.name(), "New Moon");
        assert_eq!(LunarPhase::FirstQuarter.name(), "First Quarter");
        assert_eq!(LunarPhase::FullMoon.name(), "Full Moon");
        assert_eq!(LunarPhase::LastQuarter.name(), "Last Quarter");

        assert_eq!(LunarPhase::NewMoon.description(), "Rare elements in the dark");
        assert_eq!(LunarPhase::FirstQuarter.description(), "Common ones in the light");
        assert_eq!(LunarPhase::FullMoon.description(), "Everything at full moon");
        assert_eq!(LunarPhase::LastQuarter.description(), "Preparation for next cycle");

        // Test phase duration
        assert_eq!(LunarPhase::NewMoon.duration_seconds(), 105);
        assert_eq!(LunarPhase::FirstQuarter.duration_seconds(), 105);
        assert_eq!(LunarPhase::FullMoon.duration_seconds(), 105);
        assert_eq!(LunarPhase::LastQuarter.duration_seconds(), 105);

        // Test phase transitions
        assert_eq!(LunarPhase::NewMoon.next(), LunarPhase::FirstQuarter);
        assert_eq!(LunarPhase::FirstQuarter.next(), LunarPhase::FullMoon);
        assert_eq!(LunarPhase::FullMoon.next(), LunarPhase::LastQuarter);
        assert_eq!(LunarPhase::LastQuarter.next(), LunarPhase::NewMoon);

        // Test all transitions cycle back
        let mut phase = LunarPhase::NewMoon;
        for _ in 0..8 { // Two full cycles
            phase = phase.next();
        }
        assert_eq!(phase, LunarPhase::NewMoon);
    }

    /// Test 3: Lunar cycle management
    #[test]
    fn test_lunar_cycle() {
        let cycle = LunarCycle::default();

        // Test default duration
        assert_eq!(cycle.total_cycle_seconds, 420);

        // Test phase duration calculation
        let duration = cycle.phase_duration(LunarPhase::NewMoon);
        assert_eq!(duration.as_secs(), 105);

        // Test phase at time
        assert_eq!(cycle.phase_at_time(Duration::from_secs(0)), LunarPhase::NewMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(104)), LunarPhase::NewMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(105)), LunarPhase::FirstQuarter);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(209)), LunarPhase::FirstQuarter);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(210)), LunarPhase::FullMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(314)), LunarPhase::FullMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(315)), LunarPhase::LastQuarter);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(419)), LunarPhase::LastQuarter);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(420)), LunarPhase::NewMoon);
        assert_eq!(cycle.phase_at_time(Duration::from_secs(525)), LunarPhase::FirstQuarter);

        // Test custom cycle
        let custom_cycle = LunarCycle::new(200);
        assert_eq!(custom_cycle.total_cycle_seconds, 200);
        assert_eq!(custom_cycle.phase_duration(LunarPhase::NewMoon).as_secs(), 50);
    }

    #[test]
    fn test_create_planet_function_exists() {
        // Verify the function signature compiles
        let (_tx_orch, rx_orch) = unbounded::<common_game::protocols::orchestrator_planet::OrchestratorToPlanet>();
        let (tx_planet, _rx_planet) = unbounded::<common_game::protocols::orchestrator_planet::PlanetToOrchestrator>();
        let (_tx_expl, rx_expl) = unbounded::<common_game::protocols::planet_explorer::ExplorerToPlanet>();

        let _result = create_planet(1, rx_orch, tx_planet, rx_expl);
    }

    #[test]
    fn test_luna4_phase_resources_mapping() {
        use crate::planet::resources::ResourceManager;
        use crate::planet::cycle::LunarPhase;

        let manager = ResourceManager::new();

        // Test each phase
        let new_moon = manager.get_available_resources(LunarPhase::NewMoon);
        assert_eq!(new_moon.basic.len(), 1);
        assert!(new_moon.basic.contains(&BasicResourceType::Carbon));

        let first_quarter = manager.get_available_resources(LunarPhase::FirstQuarter);
        assert_eq!(first_quarter.basic.len(), 2);
        assert!(first_quarter.basic.contains(&BasicResourceType::Oxygen));
        assert!(first_quarter.basic.contains(&BasicResourceType::Hydrogen));

        let full_moon = manager.get_available_resources(LunarPhase::FullMoon);
        assert_eq!(full_moon.basic.len(), 4);

        let last_quarter = manager.get_available_resources(LunarPhase::LastQuarter);
        assert_eq!(last_quarter.basic.len(), 2);
        assert!(last_quarter.basic.contains(&BasicResourceType::Oxygen));
        assert!(last_quarter.basic.contains(&BasicResourceType::Silicon));
    }
}






