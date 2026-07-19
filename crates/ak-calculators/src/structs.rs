use ak_core::StructureView;

#[derive(Debug)]
pub enum CalculatorError {
    NotImplemented,
    InvalidStructure,
    CalculationFailed,
}

pub struct CalculatorResult {
    pub energy: Option<f64>,
    pub forces: Option<Vec<[f64; 3]>>,
}

pub trait Calculator {
    fn calculate_energy(&self, view: &StructureView) -> Result<f64, CalculatorError>;

    fn calculate_forces(&self, view: &StructureView) -> Result<Vec<[f64; 3]>, CalculatorError>;

    fn calculate(&self, view: &StructureView) -> Result<CalculatorResult, CalculatorError> {
        Ok(CalculatorResult {
            energy: Some(self.calculate_energy(view)?),
            forces: Some(self.calculate_forces(view)?),
        })
    }
}
