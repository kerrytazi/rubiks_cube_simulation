use three_d::*;

pub fn my_axes(context: &Context, radius: f32, length: f32) -> Gm<InstancedMesh, ColorMaterial> {
	let mut cpu_mesh = CpuMesh::arrow(0.9, 0.6, 16);

	cpu_mesh
		.transform(Mat4::from_nonuniform_scale(length, radius, radius))
		.unwrap();

	let model = Gm::new(
		InstancedMesh::new(
			context,
			&Instances {
				transformations: vec![
					Mat4::identity(),
					Mat4::from_angle_z(degrees(90.0)),
					Mat4::from_angle_y(degrees(-90.0)),
				],
				texture_transformations: None,
				colors: Some(vec![
					Srgba::new_opaque(128, 0, 0),
					Srgba::new_opaque(0, 128, 0),
					Srgba::new_opaque(0, 0, 128),
				]),
			},
			&cpu_mesh,
		),
		ColorMaterial::default(),
	);

	model
}
