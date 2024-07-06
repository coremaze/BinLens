// struct DecodingScheme {

// }

struct Uniforms {
	viewport_position: vec2f,
	viewport_resolution: vec2f,
	target_width: u32,
	scale: u32,
	bit_offset: u32,
	decoding_red0bit: i32,
	decoding_red1bit: i32,
	decoding_red2bit: i32,
	decoding_red3bit: i32,
	decoding_red4bit: i32,
	decoding_red5bit: i32,
	decoding_red6bit: i32,
	decoding_red7bit: i32,
	decoding_green0bit: i32,
	decoding_green1bit: i32,
	decoding_green2bit: i32,
	decoding_green3bit: i32,
	decoding_green4bit: i32,
	decoding_green5bit: i32,
	decoding_green6bit: i32,
	decoding_green7bit: i32,
	decoding_blue0bit: i32,
	decoding_blue1bit: i32,
	decoding_blue2bit: i32,
	decoding_blue3bit: i32,
	decoding_blue4bit: i32,
	decoding_blue5bit: i32,
	decoding_blue6bit: i32,
	decoding_blue7bit: i32,
	decoding_bits_per_pixel: u32,
}


@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> data: array<u32>;

struct VertexIn {
	@builtin(vertex_index) vertex_index: u32,
}

struct VertexOut {
	@builtin(position) position: vec4f,
}

fn srgbToLinear(srgb: f32) -> f32 {
    if srgb <= 0.04045 {
        return srgb / 12.92;
    } else {
        return pow((srgb + 0.055) / 1.055, 2.4);
    }
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

	let pixel_index = (data_y * u32(uniforms.target_width) + data_x);


	var red: u32 = 0;
	var green: u32 = 0;
	var blue: u32 = 0;

	var bit: u32 = 0;
	var bit_shift: u32 = 0;
	var array_index: u32 = 0;
	
	var color_bit_index: u32 = 0;
	var bit_index: u32 = pixel_index * uniforms.decoding_bits_per_pixel + uniforms.bit_offset;


	// Red bit 7
	if (uniforms.decoding_red7bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red7bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Red bit 6
	if (uniforms.decoding_red6bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red6bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Red bit 5
	if (uniforms.decoding_red5bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red5bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Red bit 4
	if (uniforms.decoding_red4bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red4bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Red bit 3
	if (uniforms.decoding_red3bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red3bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Red bit 2
	if (uniforms.decoding_red2bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red2bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Red bit 1
	if (uniforms.decoding_red1bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red1bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Red bit 0
	if (uniforms.decoding_red0bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_red0bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	red = red << 1;
	red = red | bit;

	// Green bit 7
	if (uniforms.decoding_green7bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green7bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

	// Green bit 6
	if (uniforms.decoding_green6bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green6bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

	// Green bit 5
	if (uniforms.decoding_green5bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green5bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

	// Green bit 4
	if (uniforms.decoding_green4bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green4bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

	// Green bit 3
	if (uniforms.decoding_green3bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green3bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

	// Green bit 2
	if (uniforms.decoding_green2bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green2bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

	// Green bit 1
	if (uniforms.decoding_green1bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green1bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

	// Green bit 0
	if (uniforms.decoding_green0bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_green0bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	green = green << 1;
	green = green | bit;

		// Blue bit 7
	if (uniforms.decoding_blue7bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue7bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	// Blue bit 6
	if (uniforms.decoding_blue6bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue6bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	// Blue bit 5
	if (uniforms.decoding_blue5bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue5bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	// Blue bit 4
	if (uniforms.decoding_blue4bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue4bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	// Blue bit 3
	if (uniforms.decoding_blue3bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue3bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	// Blue bit 2
	if (uniforms.decoding_blue2bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue2bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	// Blue bit 1
	if (uniforms.decoding_blue1bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue1bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	// Blue bit 0
	if (uniforms.decoding_blue0bit >= 0) { // If bit is assigned to something
		color_bit_index = u32(uniforms.decoding_blue0bit);
		array_index = (color_bit_index + bit_index) / 32u;
		bit_shift = 31u - ((color_bit_index + bit_index) % 32u);
		if (array_index < arrayLength(&data)) {
			bit = (data[array_index] >> bit_shift) & 1u;
		}
		else {
			bit = 0u;
		}
	}
	else {
		bit = 0u;
	}
	blue = blue << 1;
	blue = blue | bit;

	let r = f32(red) / 255.0;
	let g = f32(green) / 255.0;
	let b = f32(blue) / 255.0;

	return vec4f(srgbToLinear(r), srgbToLinear(g), srgbToLinear(b), 1.0);
}
