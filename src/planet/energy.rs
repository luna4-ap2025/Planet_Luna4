//! Energy cell management for Luna4
//!
//! Manages the 5 energy cells with intelligent charging/discharging logic
//! specific to Luna4's operational requirements as a Type D planet.

use super::errors::Luna4Error;
use common_game::components::energy_cell::EnergyCell;
use common_game::components::planet::PlanetState;
use common_game::components::sunray::Sunray;

/// Number of energy cells in a Luna4 planet (fixed by Type D specification)
const LUNA4_ENERGY_CELL_COUNT: usize = 5;

/// Manages Luna4's energy cells with constraint validation
#[derive(Debug)]
pub struct EnergyManager {
    /// Number of energy cells (must be 5 for Luna4)
    cell_count: usize,
}

impl EnergyManager {
    /// Creates a new energy manager for Luna4
    pub fn new(cell_count: usize) -> Result<Self, Luna4Error> {
        if cell_count != LUNA4_ENERGY_CELL_COUNT {
            return Err(Luna4Error::EnergyError(format!(
                "Luna4 must have exactly {} energy cells, got {}",
                LUNA4_ENERGY_CELL_COUNT, cell_count
            )));
        }

        Ok(Self { cell_count })
    }

    /// Charges an energy cell using a sunray
    pub fn charge_cell(
        &self,
        sunray: Sunray,
        state: &mut PlanetState,
    ) -> Result<(), Luna4Error> {
        match state.charge_cell(sunray) {
            Some(_wasted_sunray) => {
                // All cells were already charged - sunray wasted (normal operation)
                Ok(())
            }
            None => Ok(()), // Successfully charged a cell
        }
    }

    /// Uses an energy cell to perform an operation
    pub fn use_energy_cell<F, T>(
        &self,
        state: &mut PlanetState,
        operation: F,
    ) -> Result<T, Luna4Error>
    where
        F: FnOnce(&mut EnergyCell) -> Result<T, String>,
    {
        // Find first charged cell
        if let Some((cell, _index)) = state.full_cell() {
            operation(cell).map_err(|e| Luna4Error::EnergyError(e))
        } else {
            Err(Luna4Error::EnergyError(
                "No charged energy cells available".to_string()
            ))
        }
    }

    /// Returns the number of currently charged energy cells
    pub fn available_charged_cells(&self, state: &PlanetState) -> usize {
        (0..self.cell_count)
            .filter(|&i| state.cell(i).is_charged())
            .count()
    }

    /// Returns the total number of energy cells (always 5 for Luna4)
    pub fn total_cells(&self) -> usize {
        self.cell_count
    }
}
