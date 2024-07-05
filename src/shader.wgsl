struct Uniforms {
	viewport_position: vec2f,
	viewport_resolution: vec2f,
	target_width: u32,
	scale: u32,
}


@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> data: array<u32>;

struct VertexIn {
	@builtin(vertex_index) vertex_index: u32,
}

struct VertexOut {
	@builtin(position) position: vec4f,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
	let uv = vec2f(vec2u((in.vertex_index << 1) & 2, in.vertex_index & 2));
	let position = vec4f(uv * 2. - 1., 0., 1.);
	return VertexOut(position);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
	let real_pos_x = in.position.x - uniforms.viewport_position.x;
	let real_pos_y = in.position.y - uniforms.viewport_position.y;

	if (real_pos_x < 0.0 || real_pos_y < 0.0) {
		return vec4f(0.0, 0.0, 0.0, 1.0);
	}

	let data_x = u32(real_pos_x) / uniforms.scale;
	let data_y = u32(real_pos_y) / uniforms.scale;

	if (data_x > uniforms.target_width) {
		return vec4f(0.0, 0.0, 0.0, 1.0);
	} 

	// if (real_pos_x < 20.0) {
	// 	return vec4f(1.0, 0.0, 0.0, 1.0);
	// }

	// if (real_pos_y < 20.0) {
	// 	return vec4f(0.0, 0.0, 1.0, 1.0);
	// }

	let i: u32 = data_y * u32(uniforms.target_width) + data_x;
	if (i < arrayLength(&data)) {
		let pixel = data[i];
		let red = pixel & 0x000000FF;
		let green = (pixel & 0x0000FF00) >> 8;
		let blue = (pixel & 0x00FF0000) >> 16;

		let r = f32(red) / 255.0;
		let g = f32(green) / 255.0;
		let b = f32(blue) / 255.0;

		
		return vec4f(r, g, b, 1.0);
	}
	else {
		return vec4f(0.0, 0.0, 0.0, 1.0);
	}
}
