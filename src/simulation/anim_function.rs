pub enum AnimFunction {
	EaseInOut,
}

impl AnimFunction {
	pub fn calc(&self, t: f64) -> f64 {
		match *self {
			AnimFunction::EaseInOut => {
				if t < 0.5 {
					2.0 * t * t
				} else {
					-1.0 + (4.0 - 2.0 * t) * t
				}
			},
		}
	}
}
