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

/// Tracks the current state of Luna4's energy system
///
/// This struct provides a snapshot view of energy cell status
/// for monitoring and logging purposes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct EnergyStatus {
    /// Total number of energy cells (always 5 for Luna4)
    pub(crate) total_cells: usize,
    /// Number of currently charged cells
    pub(crate) charged_cells: usize,
    /// Number of currently discharged cells
    pub(crate) discharged_cells: usize,
    /// Percentage of charged cells (0.0 to 100.0)
    pub(crate) percentage_charged: f32,
}

impl EnergyStatus {
    /// Creates a new energy status from cell counts
    ///
    /// # Arguments
    /// * `total_cells` - Total number of energy cells
    /// * `charged_cells` - Number of currently charged cells
    ///
    /// # Returns
    /// New `EnergyStatus` instance
    #[allow(dead_code)]
    pub(crate) fn new(total_cells: usize, charged_cells: usize) -> Self {
        let discharged_cells = total_cells.saturating_sub(charged_cells);
        let percentage_charged = if total_cells > 0 {
            (charged_cells as f32 / total_cells as f32) * 100.0
        } else {
            0.0
        };
        
        Self {
            total_cells,
            charged_cells,
            discharged_cells,
            percentage_charged,
        }
    }
    
    /// Creates a display-friendly summary of the energy status
    ///
    /// # Returns
    /// Formatted string showing energy cell status
    #[allow(dead_code)]
    pub(crate) fn display_summary(&self) -> String {
        format!(
            "Energy: {}/{} cells charged ({:.1}%)",
            self.charged_cells,
            self.total_cells,
            self.percentage_charged
        )
    }
}

/// Manages Luna4's energy cells with constraint validation
///
/// This struct enforces Luna4's energy constraints (exactly 5 cells)
/// and provides operations for charging, discharging, and energy management.
#[derive(Debug)]
pub(crate) struct EnergyManager {
    /// Number of energy cells (must be 5 for Luna4)
    cell_count: usize,
}

impl EnergyManager {
    /// Creates a new energy manager for Luna4
    ///
    /// # Arguments
    /// * `cell_count` - Number of energy cells (must be 5)
    ///
    /// # Returns
    /// `Result<Self, Luna4Error>` - Energy manager instance
    ///
    /// # Errors
    /// Returns `Luna4Error::EnergyError` if cell count is not exactly 5
    pub(crate) fn new(cell_count: usize) -> Result<Self, Luna4Error> {
        if cell_count != LUNA4_ENERGY_CELL_COUNT {
            return Err(Luna4Error::EnergyError(format!(
                "Luna4 must have exactly {} energy cells, got {}",
                LUNA4_ENERGY_CELL_COUNT, cell_count
            )));
        }
        
        Ok(Self { cell_count })
    }
    
    /// Charges an energy cell using a sunray
    ///
    /// Attempts to charge the first discharged cell found.
    /// If all cells are already charged, the sunray is wasted (normal operation).
    ///
    /// # Arguments
    /// * `sunray` - Sunray to use for charging
    /// * `state` - Planet state containing energy cells
    ///
    /// # Returns
    /// `Result<(), Luna4Error>` - Success or error indication
    pub(crate) fn charge_cell(
        &self,
        sunray: Sunray,
        state: &mut PlanetState,
    ) -> Result<(), Luna4Error> {
        // Find first discharged cell
        for i in 0..self.cell_count {
            let cell = state.cell_mut(i);
            if !cell.is_charged() {
                cell.charge(sunray);
                return Ok(());
            }
        }
        
        // All cells are charged - sunray is wasted (normal operation)
        Ok(())
    }
    
    /// Uses an energy cell to perform an operation
    ///
    /// # Arguments
    /// * `state` - Planet state containing energy cells
    /// * `operation` - Function to execute with a charged energy cell
    ///
    /// # Returns
    /// `Result<T, Luna4Error>` - Result of the operation or error
    ///
    /// # Errors
    /// Returns `Luna4Error::EnergyError` if no charged cells are available
    pub(crate) fn use_energy_cell<F, T>(
        &self,
        state: &mut PlanetState,
        operation: F,
    ) -> Result<T, Luna4Error>
    where
        F: FnOnce(&mut EnergyCell) -> Result<T, String>,
    {
        // Find first charged cell
        for i in 0..self.cell_count {
            let cell = state.cell_mut(i);
            if cell.is_charged() {
                return operation(cell).map_err(Luna4Error::EnergyError);
            }
        }
        
        Err(Luna4Error::EnergyError(
            "No charged energy cells available".to_string()
        ))
    }
    
    /// Returns the number of currently charged energy cells
    ///
    /// # Arguments
    /// * `state` - Planet state containing energy cells
    ///
    /// # Returns
    /// Number of charged cells (0-5)
    pub(crate) fn available_charged_cells(&self, state: &PlanetState) -> usize {
        (0..self.cell_count)
            .filter(|&i| state.cell(i).is_charged())
            .count()
    }
    
    /// Returns the number of currently discharged energy cells
    ///
    /// # Arguments
    /// * `state` - Planet state containing energy cells
    ///
    /// # Returns
    /// Number of discharged cells (0-5)
    #[allow(dead_code)]
    pub(crate) fn available_discharged_cells(&self, state: &PlanetState) -> usize {
        (0..self.cell_count)
            .filter(|&i| !state.cell(i).is_charged())
            .count()
    }
    
    /// Checks if sufficient energy is available for an operation
    ///
    /// # Arguments
    /// * `state` - Planet state containing energy cells
    /// * `required_cells` - Number of charged cells required
    ///
    /// # Returns
    /// `true` if at least `required_cells` are charged, `false` otherwise
    #[allow(dead_code)]
    pub(crate) fn has_sufficient_energy(&self, state: &PlanetState, required_cells: usize) -> bool {
        self.available_charged_cells(state) >= required_cells
    }
    
    /// Returns complete energy status information
    ///
    /// # Arguments
    /// * `state` - Planet state containing energy cells
    ///
    /// # Returns
    /// Detailed energy status
    #[allow(dead_code)]
    pub(crate) fn get_energy_status(&self, state: &PlanetState) -> EnergyStatus {
        let charged = self.available_charged_cells(state);
        EnergyStatus::new(self.cell_count, charged)
    }
    
    /// Returns the total number of energy cells (always 5 for Luna4)
    ///
    /// # Returns
    /// Number of energy cells
    #[allow(dead_code)]
    pub(crate) fn total_cells(&self) -> usize {
        self.cell_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_energy_manager_validation() {
        // Should accept exactly 5 cells
        assert!(EnergyManager::new(5).is_ok());
        
        // Should reject incorrect cell counts
        assert!(EnergyManager::new(4).is_err());
        assert!(EnergyManager::new(6).is_err());
        assert!(EnergyManager::new(0).is_err());
    }
    
    #[test]
    fn test_energy_status_calculation() {
        let status = EnergyStatus::new(5, 3);
        
        assert_eq!(status.total_cells, 5);
        assert_eq!(status.charged_cells, 3);
        assert_eq!(status.discharged_cells, 2);
        assert!((status.percentage_charged - 60.0).abs() < 0.001);
        
        // Test display summary
        let summary = status.display_summary();
        assert!(summary.contains("3/5"));
        assert!(summary.contains("60.0%"));
    }
    
    #[test]
    fn test_cell_count_method() {
        let manager = EnergyManager::new(5).unwrap();
        assert_eq!(manager.total_cells(), 5);
    }
    
    #[test]
    fn test_edge_cases() {
        // Test with zero charged cells
        let status = EnergyStatus::new(5, 0);
        assert_eq!(status.percentage_charged, 0.0);
        
        // Test with all cells charged
        let status = EnergyStatus::new(5, 5);
        assert_eq!(status.percentage_charged, 100.0);
        
        // Test saturation for discharged cells calculation
        let status = EnergyStatus::new(5, 10); // More charged than total
        assert_eq!(status.discharged_cells, 0); // Should saturate to 0
    }
}