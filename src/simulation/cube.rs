use super::{cube_colors::{CubeColor, CubeColors}, keyframe::{KeyFrame, KeyFrameResult}, solver::Sides};

use three_d::*;

enum CubeAnimationType {
	RotateX,
	RotateY,
	RotateZ,
}

struct CubeAnimation {
	start_mat: Mat4,
	keyframe: KeyFrame,
	anim_type: CubeAnimationType,
}

impl CubeAnimation {
	fn new(cube: &Cube, start_time: f64, duration: f64, angle: Rad<f32>, anim_type: CubeAnimationType) -> Self {
		CubeAnimation {
			start_mat: cube.gm.transformation(),
			keyframe: KeyFrame::new_ease_in_out(start_time, duration, angle.0 as f64),
			anim_type: anim_type,
		}
	}
}

type CubeGm = Gm<Mesh, ColorMaterial>;

pub struct Cube {
	gm: CubeGm,
	animation: Option<CubeAnimation>,
	sides: Sides,
}

impl Cube {
	pub fn new(context: &Context, center: Vec3, cube_colors: CubeColors) -> Self {
		let mesh = Self::create_mesh(center, cube_colors);

		let material = {
			let mut material = ColorMaterial::new(
				&context,
				&CpuMaterial::default(),
			);
			material.render_states.cull = Cull::Back;
			material
		};

		Cube {
			gm: CubeGm::new(Mesh::new(&context, &mesh), material),
			animation: None,
			sides: Sides::default(),
		}
	}

	fn add_plane(
		indices: &mut Vec<u16>,
		positions: &mut Vec<Vec3>,
		colors: &mut Vec<Srgba>,
		center: Vec3,
		rotation: Quat,
		color: Srgba
	) {
		if positions.len() > u16::max_value() as usize {
			panic!("add_plane: positions.len() > u16::max_value()");
		}

		let idx_offset = positions.len() as u16;

		indices.extend_from_slice(&[
			idx_offset + 0,
			idx_offset + 1,
			idx_offset + 2,
			idx_offset + 2,
			idx_offset + 3,
			idx_offset + 0,
		]);

		let halfsize = 0.5;

		positions.extend_from_slice(&[
			center + rotation * Vec3::new(-halfsize, -halfsize, 0.0),
			center + rotation * Vec3::new( halfsize, -halfsize, 0.0),
			center + rotation * Vec3::new( halfsize,  halfsize, 0.0),
			center + rotation * Vec3::new(-halfsize,  halfsize, 0.0),
		]);

		colors.extend_from_slice(&[
			color,
			color,
			color,
			color,
		]);
	}

	fn create_mesh(center: Vec3, cube_colors: CubeColors) -> CpuMesh {
		let mut indices = Vec::with_capacity(6 * 6);
		let mut positions  = Vec::with_capacity(4 * 6);
		let mut colors  = Vec::with_capacity(4 * 6);

		Self::add_plane(&mut indices, &mut positions, &mut colors, center + vec3(-0.5,  0.0,  0.0), Quat::from_angle_y(degrees(-90.0)), CubeColor::to_srgba(cube_colors.left));
		Self::add_plane(&mut indices, &mut positions, &mut colors, center + vec3( 0.5,  0.0,  0.0), Quat::from_angle_y(degrees( 90.0)), CubeColor::to_srgba(cube_colors.right));
		Self::add_plane(&mut indices, &mut positions, &mut colors, center + vec3( 0.0, -0.5,  0.0), Quat::from_angle_x(degrees( 90.0)), CubeColor::to_srgba(cube_colors.down));
		Self::add_plane(&mut indices, &mut positions, &mut colors, center + vec3( 0.0,  0.5,  0.0), Quat::from_angle_x(degrees(-90.0)), CubeColor::to_srgba(cube_colors.up));
		Self::add_plane(&mut indices, &mut positions, &mut colors, center + vec3( 0.0,  0.0, -0.5), Quat::from_angle_y(degrees(180.0)), CubeColor::to_srgba(cube_colors.back));
		Self::add_plane(&mut indices, &mut positions, &mut colors, center + vec3( 0.0,  0.0,  0.5), Quat::zero(),                       CubeColor::to_srgba(cube_colors.front));

		CpuMesh {
			indices: Indices::U16(indices),
			positions: Positions::F32(positions),
			colors: Some(colors),
			..Default::default()
		}
	}

	pub fn rotate_x(&mut self, cw: bool, start_time: f64, duration: f64) {
		let angle: Rad<f32> = degrees(if cw { -90.0 } else { 90.0 }).into();

		if duration > 0.0 {
			self.animation = Some(CubeAnimation::new(self, start_time, duration, angle, CubeAnimationType::RotateX));
		} else {
			let prev = self.gm.transformation();
			self.gm.set_transformation(Mat4::from_angle_x(angle) * prev);
		}

		self.sides = self.sides.rotated_x(cw);
	}

	pub fn rotate_y(&mut self, cw: bool, start_time: f64, duration: f64) {
		let angle: Rad<f32> = degrees(if cw { -90.0 } else { 90.0 }).into();

		if duration > 0.0 {
			self.animation = Some(CubeAnimation::new(self, start_time, duration, angle, CubeAnimationType::RotateY));
		} else {
			let prev = self.gm.transformation();
			self.gm.set_transformation(Mat4::from_angle_y(angle) * prev);
		}

		self.sides = self.sides.rotated_y(cw);
	}

	pub fn rotate_z(&mut self, cw: bool, start_time: f64, duration: f64) {
		let angle: Rad<f32> = degrees(if cw { -90.0 } else { 90.0 }).into();

		if duration > 0.0 {
			self.animation = Some(CubeAnimation::new(self, start_time, duration, angle, CubeAnimationType::RotateZ));
		} else {
			let prev = self.gm.transformation();
			self.gm.set_transformation(Mat4::from_angle_z(angle) * prev);
		}

		self.sides = self.sides.rotated_z(cw);
	}

	pub fn update_animation(&mut self, current_time: f64) -> bool {
		let animation = if let Some(animation) = &self.animation {
			animation
		} else {
			return false;
		};

		let KeyFrameResult { value, ended } = animation.keyframe.calc(current_time);

		let rotation_mat = match animation.anim_type {
			CubeAnimationType::RotateX => { Mat4::from_angle_x(radians(value as f32)) },
			CubeAnimationType::RotateY => { Mat4::from_angle_y(radians(value as f32)) },
			CubeAnimationType::RotateZ => { Mat4::from_angle_z(radians(value as f32)) },
		};

		self.gm.set_transformation(rotation_mat * animation.start_mat);

		if ended {
			self.animation = None;
		}

		ended
	}

	pub fn get_sides(&self) -> Sides {
		self.sides
	}
}

impl<'a> IntoIterator for &'a Cube {
	type Item = <&'a CubeGm as IntoIterator>::Item;
	type IntoIter = <&'a CubeGm as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.gm.into_iter()
	}
}
