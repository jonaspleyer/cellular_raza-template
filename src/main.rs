use cellular_raza_template::*;

fn main() -> Result<(), cellular_raza::prelude::SimulationError> {
    let parameters = SimulationParameters::default();
    run_simulation(&parameters)?;
    Ok(())
}
