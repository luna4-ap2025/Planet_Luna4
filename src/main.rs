mod ai; // importa ai.rs

use common_game::components::planet::{Planet, PlanetType};
use common_game::components::energy_cell::EnergyCell;
use common_game::components::resource::{BasicResourceType, Combinator, ComplexResourceType, Generator};
use common_game::components::rocket::Rocket;
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel::select_biased;

use ai::Luna4AI;

fn main() {
    let ai = Box::new(Luna4AI);

    // Creazione dei canali con crossbeam
    let (orch_tx, orch_rx) = crossbeam_channel::unbounded();
    let (expl_tx, expl_rx) = crossbeam_channel::unbounded();

    let mut planet = Planet::new(
        1,
        PlanetType::A,
        ai,
        vec![BasicResourceType::Carbon],
        vec![],
        (orch_tx, orch_rx),
        (expl_tx, expl_rx),
    ).expect("Planet validation failed");

    planet.run().unwrap();
}
