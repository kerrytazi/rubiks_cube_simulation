use std::sync::{Arc, atomic::AtomicBool};

use super::rubiks_action::RubiksAction;

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
	Left,
	Right,
	Down,
	Up,
	Back,
	Front,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Sides {
	pub left: Side,
	pub right: Side,
	pub down: Side,
	pub up: Side,
	pub back: Side,
	pub front: Side,
	_pad1: Side, // Use padding to align struct to 8 bytes for fast comparison
	_pad2: Side,
}

impl Sides {
	pub fn rotated_x(self, cw: bool) -> Self {
		let mut copy = self;
		copy.down = if cw { self.front } else { self.back };
		copy.up = if cw { self.back } else { self.front };
		copy.back = if cw { self.down } else { self.up };
		copy.front = if cw { self.up } else { self.down };
		copy
	}

	pub fn rotated_y(self, cw: bool) -> Self {
		let mut copy = self;
		copy.left = if cw { self.front } else { self.back };
		copy.right = if cw { self.back } else { self.front };
		copy.back = if cw { self.left } else { self.right };
		copy.front = if cw { self.right } else { self.left };
		copy
	}

	pub fn rotated_z(self, cw: bool) -> Self {
		let mut copy = self;
		copy.left = if cw { self.up } else { self.down };
		copy.right = if cw { self.down } else { self.up };
		copy.down = if cw { self.left } else { self.right };
		copy.up = if cw { self.right } else { self.left };
		copy
	}
}

impl Default for Sides {
	fn default() -> Self {
		Sides {
			left: Side::Left,
			right: Side::Right,
			down: Side::Down,
			up: Side::Up,
			back: Side::Back,
			front: Side::Front,
			_pad1: Side::Left,
			_pad2: Side::Left,
		}
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct Solver {
	pub cubes: [[[Sides; 3]; 3]; 3],
}

impl Solver {
	pub fn is_solved(&self) -> bool {
		self.cubes
			.iter()
			.flatten()
			.flatten()
			.all(|c| *c == self.cubes[1][1][1])
	}

	pub fn try_solve(&self, max_depth: usize, cancel_flag_opt: &Option<Arc<AtomicBool>>) -> Option<Vec<RubiksAction>> {
		if max_depth == 0 {
			return None;
		}

		if self.is_solved() {
			return Some(Vec::new());
		}

		for i in 1..=max_depth {
			if let Some(mut result) = self.try_solve_recursive(1, i, cancel_flag_opt) {
				result.reverse();
				return Some(result);
			}
		}

		None
	}

	fn try_solve_recursive(self, cur_depth: usize, max_depth: usize, cancel_flag_opt: &Option<Arc<AtomicBool>>) -> Option<Vec<RubiksAction>> {
		let actions = [
			RubiksAction::Left { prime: false, wide: false },
			RubiksAction::Right { prime: false, wide: false },
			RubiksAction::Down { prime: false, wide: false },
			RubiksAction::Up { prime: false, wide: false },
			RubiksAction::Back { prime: false, wide: false },
			RubiksAction::Front { prime: false, wide: false },
			RubiksAction::Middle { prime: false },
			RubiksAction::Equatorial { prime: false },
			RubiksAction::Standing { prime: false },

			RubiksAction::Left { prime: true, wide: false },
			RubiksAction::Right { prime: true, wide: false },
			RubiksAction::Down { prime: true, wide: false },
			RubiksAction::Up { prime: true, wide: false },
			RubiksAction::Back { prime: true, wide: false },
			RubiksAction::Front { prime: true, wide: false },
			RubiksAction::Middle { prime: true },
			RubiksAction::Equatorial { prime: true },
			RubiksAction::Standing { prime: true },
		];

		for action in actions {
			let mut copy = self.clone();
			copy.apply(action);

			if cur_depth == max_depth {
				if copy.is_solved() {
					let mut result = Vec::with_capacity(max_depth + 1);
					result.push(action);
					return Some(result);
				}
			} else {
				if let Some(cancel_flag) = cancel_flag_opt && cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
					return None;
				}

				if let Some(mut result) = copy.try_solve_recursive(cur_depth + 1, max_depth, cancel_flag_opt) {
					result.push(action);
					return Some(result);
				}
			}
		}

		None
	}

	fn rotate_x(&mut self, i: usize, cw: bool) {
		for y in 0..3 {
			for z in 0..3 {
				self.cubes[i][y][z] = self.cubes[i][y][z].rotated_x(cw);
			}
		}

		for (f, t) in Self::rotations(cw) {
			self.swap((i, f.0, f.1), (i, t.0, t.1));
		}
	}

	fn rotate_y(&mut self, i: usize, cw: bool) {
		for x in 0..3 {
			for z in 0..3 {
				self.cubes[x][i][z] = self.cubes[x][i][z].rotated_y(cw);
			}
		}

		for (f, t) in Self::rotations(!cw) {
			self.swap((f.0, i, f.1), (t.0, i, t.1));
		}
	}

	fn rotate_z(&mut self, i: usize, cw: bool) {
		for x in 0..3 {
			for y in 0..3 {
				self.cubes[x][y][i] = self.cubes[x][y][i].rotated_z(cw);
			}
		}

		for (f, t) in Self::rotations(cw) {
			self.swap((f.0, f.1, i), (t.0, t.1, i));
		}
	}

	fn apply(&mut self, action: RubiksAction) {
		match action {
			RubiksAction::Left { prime, wide } =>  { self.rotate_x(0,  prime); if wide { self.rotate_x(1,  prime); } },
			RubiksAction::Right { prime, wide } => { self.rotate_x(2, !prime); if wide { self.rotate_x(1, !prime); } },
			RubiksAction::Down { prime, wide } =>  { self.rotate_y(0,  prime); if wide { self.rotate_y(1,  prime); } },
			RubiksAction::Up { prime, wide } =>    { self.rotate_y(2, !prime); if wide { self.rotate_y(1, !prime); } },
			RubiksAction::Back { prime, wide } =>  { self.rotate_z(0,  prime); if wide { self.rotate_z(1,  prime); } },
			RubiksAction::Front { prime, wide } => { self.rotate_z(2, !prime); if wide { self.rotate_z(1, !prime); } },
			RubiksAction::Middle { prime } =>      { self.rotate_x(1,  prime); },
			RubiksAction::Equatorial { prime } =>  { self.rotate_y(1,  prime); },
			RubiksAction::Standing { prime } =>    { self.rotate_z(1,  prime); },
			RubiksAction::RotateCubeX { prime } => { self.rotate_x(0,  prime); self.rotate_x(1,  prime); self.rotate_x(2,  prime); },
			RubiksAction::RotateCubeY { prime } => { self.rotate_y(0,  prime); self.rotate_y(1,  prime); self.rotate_y(2,  prime); },
			RubiksAction::RotateCubeZ { prime } => { self.rotate_z(0, !prime); self.rotate_z(1, !prime); self.rotate_z(2, !prime); },
		}
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
