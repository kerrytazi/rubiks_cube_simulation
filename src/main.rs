mod simulation;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	#[cfg(debug_assertions)]
	std::panic::set_hook(Box::new(|panic_info| {
		eprintln!("Panic info: {}", panic_info);

		#[cfg(target_arch = "x86_64")]
		unsafe {
			std::arch::asm!("int3");
		}
	}));

	let simulation = simulation::SimulationWindow::new("Rubiks Cube Simulation")?;
	simulation.window_loop();
	Ok(())
}
