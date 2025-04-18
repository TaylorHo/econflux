mod conditions;
mod export;
mod initial_state;
mod normalization;
mod population;
mod simulation;

use conditions::demo_condition;
use export::demo_export;
use initial_state::demo_initial_state;
use normalization::demo_normalize;
use population::demo_population;
use simulation::demo_simulate;

fn main() {
    println!("EconFlux - Statistical Physics for Economy and Society");

    // Demo function calls
    demo_condition();
    demo_population();
    demo_initial_state();
    demo_normalize();
    demo_simulate();
    demo_export();
}
