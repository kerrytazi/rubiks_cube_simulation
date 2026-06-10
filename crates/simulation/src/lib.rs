mod anim_function;
mod cube_colors;
mod cube;
mod keyframe;
mod rubiks_action;
mod rubiks_cube;
mod solver;
mod utils;

use rubiks_action::RubiksAction;
use rubiks_cube::RubiksCube;

use std::{sync::{Arc, atomic::AtomicBool}};

use rand::RngExt;
use three_d::*;

const DEFAULT_ANIMATION_DURATION_MS: f64 = 125.0;
const DEFAULT_WIDE_NOTATION_W: bool = false;

struct SolverThread {
	thread: std::thread::JoinHandle<Option<Vec<RubiksAction>>>,
	cancel_flag: Arc<AtomicBool>,
}

struct Simulation {
	context: Context,
	camera: Camera,
	camera_text: Camera,
	control: OrbitControl,

	rubiks: RubiksCube,
	history: Vec::<RubiksAction>,
	past_active_history_item: usize,
	animations: std::collections::VecDeque<RubiksAction>,

	pressed_keys: std::collections::HashSet<Key>,

	text_generator: TextGenerator<'static>,
	axes: Gm<InstancedMesh, ColorMaterial>,
	history_text_left: Gm<Mesh, ColorMaterial>,
	history_text_right: Gm<Mesh, ColorMaterial>,
	solver_text: Gm<Mesh, ColorMaterial>,

	solver_thread: Option<SolverThread>,

	show_axes: bool,
}

impl Simulation {
	fn render(&mut self, frame_input: &mut FrameInput) -> FrameOutput {
		self.camera.set_viewport(frame_input.viewport);
		self.camera_text.set_viewport(frame_input.viewport);

		self.control.handle_events(&mut self.camera, &mut frame_input.events);

		self.update_animations(frame_input.accumulated_time);

		for event in &frame_input.events {
			self.handle_event(&event, frame_input.accumulated_time);
		}

		if let Some(prev_solver_thread) = self.solver_thread.take_if(|prev| prev.thread.is_finished()) {
			if let Ok(thread_result) = prev_solver_thread.thread.join() {
				if let Some(actions) = thread_result {
					let mut text_actions = String::new();

					for action in actions {
						text_actions.push_str(action.to_string_notation(DEFAULT_WIDE_NOTATION_W));
						text_actions.push_str(" ");
					}

					self.solver_text = self.recreate_text(&text_actions, Srgba::new_opaque(0, 128, 0));
				} else {
					self.solver_text = self.recreate_text("No solutions", Srgba::RED);
				}
			} else {
				self.solver_text = self.recreate_text("Solver thread panicked", Srgba::RED);
			}

			let prev_transform = self.solver_text.transformation();
			self.solver_text.set_transformation(Mat4::from_translation(vec3(0.0, -30.0, 0.0)) * prev_transform);
		}

		let screen = frame_input.screen();

		screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));

		screen.render(&self.camera, self.rubiks.into_iter(), &[]);

		if self.show_axes {
			screen.render(&self.camera, self.axes.into_iter(), &[]);
		}

		screen.render(&self.camera_text, [&self.history_text_left, &self.history_text_right, &self.solver_text], &[]);

		FrameOutput::default()
	}

	fn handle_keypress_event(&mut self, event: &Event, current_time: f64) {
		let key = if let Event::KeyPress { kind, .. } = event {
			kind
		} else {
			panic!("handle_keypress_event");
		};

		if !self.pressed_keys.insert(*key) {
			return;
		}

		match event {
			Event::KeyPress { kind: Key::L, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Left{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::R, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Right{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::D, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Down{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::U, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Up{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::B, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Back{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::F, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Front{ prime: modifiers.shift, wide: modifiers.alt });
			},
			Event::KeyPress { kind: Key::M, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Middle{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::E, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Equatorial{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::S, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::Standing{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::X, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::RotateCubeX{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::Y, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::RotateCubeY{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::Z, modifiers, .. } => {
				self.add_new_action(current_time, RubiksAction::RotateCubeZ{ prime: modifiers.shift });
			},
			Event::KeyPress { kind: Key::Tab, .. } => {
				self.show_axes = !self.show_axes;
			},
			Event::KeyPress { kind: Key::ArrowLeft, modifiers, .. } => {
				if modifiers.ctrl {
					while self.try_move_history_back(current_time) {}
				} else {
					self.try_move_history_back(current_time);
				}
			},
			Event::KeyPress { kind: Key::ArrowRight, modifiers, .. } => {
				if modifiers.ctrl {
					while self.try_move_history_forward(current_time) {}
				} else {
					self.try_move_history_forward(current_time);
				}
			},
			Event::KeyPress { kind: Key::F1, .. } => {
				self.reset_rubiks();
			},
			Event::KeyPress { kind: Key::F2, .. } => {
				self.shuffle(20);
			},
			_ => {},
		}
	}

	fn handle_keyrelease_event(&mut self, event: &Event, _current_time: f64) {
		let key = if let Event::KeyRelease { kind, .. } = event {
			kind
		} else {
			panic!("handle_keyrelease_event");
		};

		self.pressed_keys.remove(key);
	}

	fn handle_event(&mut self, event: &Event, current_time: f64) {
		match event {
			Event::KeyPress { .. } => self.handle_keypress_event(event, current_time),
			Event::KeyRelease { .. } => self.handle_keyrelease_event(event, current_time),
			_ => {},
		}
	}

	fn try_move_history_back(&mut self, current_time: f64) -> bool {
		if self.past_active_history_item > 0 {
			self.add_animation(current_time, self.history[self.past_active_history_item - 1].inverse());
			self.past_active_history_item -= 1;
			self.recreate_history();
			self.recreate_solver(true);
			true
		} else {
			false
		}
	}

	fn try_move_history_forward(&mut self, current_time: f64) -> bool {
		if self.past_active_history_item < self.history.len() {
			self.past_active_history_item += 1;
			self.add_animation(current_time, self.history[self.past_active_history_item - 1]);
			self.recreate_history();
			self.recreate_solver(true);
			true
		} else {
			false
		}
	}

	fn reset_rubiks(&mut self) {
		self.history.clear();
		self.past_active_history_item = 0;
		self.recreate_history();
		self.recreate_solver(false);
		self.rubiks = RubiksCube::new(&self.context);
	}

	fn shuffle(&mut self, moves_count: usize) {
		let mut rng = rand::rng();

		for _ in 0..moves_count {
			match rng.random_range(0..9) {
				0 => self.rubiks.rotate_x(0, rng.random_bool(0.5), 0.0, 0.0),
				1 => self.rubiks.rotate_x(1, rng.random_bool(0.5), 0.0, 0.0),
				2 => self.rubiks.rotate_x(2, rng.random_bool(0.5), 0.0, 0.0),
				3 => self.rubiks.rotate_y(0, rng.random_bool(0.5), 0.0, 0.0),
				4 => self.rubiks.rotate_y(1, rng.random_bool(0.5), 0.0, 0.0),
				5 => self.rubiks.rotate_y(2, rng.random_bool(0.5), 0.0, 0.0),
				6 => self.rubiks.rotate_z(0, rng.random_bool(0.5), 0.0, 0.0),
				7 => self.rubiks.rotate_z(1, rng.random_bool(0.5), 0.0, 0.0),
				8 => self.rubiks.rotate_z(2, rng.random_bool(0.5), 0.0, 0.0),
				_ => { panic!("rubik shuffle out of bounds"); },
			}
		}
	}

	fn update_animations(&mut self, current_time: f64) {
		if self.rubiks.update_animations(current_time) {
			self.animations.pop_front().unwrap();

			if let Some(next_action) = self.animations.front() {
				self.rubiks.apply(*next_action, current_time, DEFAULT_ANIMATION_DURATION_MS);
			}
		}
	}

	fn add_animation(&mut self, current_time: f64, action: RubiksAction) {
		if self.animations.len() == 0 {
			self.rubiks.apply(action, current_time, DEFAULT_ANIMATION_DURATION_MS);
		}

		if DEFAULT_ANIMATION_DURATION_MS != 0.0 {
			self.animations.push_back(action);
		}
	}

	fn add_new_action(&mut self, current_time: f64, action: RubiksAction) {
		self.history.truncate(self.past_active_history_item);
		self.history.push(action);
		self.add_animation(current_time, action);
		self.past_active_history_item += 1;
		self.recreate_history();
		self.recreate_solver(true);
	}

	fn recreate_text(&mut self, s: &str, color: Srgba) -> Gm<Mesh, ColorMaterial> {
		let text_mesh = self.text_generator.generate(s, TextLayoutOptions::default());

		let mut text = Gm::new(
			Mesh::new(&self.context, &text_mesh),
			ColorMaterial {
				color,
				..Default::default()
			},
		);
		text.set_transformation(
			Mat4::from_translation(vec3(
				5.0,
				self.camera_text.viewport().height as f32 - 35.0,
				0.0,
			)),
		);

		text
	}

	fn recreate_solver(&mut self, try_solve: bool) {
		if cfg!(target_arch = "wasm32") {
			return;
		}

		if let Some(prev_solver_thread) = self.solver_thread.take() {
			prev_solver_thread.cancel_flag.store(true, std::sync::atomic::Ordering::SeqCst);
		}

		let solver = self.rubiks.to_solver();

		if solver.is_solved() {
			self.solver_text = self.recreate_text("Solved", Srgba::new_opaque(0, 128, 0));
			let prev_transform = self.solver_text.transformation();
			self.solver_text.set_transformation(Mat4::from_translation(vec3(0.0, -30.0, 0.0)) * prev_transform);
			return;
		}

		if !try_solve {
			self.solver_text = self.recreate_text("", Srgba::BLACK);
			return;
		}

		self.solver_text = self.recreate_text("Solving...", Srgba::new_opaque(0, 128, 0));
		let prev_transform = self.solver_text.transformation();
		self.solver_text.set_transformation(Mat4::from_translation(vec3(0.0, -30.0, 0.0)) * prev_transform);

		let cancel_flag = Arc::new(AtomicBool::new(false));
		let cancel_flag_2 = cancel_flag.clone();

		let thread = std::thread::spawn(move || {
			solver.try_solve(6, &Some(cancel_flag_2))
		});

		self.solver_thread = Some(SolverThread {
			thread,
			cancel_flag,
		});
	}

	fn recreate_history(&mut self) {
		let mut text_left = String::new();
		let mut text_right = String::new();

		for (index, item) in self.history.iter().enumerate() {
			let text_ref = if index < self.past_active_history_item { &mut text_left } else { &mut text_right };
			text_ref.push_str(item.to_string_notation(DEFAULT_WIDE_NOTATION_W));
			text_ref.push_str(" ");
		}

		if text_left.as_bytes().len() > 40 {
			self.history_text_left = self.recreate_text(text_left.split_at(text_left.as_bytes().len() - 40).1, Srgba::BLACK);
		} else {
			self.history_text_left = self.recreate_text(&text_left, Srgba::BLACK);
		}

		self.history_text_right = self.recreate_text(&text_right, Srgba::new_opaque(128, 128, 128));

		if !text_left.is_empty() {
			let prev_transform = self.history_text_right.transformation();
			self.history_text_right.set_transformation(Mat4::from_translation(vec3(self.history_text_left.aabb().max().x, 0.0, 0.0)) * prev_transform);
		}
	}
}

pub struct SimulationWindow {
	window: Window,
	simulation: Simulation,
}

impl SimulationWindow {
	pub fn new(window_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let window = Window::new(WindowSettings {
			title: window_name.to_string(),
			..Default::default()
		})?;

		let context = window.gl();

		let camera = Camera::new_perspective(
			window.viewport(),
			vec3(5.0, 5.0, 5.0),
			vec3(0.0, 0.0, 0.0),
			vec3(0.0, 1.0, 0.0),
			degrees(60.0),
			0.01,
			1000.0,
		);

		let camera_text = Camera::new_2d(window.viewport());

		let control = OrbitControl::new(camera.target(), 1.0, 100.0);

		let rubiks = RubiksCube::new(&context);

		let axes = utils::my_axes(&context, 0.05, 3.0);

		let text_generator = TextGenerator::new(include_bytes!("assets/OpenSans-Regular.ttf"), 0, 30.0).unwrap();

		let history_text_left = Gm::new(
			Mesh::new(&context, &text_generator.generate("", TextLayoutOptions::default())),
			ColorMaterial {
				color: Srgba::BLACK,
				..Default::default()
			},
		);
		let history_text_right = Gm::new(
			Mesh::new(&context, &text_generator.generate("", TextLayoutOptions::default())),
			ColorMaterial {
				color: Srgba::BLACK,
				..Default::default()
			},
		);
		let solver_text = Gm::new(
			Mesh::new(&context, &text_generator.generate("", TextLayoutOptions::default())),
			ColorMaterial {
				color: Srgba::BLACK,
				..Default::default()
			},
		);

		Ok(SimulationWindow {
			window,
			simulation: Simulation {
				context,
				camera,
				camera_text,
				control,
				rubiks,
				history: Vec::new(),
				past_active_history_item: 0,
				animations: std::collections::VecDeque::new(),
				pressed_keys: std::collections::HashSet::new(),
				text_generator,
				axes,
				history_text_left,
				history_text_right,
				solver_text,
				solver_thread: None,
				show_axes: false,
			},
		})
	}

	pub fn window_loop(self) {
		let SimulationWindow { window, mut simulation } = self;
		window.render_loop(move |mut frame_input| {
			simulation.render(&mut frame_input)
		});
	}
}
