use common_game::components::planet::{PlanetAI, PlanetState};
use common_game::components::resource::{Generator, Combinator};
use common_game::protocols::messages::*;
use common_game::components::rocket::Rocket;

pub struct Luna4AI;

impl PlanetAI for Luna4AI {
    fn start(&mut self, _state: &PlanetState) {
        println!("Planet started");
    }

    fn stop(&mut self, _state: &PlanetState) {
        println!("Planet stopped");
    }

    fn handle_orchestrator_msg(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        _msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        None
    }

    fn handle_explorer_msg(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        _msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        None
    }

    fn handle_asteroid(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> Option<Rocket> {
        None
    }
}
