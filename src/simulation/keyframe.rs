use super::anim_function::AnimFunction;

pub struct KeyFrame {
	start_time: f64,
	finish_time: f64,
	start_value: f64,
	finish_value: f64,
	anim: AnimFunction,
}

pub struct KeyFrameResult {
	pub value: f64,
	pub ended: bool,
}

impl KeyFrame {
	pub fn new_ease_in_out(start_time: f64, duration: f64, finish_value: f64) -> Self {
		KeyFrame {
			start_time,
			finish_time: start_time + duration,
			start_value: 0.0,
			finish_value,
			anim: AnimFunction::EaseInOut,
		}
	}

	pub fn calc(&self, current_time: f64) -> KeyFrameResult {
		if current_time < self.finish_time {
			let normalized_time = (current_time - self.start_time) / (self.finish_time - self.start_time);
			let normalized_value = self.anim.calc(normalized_time);

			KeyFrameResult {
				value: self.start_value + (self.finish_value - self.start_value) * normalized_value,
				ended: false,
			}
		} else {
			self.calc_end()
		}
	}

	pub fn calc_end(&self) -> KeyFrameResult {
		KeyFrameResult {
			value: self.finish_value,
			ended: true,
		}
	}
}
