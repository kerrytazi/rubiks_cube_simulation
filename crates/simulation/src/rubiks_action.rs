#[derive(Clone, Copy, PartialEq)]
pub enum RubiksAction {
	Left{ prime: bool, wide: bool },
	Right{ prime: bool, wide: bool },
	Down{ prime: bool, wide: bool },
	Up{ prime: bool, wide: bool },
	Back{ prime: bool, wide: bool },
	Front{ prime: bool, wide: bool },
	Middle{ prime: bool },
	Equatorial{ prime: bool },
	Standing{ prime: bool },
	RotateCubeX{ prime: bool },
	RotateCubeY{ prime: bool },
	RotateCubeZ{ prime: bool },
}

impl RubiksAction {
	pub fn to_string_notation(&self, wide_w: bool) -> &'static str {
		match *self {
			Self::Left { prime, wide } =>  if wide { if wide_w { if prime { "Lw'"} else { "Lw" } } else { if prime { "l'"} else { "l" } } } else { if prime { "L'"} else { "L" } },
			Self::Right { prime, wide } => if wide { if wide_w { if prime { "Rw'"} else { "Rw" } } else { if prime { "r'"} else { "r" } } } else { if prime { "R'"} else { "R" } },
			Self::Down { prime, wide } =>  if wide { if wide_w { if prime { "Dw'"} else { "Dw" } } else { if prime { "d'"} else { "d" } } } else { if prime { "D'"} else { "D" } },
			Self::Up { prime, wide } =>    if wide { if wide_w { if prime { "Uw'"} else { "Uw" } } else { if prime { "u'"} else { "u" } } } else { if prime { "U'"} else { "U" } },
			Self::Back { prime, wide } =>  if wide { if wide_w { if prime { "Bw'"} else { "Bw" } } else { if prime { "b'"} else { "b" } } } else { if prime { "B'"} else { "B" } },
			Self::Front { prime, wide } => if wide { if wide_w { if prime { "Fw'"} else { "Fw" } } else { if prime { "f'"} else { "f" } } } else { if prime { "F'"} else { "F" } },
			Self::Middle { prime } =>      if prime { "M'"} else { "M" },
			Self::Equatorial { prime } =>  if prime { "E'"} else { "E" },
			Self::Standing { prime } =>    if prime { "S'"} else { "S" },
			Self::RotateCubeX { prime } => if prime { "x'"} else { "x" },
			Self::RotateCubeY { prime } => if prime { "y'"} else { "y" },
			Self::RotateCubeZ { prime } => if prime { "z'"} else { "z" },
		}
	}

	pub fn inverse(&self) -> RubiksAction {
		match *self {
			Self::Left { prime, wide } =>  Self::Left { prime: !prime, wide },
			Self::Right { prime, wide } => Self::Right { prime: !prime, wide },
			Self::Down { prime, wide } =>  Self::Down { prime: !prime, wide },
			Self::Up { prime, wide } =>    Self::Up { prime: !prime, wide },
			Self::Back { prime, wide } =>  Self::Back { prime: !prime, wide },
			Self::Front { prime, wide } => Self::Front { prime: !prime, wide },
			Self::Middle { prime } =>      Self::Middle { prime: !prime },
			Self::Equatorial { prime } =>  Self::Equatorial { prime: !prime },
			Self::Standing { prime } =>    Self::Standing { prime: !prime },
			Self::RotateCubeX { prime } => Self::RotateCubeX { prime: !prime },
			Self::RotateCubeY { prime } => Self::RotateCubeY { prime: !prime },
			Self::RotateCubeZ { prime } => Self::RotateCubeZ { prime: !prime },
		}
	}
}
