use three_d::Srgba;

use crate::solver::Side;

#[derive(Clone, Copy)]
pub enum CubeColor {
	Orange,
	Red,
	Yellow,
	White,
	Blue,
	Green,
}

impl CubeColor {
	pub fn to_srgba(cc: Option<Self>) -> Srgba {
		match cc {
			None => Srgba::new_opaque(64, 64, 64),
			Some(CubeColor::Orange) => Srgba::new_opaque(255, 165, 0),
			Some(CubeColor::Red) => Srgba::new_opaque(255, 0, 0),
			Some(CubeColor::Yellow) => Srgba::new_opaque(255, 255, 0),
			Some(CubeColor::White) => Srgba::new_opaque(255, 255, 255),
			Some(CubeColor::Blue) => Srgba::new_opaque(0, 0, 255),
			Some(CubeColor::Green) => Srgba::new_opaque(0, 255, 0),
		}
	}
}

impl From<Side> for CubeColor {
	fn from(value: Side) -> Self {
		match value {
			Side::Left => CubeColor::Orange,
			Side::Right => CubeColor::Red,
			Side::Down => CubeColor::Yellow,
			Side::Up => CubeColor::White,
			Side::Back => CubeColor::Blue,
			Side::Front => CubeColor::Green,
		}
	}
}

pub struct CubeColors {
	pub left: Option<CubeColor>,
	pub right: Option<CubeColor>,
	pub down: Option<CubeColor>,
	pub up: Option<CubeColor>,
	pub back: Option<CubeColor>,
	pub front: Option<CubeColor>,
}
