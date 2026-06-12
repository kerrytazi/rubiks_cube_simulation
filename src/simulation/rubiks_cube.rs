use three_d::*;
use super::cube::Cube;
use super::cube_colors::{CubeColor, CubeColors};
use super::rubiks_action::RubiksAction;
use super::solver::Solver;

pub struct RubiksCube {
	cubes: [[[Box<Cube>; 3]; 3]; 3],
}

impl RubiksCube {
	pub fn new(context: &Context) -> Self {
		let cubes = std::array::from_fn(|x| {
			std::array::from_fn(|y| {
				std::array::from_fn(|z| {
					let center = vec3(x as f32 - 1.0, y as f32 - 1.0, z as f32 - 1.0) * 1.02;

					Box::new(Cube::new(&context, center, CubeColors {
						left: if x == 0 { Some(CubeColor::Orange) } else { None },
						right: if x == 2 { Some(CubeColor::Red) } else { None },
						down: if y == 0 { Some(CubeColor::Yellow) } else { None },
						up: if y == 2 { Some(CubeColor::White) } else { None },
						back: if z == 0 { Some(CubeColor::Blue) } else { None },
						front: if z == 2 { Some(CubeColor::Green) } else { None },
					}))
				})
			})
		});

		RubiksCube { cubes }
	}

	pub fn rotate_x(&mut self, i: usize, cw: bool, start_time: f64, duration: f64) {
		for y in 0..3 {
			for z in 0..3 {
				self.cubes[i][y][z].rotate_x(cw, start_time, duration);
			}
		}

		for (f, t) in Self::rotations(cw) {
			self.swap((i, f.0, f.1), (i, t.0, t.1));
		}
	}

	pub fn rotate_y(&mut self, i: usize, cw: bool, start_time: f64, duration: f64) {
		for x in 0..3 {
			for z in 0..3 {
				self.cubes[x][i][z].rotate_y(cw, start_time, duration);
			}
		}

		for (f, t) in Self::rotations(!cw) {
			self.swap((f.0, i, f.1), (t.0, i, t.1));
		}
	}

	pub fn rotate_z(&mut self, i: usize, cw: bool, start_time: f64, duration: f64) {
		for x in 0..3 {
			for y in 0..3 {
				self.cubes[x][y][i].rotate_z(cw, start_time, duration);
			}
		}

		for (f, t) in Self::rotations(cw) {
			self.swap((f.0, f.1, i), (t.0, t.1, i));
		}
	}

	pub fn apply(&mut self, action: RubiksAction, start_time: f64, duration: f64) {
		match action {
			RubiksAction::Left { prime, wide } =>  { self.rotate_x(0,  prime, start_time, duration); if wide { self.rotate_x(1,  prime, start_time, duration); } },
			RubiksAction::Right { prime, wide } => { self.rotate_x(2, !prime, start_time, duration); if wide { self.rotate_x(1, !prime, start_time, duration); } },
			RubiksAction::Down { prime, wide } =>  { self.rotate_y(0,  prime, start_time, duration); if wide { self.rotate_y(1,  prime, start_time, duration); } },
			RubiksAction::Up { prime, wide } =>    { self.rotate_y(2, !prime, start_time, duration); if wide { self.rotate_y(1, !prime, start_time, duration); } },
			RubiksAction::Back { prime, wide } =>  { self.rotate_z(0,  prime, start_time, duration); if wide { self.rotate_z(1,  prime, start_time, duration); } },
			RubiksAction::Front { prime, wide } => { self.rotate_z(2, !prime, start_time, duration); if wide { self.rotate_z(1, !prime, start_time, duration); } },
			RubiksAction::Middle { prime } =>      { self.rotate_x(1,  prime, start_time, duration); },
			RubiksAction::Equatorial { prime } =>  { self.rotate_y(1,  prime, start_time, duration); },
			RubiksAction::Standing { prime } =>    { self.rotate_z(1,  prime, start_time, duration); },
			RubiksAction::RotateCubeX { prime } => { self.rotate_x(0,  prime, start_time, duration); self.rotate_x(1,  prime, start_time, duration); self.rotate_x(2,  prime, start_time, duration); },
			RubiksAction::RotateCubeY { prime } => { self.rotate_y(0,  prime, start_time, duration); self.rotate_y(1,  prime, start_time, duration); self.rotate_y(2,  prime, start_time, duration); },
			RubiksAction::RotateCubeZ { prime } => { self.rotate_z(0, !prime, start_time, duration); self.rotate_z(1, !prime, start_time, duration); self.rotate_z(2, !prime, start_time, duration); },
		}
	}

	pub fn update_animations(&mut self, current_time: f64) -> bool {
		let mut ended = false;

		for x in 0..3 {
			for y in 0..3 {
				for z in 0..3 {
					ended = self.cubes[x][y][z].update_animation(current_time) || ended;
				}
			}
		}

		ended
	}

	pub fn to_solver(&self) -> Solver {
		let cubes = std::array::from_fn(|x| {
			std::array::from_fn(|y| {
				std::array::from_fn(|z| {
					self.cubes[x][y][z].get_sides()
				})
			})
		});

		Solver { cubes }
	}

	fn swap(&mut self, index1: (usize, usize, usize), index2: (usize, usize, usize)) {
		if index1 == index2 {
			panic!("Can't swap with itself");
		}

		unsafe {
			let ptr1 = &mut self.cubes[index1.0][index1.1][index1.2] as *mut _;
			let ptr2 = &mut self.cubes[index2.0][index2.1][index2.2] as *mut _;
			std::mem::swap(&mut *ptr1, &mut *ptr2);
		}
	}

	const fn rotations(cw: bool) -> [((usize, usize), (usize, usize)); 6] {
		let mut rotations: [((usize, usize), (usize, usize)); 6] = [
			((0, 2), (0, 0)),
			((0, 0), (2, 0)),
			((2, 0), (2, 2)),
			((0, 1), (1, 0)),
			((1, 0), (2, 1)),
			((2, 1), (1, 2)),
		];

		if !cw {
			rotations.reverse();
		}

		rotations
	}
}

// God forbid allocations
impl<'a> IntoIterator for &'a RubiksCube {
	type Item = <&'a Cube as IntoIterator>::Item;
	type IntoIter = std::iter::FlatMap<
		std::iter::Flatten<
			std::iter::Flatten<
				std::slice::Iter<'a, [[Box<Cube>; 3]; 3]>
			>
		>,
		<&'a Cube as IntoIterator>::IntoIter,
		for<'c> fn(&'c Box<Cube>) -> <&'c Cube as IntoIterator>::IntoIter
	>;

	fn into_iter(self) -> Self::IntoIter {
		self.cubes
			.iter()
			.flatten()
			.flatten()
			.flat_map(|c| c.into_iter())
	}
}
