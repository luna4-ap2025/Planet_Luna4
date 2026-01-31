//! Comprehensive test suite for Luna4 - The quiet, essential resource moon
//! Tests every module, function, and edge case to ensure robustness.

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;
    use std::time::{Duration, Instant};
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};


    use crate::create_planet;
    use crate::planet::*;
    //use crate::logging::*;
    use crate::planet::cycle::*;
    use crate::planet::energy::*;
    use crate::planet::errors::*;
    use crate::planet::resources::*;
    use crate::planet::state::*;
    use crate::planet::luna4::{Luna4, Luna4Id};

    
    /// Test 1: Luna4 creation and basic properties
    #[test]
    fn test_luna4_creation() {
        // Test Luna4Id
        let id = Luna4Id::new(42);
        assert_eq!(id.as_u32(), 42);
        assert_eq!(format!("{}", id), "Luna4#42");

        // Test from conversion
        let id2: Luna4Id = 99.into();
        assert_eq!(id2.as_u32(), 99);

        // Test Luna4 constructor
        let luna4 = Luna4::new(1).unwrap();
        assert_eq!(luna4.id().as_u32(), 1);
    }

    /// Test 2: Lunar phase functionality
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

    /// Test 4: Energy manager functionality
    #[test]
    fn test_energy_manager() {
        // Test valid construction
        let manager = EnergyManager::new(5).unwrap();
        assert_eq!(manager.total_cells(), 5);

        // Test invalid construction
        let result = EnergyManager::new(4);
        assert!(result.is_err());
        match result {
            Err(Luna4Error::EnergyError(msg)) => {
                assert!(msg.contains("Luna4 must have exactly 5 energy cells"));
            }
            _ => panic!("Expected EnergyError"),
        }

        // Test with 6 cells (also invalid)
        let result = EnergyManager::new(6);
        assert!(result.is_err());

        // Note: We can't easily test charging/using cells without mocking PlanetState
        // since the actual PlanetState implementation is in common_game and may have
        // private fields or require complex setup
    }

    /// Test 5: Resource manager functionality
    #[test]
    fn test_resource_manager() {
        let manager = ResourceManager::new();

        // Test New Moon phase
        let new_moon = manager.get_available_resources(LunarPhase::NewMoon);
        assert_eq!(new_moon.basic.len(), 1);
        assert!(new_moon.basic.contains(&BasicResourceType::Carbon));
        assert!(!new_moon.basic.contains(&BasicResourceType::Oxygen));
        assert!(new_moon.complex.is_empty());

        // Test First Quarter phase
        let first_quarter = manager.get_available_resources(LunarPhase::FirstQuarter);
        assert_eq!(first_quarter.basic.len(), 2);
        assert!(first_quarter.basic.contains(&BasicResourceType::Oxygen));
        assert!(first_quarter.basic.contains(&BasicResourceType::Hydrogen));
        assert!(!first_quarter.basic.contains(&BasicResourceType::Carbon));
        assert!(first_quarter.complex.is_empty());

        // Test Full Moon phase
        let full_moon = manager.get_available_resources(LunarPhase::FullMoon);
        assert_eq!(full_moon.basic.len(), 4);
        assert!(full_moon.basic.contains(&BasicResourceType::Oxygen));
        assert!(full_moon.basic.contains(&BasicResourceType::Hydrogen));
        assert!(full_moon.basic.contains(&BasicResourceType::Carbon));
        assert!(full_moon.basic.contains(&BasicResourceType::Silicon));
        assert!(full_moon.complex.is_empty());

        // Test Last Quarter phase
        let last_quarter = manager.get_available_resources(LunarPhase::LastQuarter);
        assert_eq!(last_quarter.basic.len(), 2);
        assert!(last_quarter.basic.contains(&BasicResourceType::Oxygen));
        assert!(last_quarter.basic.contains(&BasicResourceType::Silicon));
        assert!(!last_quarter.basic.contains(&BasicResourceType::Hydrogen));
        assert!(last_quarter.complex.is_empty());

        // Test PhaseResources builder
        let resources = PhaseResources::basic(vec![
            BasicResourceType::Oxygen,
            BasicResourceType::Hydrogen,
        ]);
        assert_eq!(resources.basic.len(), 2);
        assert!(resources.basic.contains(&BasicResourceType::Oxygen));
        assert!(resources.basic.contains(&BasicResourceType::Hydrogen));
        assert!(resources.complex.is_empty());
    }

    /// Test 6: Luna4State functionality
    #[test]
    fn test_luna4_state() {
        let mut state = Luna4State::new(42);

        assert_eq!(state.id, 42);
        assert_eq!(state.current_phase, LunarPhase::NewMoon);
        assert_eq!(state.explorer_count(), 0);
        assert_eq!(state.present_explorers.len(), 0);

        // Test explorer registration
        state.register_explorer_arrival(1);
        state.register_explorer_arrival(2);
        assert_eq!(state.explorer_count(), 2);
        assert!(state.present_explorers.contains(&1));
        assert!(state.present_explorers.contains(&2));

        // Test duplicate registration
        state.register_explorer_arrival(1);
        assert_eq!(state.explorer_count(), 2); // Should not increase

        // Test explorer departure
        state.register_explorer_departure(1);
        assert_eq!(state.explorer_count(), 1);
        assert!(!state.present_explorers.contains(&1));
        assert!(state.present_explorers.contains(&2));

        // Test non-existent departure
        state.register_explorer_departure(999);
        assert_eq!(state.explorer_count(), 1);

        // Test phase update (simulate time passing)
        // First update should not transition (not enough time passed)
        let old_phase = state.current_phase;
        let result = state.update_phase();
        assert!(result.is_none());
        assert_eq!(state.current_phase, old_phase);

        // Test stats
        assert_eq!(state.stats.successful_generations, 0);
        assert_eq!(state.stats.failed_generations, 0);
        assert_eq!(state.stats.sunrays_received, 0);
        assert_eq!(state.stats.explorer_messages_processed, 0);

        // Test operational stats
        let op_stats = OperationalStats::new();
        assert_eq!(op_stats.total_resources_generated, 0);
        assert_eq!(op_stats.phase_transitions, 0);
        assert_eq!(op_stats.explorer_arrivals, 0);
        assert_eq!(op_stats.explorer_departures, 0);
    }

    /// Test 7: Error types
    #[test]
    fn test_luna4_errors() {
        // Test error display formatting
        let error = Luna4Error::EnergyError("test".to_string());
        assert_eq!(format!("{}", error), "Invalid energy configuration: test");

        let error = Luna4Error::PlanetCreation("creation".to_string());
        assert_eq!(format!("{}", error), "Failed to create planet: creation");

        let error = Luna4Error::ResourceError("gen".to_string());
        assert_eq!(format!("{}", error), "Resource generation failed: gen");

        let error = Luna4Error::PhaseError("timing".to_string());
        assert_eq!(format!("{}", error), "Lunar phase timing error: timing");

        let error = Luna4Error::OperationalError("op".to_string());
        assert_eq!(format!("{}", error), "Operational error: op");

        // Test debug formatting
        let _ = format!("{:?}", error);
    }

    /// Test 8: Statistics tracking
    #[test]
    fn test_luna4_stats() {
        use crate::planet::luna4::stats::Luna4Stats;

        let mut stats = Luna4Stats::new();

        // Test initial state
        assert_eq!(stats.successful_generations, 0);
        assert_eq!(stats.failed_generations, 0);
        assert_eq!(stats.sunrays_received, 0);
        assert_eq!(stats.explorer_messages_processed, 0);

        // Test recording
        stats.record_successful_generation();
        stats.record_successful_generation();
        stats.record_failed_generation();
        stats.record_sunray_received();
        stats.record_sunray_received();
        stats.record_sunray_received();
        stats.record_explorer_message_processed();
        stats.record_explorer_message_processed();
        stats.record_explorer_message_processed();
        stats.record_explorer_message_processed();

        assert_eq!(stats.successful_generations, 2);
        assert_eq!(stats.failed_generations, 1);
        assert_eq!(stats.sunrays_received, 3);
        assert_eq!(stats.explorer_messages_processed, 4);

        // Test success rate calculation
        let rate = stats.generation_success_rate();
        // 2 successes / 3 attempts = 66.666...%
        assert!(rate > 66.6 && rate < 66.7);

        // Test zero attempts
        let empty_stats = Luna4Stats::new();
        assert_eq!(empty_stats.generation_success_rate(), 0.0);

        // Test display summary
        let summary = stats.display_summary();
        assert!(summary.contains("Stats:"));
        assert!(summary.contains("2 successful"));
        assert!(summary.contains("1 failed"));
        assert!(summary.contains("66.7%"));
        assert!(summary.contains("3 sunrays"));
        assert!(summary.contains("4 explorer messages"));
    }

    /// Test 9: Planet creation and wrapper
    #[test]
    fn test_create_planet_function() {
        use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
        use common_game::protocols::planet_explorer::ExplorerToPlanet;

        // Test that create_planet exists and returns proper types
        let (_tx_orch, rx_orch) = unbounded::<OrchestratorToPlanet>();
        let (tx_planet, _rx_planet) = unbounded::<PlanetToOrchestrator>();
        let (_tx_expl, rx_expl) = unbounded::<ExplorerToPlanet>();

        let result = create_planet(1, rx_orch, tx_planet, rx_expl);

        // The function should at least compile and return a Result
        assert!(result.is_ok());
    }

    /// Test 10: Concurrent access and thread safety
    #[test]
    fn test_concurrent_access() {
        use std::thread;

        // Test that Luna4Id can be shared between threads
        let id = Arc::new(Luna4Id::new(1));

        let handles: Vec<_> = (0..10).map(|i| {
            let id_clone = Arc::clone(&id);
            thread::spawn(move || {
                // Each thread reads the ID
                assert_eq!(id_clone.as_u32(), 1);
                format!("Thread {}: {}", i, id_clone)
            })
        }).collect();

        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.contains("Thread"));
            assert!(result.contains("Luna4#1"));
        }
    }

    /// Test 11: Edge cases and error conditions
    #[test]
    fn test_edge_cases() {
        // Test maximum ID value
        let max_id = Luna4Id::new(u32::MAX);
        assert_eq!(max_id.as_u32(), u32::MAX);

        // Test zero ID
        let zero_id = Luna4Id::new(0);
        assert_eq!(zero_id.as_u32(), 0);

        // Test resource generation with all phases
        let manager = ResourceManager::new();

        for phase in &[
            LunarPhase::NewMoon,
            LunarPhase::FirstQuarter,
            LunarPhase::FullMoon,
            LunarPhase::LastQuarter,
        ] {
            let resources = manager.get_available_resources(*phase);
            // All phases should have some basic resources
            assert!(!resources.basic.is_empty());
            // No phases should have complex resources
            assert!(resources.complex.is_empty());
        }

        // Test phase cycling at boundary
        let mut state = Luna4State::new(1);
        state.phase_start_time = Instant::now() - Duration::from_secs(500);
        let old_phase = state.current_phase;
        let transition = state.update_phase();
        assert!(transition.is_some());
        assert_ne!(state.current_phase, old_phase);
    }

    /// Test 12: Performance and timing
    #[test]
    fn test_performance_characteristics() {
        // Test that phase calculations are fast
        let start = Instant::now();

        let cycle = LunarCycle::default();
        for i in 0..1000 {
            let _phase = cycle.phase_at_time(Duration::from_secs(i));
        }

        let duration = start.elapsed();
        // Should complete quickly
        assert!(duration < Duration::from_millis(10));

        // Test state updates are fast
        let mut state = Luna4State::new(1);
        let start = Instant::now();

        for _ in 0..1000 {
            let _ = state.update_phase();
        }

        let duration = start.elapsed();
        assert!(duration < Duration::from_millis(10));
    }

    /// Test 13: Resource availability by phase
    #[test]
    fn test_phase_based_resource_availability() {
        let manager = ResourceManager::new();

        // Map of phases to expected resources
        let expectations = vec![
            (LunarPhase::NewMoon, vec![BasicResourceType::Carbon]),
            (LunarPhase::FirstQuarter, vec![
                BasicResourceType::Oxygen,
                BasicResourceType::Hydrogen,
            ]),
            (LunarPhase::FullMoon, vec![
                BasicResourceType::Oxygen,
                BasicResourceType::Hydrogen,
                BasicResourceType::Carbon,
                BasicResourceType::Silicon,
            ]),
            (LunarPhase::LastQuarter, vec![
                BasicResourceType::Oxygen,
                BasicResourceType::Silicon,
            ]),
        ];

        for (phase, expected_resources) in expectations {
            let available = manager.get_available_resources(phase);

            // Check all expected resources are present
            for resource in &expected_resources {
                assert!(
                    available.basic.contains(resource),
                    "Phase {:?} should contain {:?}",
                    phase, resource
                );
            }

            // Check no unexpected resources
            assert_eq!(
                available.basic.len(),
                expected_resources.len(),
                "Phase {:?} should have exactly {} resources",
                phase,
                expected_resources.len()
            );

            // Never any complex resources
            assert!(available.complex.is_empty());
        }
    }

    /// Test 14: Memory safety and ownership
    #[test]
    fn test_memory_safety() {
        // Test that Luna4 can be moved
        let luna4 = Luna4::new(1).unwrap();
        let moved_luna4 = luna4;
        assert_eq!(moved_luna4.id().as_u32(), 1);

        // Test that EnergyManager can be moved
        let energy = EnergyManager::new(5).unwrap();
        let moved_energy = energy;
        assert_eq!(moved_energy.total_cells(), 5);

        // Test that ResourceManager can be moved
        let resources = ResourceManager::new();
        let _ = resources.clone();

        // Test that Luna4State can be moved
        let mut state = Luna4State::new(1);
        state.register_explorer_arrival(42);
        let moved_state = state;
        assert!(moved_state.present_explorers.contains(&42));
    }

    /// Test 15: No panics on edge inputs
    #[test]
    fn test_no_panics() {
        // Test that functions don't panic on edge cases

        // Test with extremely large durations
        let cycle = LunarCycle::default();
        let large_duration = Duration::from_secs(u64::MAX);
        let _phase = cycle.phase_at_time(large_duration);
        // Should not panic

        // Test with zero duration
        let _phase = cycle.phase_at_time(Duration::from_secs(0));
        // Should not panic

        // Test resource manager with all phases
        let manager = ResourceManager::new();
        for phase in &[
            LunarPhase::NewMoon,
            LunarPhase::FirstQuarter,
            LunarPhase::FullMoon,
            LunarPhase::LastQuarter,
        ] {
            let _resources = manager.get_available_resources(*phase);
            // Should not panic
        }
    }

    /// Test 16: Constant validation
    #[test]
    fn test_constants() {
        // Verify lunar cycle duration matches documentation
        let cycle = LunarCycle::default();
        assert_eq!(cycle.total_cycle_seconds, 420); // 7 minutes

        // Verify phase duration
        let phase_duration = cycle.phase_duration(LunarPhase::NewMoon);
        assert_eq!(phase_duration.as_secs(), 105); // 420 / 4 = 105
    }

    /// Test 17: Module visibility and exports
    #[test]
    fn test_module_visibility() {
        use luna4::Luna4Id;
        use crate::planet::{Luna4, LunarPhase};

        // These should compile
        let _id: Luna4Id = 1.into();
        let _phase = LunarPhase::NewMoon;

        // Luna4 constructor should be accessible
        let _luna4_result = Luna4::new(1);

        // Common game types should be re-exported
        use crate::planet::{PlanetType, BasicResourceType};

        let _planet_type = PlanetType::D;
        let _resource_type = BasicResourceType::Oxygen;
    }

    /// Test 18: Serialization and display traits
    #[test]
    fn test_trait_implementations() {
        // Test Display for Luna4Id
        let id = Luna4Id::new(42);
        assert_eq!(format!("{}", id), "Luna4#42");

        // Test Debug for all public types (should compile)
        let luna4 = Luna4::new(1).unwrap();
        let _debug_output = format!("{:?}", luna4);

        let phase = LunarPhase::NewMoon;
        let _debug_output = format!("{:?}", phase);

        // Test Clone for necessary types
        let id_clone = id.clone();
        assert_eq!(id_clone.as_u32(), 42);

        let phase_clone = phase.clone();
        assert_eq!(phase_clone, LunarPhase::NewMoon);

        // Test Eq and PartialEq
        let id1 = Luna4Id::new(1);
        let id2 = Luna4Id::new(1);
        let id3 = Luna4Id::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);

        // Test Hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    /// Test 19: Documentation examples compile
    #[test]
    fn test_documentation_examples() {
        // Test that the examples in documentation would compile

        // From lib.rs:
        // ```
        // let (_tx_orch, rx_orch) = unbounded::<OrchestratorToPlanet>();
        // let (tx_planet, _rx_planet) = unbounded::<PlanetToOrchestrator>();
        // let (_tx_expl, rx_expl) = unbounded::<ExplorerToPlanet>();
        // let _result = create_planet(1, rx_orch, tx_planet, rx_expl);
        // ```
        // This is already tested in test_create_planet_function

        // From cycle.rs:
        let phase = LunarPhase::NewMoon;
        assert_eq!(phase.name(), "New Moon");
        assert_eq!(phase.next(), LunarPhase::FirstQuarter);

        // From energy.rs:
        let manager = EnergyManager::new(5);
        assert!(manager.is_ok());

        // From errors.rs:
        let error = Luna4Error::EnergyError("test".to_string());
        assert!(format!("{}", error).contains("test"));
    }

    /// Test 20: Complete lifecycle test
    #[test]
    fn test_complete_lifecycle() {
        use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
        use common_game::protocols::planet_explorer::ExplorerToPlanet;

        // This test simulates a complete lifecycle of a Luna4 planet

        // 1. Create the planet
        let (tx_orch, rx_orch) = unbounded::<OrchestratorToPlanet>();
        let (tx_planet, _rx_planet) = unbounded::<PlanetToOrchestrator>();
        let (_tx_expl, rx_expl) = unbounded::<ExplorerToPlanet>();

        let planet_result = create_planet(1, rx_orch, tx_planet, rx_expl);
        assert!(planet_result.is_ok());

        // 2. The planet would normally be spawned in a thread and run
        // For testing purposes, we just verify creation succeeded
    }
}