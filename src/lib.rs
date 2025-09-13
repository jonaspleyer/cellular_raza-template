use cellular_raza::building_blocks::{BoundLennardJonesF32, CartesianCuboid, NewtonDamped2DF32};
use cellular_raza::concepts::{
    CalcError, CellAgent, Interaction, InteractionInformation, Mechanics, Position, RngError,
    Velocity,
};

use cellular_raza::core::backend::chili;

use nalgebra::Vector2;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

/// Stores all parameters used to run a simulation.
pub struct SimulationParameters {
    /// Number of agents in the domain
    pub n_agents: usize,
    /// Domain size
    pub domain_size: f32,
    /// Number of voxels used to split up the domain
    pub n_voxels: usize,
    /// Number of threads used
    pub n_threads: usize,
    /// Time increment
    pub dt: f32,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            n_agents: 200,
            domain_size: 30.0,
            n_voxels: 3,
            n_threads: 4,
            dt: 0.002,
        }
    }
}

/// The cellular agent
///
/// It consists of the
/// [NewtonDamped2DF32](https://cellular-raza.com/docs/cellular_raza_building_blocks/struct.NewtonDamped2DF32.html)
/// and
/// [BoundLennardJonesF32](https://cellular-raza.com/docs/cellular_raza_building_blocks/struct.BoundLennardJonesF32.html)
/// structs which make up the
/// [Mechanics](https://cellular-raza.com/docs/cellular_raza_concepts/trait.Mechanics.html)
/// and
/// [Interaction](https://cellular-raza.com/docs/cellular_raza_concepts/trait.Interaction.html) aspects.
#[derive(CellAgent, Clone, Deserialize, Serialize)]
pub struct Agent {
    #[Mechanics]
    pub mechanics: NewtonDamped2DF32,
    #[Interaction]
    pub interaction: BoundLennardJonesF32,
}

/// Completes a full simulation run with the given [SimulationParameters]
pub fn run_simulation(
    simulation_parameters: &SimulationParameters,
) -> Result<(), chili::SimulationError> {
    use rand::Rng;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    // Agents setup
    let agent = Agent {
        mechanics: NewtonDamped2DF32 {
            pos: Vector2::from([0.0, 0.0]),
            vel: Vector2::from([0.0, 0.0]),
            damping_constant: 1.0,
            mass: 1.0,
        },
        interaction: BoundLennardJonesF32 {
            epsilon: 0.01,
            sigma: 1.0,
            bound: 0.1,
            cutoff: 1.0,
        },
    };

    let domain_size = simulation_parameters.domain_size;
    let agents = (0..simulation_parameters.n_agents).map(|_| {
        let mut new_agent = agent.clone();
        new_agent.set_pos(&Vector2::from([
            rng.random_range(0.0..domain_size),
            rng.random_range(0.0..domain_size),
        ]));
        new_agent
    });

    // Domain Setup
    let domain = CartesianCuboid::from_boundaries_and_n_voxels(
        [0.0; 2],
        [simulation_parameters.domain_size; 2],
        [simulation_parameters.n_voxels; 2],
    )?;

    // Storage Setup
    let storage_builder = cellular_raza::prelude::StorageBuilder::new().location("./out");

    // Time Setup
    let t0: f32 = 0.0;
    let dt = simulation_parameters.dt;
    let save_points: Vec<_> = (0..21).map(|n| n as f32).collect();
    let time_stepper = cellular_raza::prelude::time::FixedStepsize::from_partial_save_points(
        t0,
        dt,
        save_points.clone(),
    )?;

    let settings = chili::Settings {
        n_threads: simulation_parameters.n_threads.try_into().unwrap(),
        time: time_stepper,
        storage: storage_builder,
        progressbar: Some("Running Simulation".into()),
    };

    chili::run_simulation!(
        domain: domain,
        agents: agents,
        settings: settings,
        aspects: [Mechanics, Interaction],
    )?;
    Ok(())
}
