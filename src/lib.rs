#![cfg(target_arch = "wasm32")]

mod simulation;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main_web() {
	console_error_panic_hook::set_once();

	let simulation = simulation::SimulationWindow::new("Rubiks Cube Simulation Web").unwrap();
	simulation.window_loop();
}
