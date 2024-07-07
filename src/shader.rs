use std::sync::Arc;

use glam::Vec2;

use iced::mouse;

use iced::widget::shader::wgpu::util::DeviceExt;
use iced::widget::shader::wgpu::{self};

use iced::widget::shader;
use iced::{Rectangle, Sandbox, Size};

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    viewport_position: Vec2,
    viewport_resolution: Vec2,
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

#[derive(Clone, Debug)]
pub struct DecodingScheme {
    pub red: [Option<u32>; 8],
    pub green: [Option<u32>; 8],
    pub blue: [Option<u32>; 8],
    pub bits_per_pixel: u32,
}

impl Default for DecodingScheme {
    fn default() -> Self {
        Self {
            red: [
                Some(31),
                Some(30),
                Some(29),
                Some(28),
                Some(27),
                Some(26),
                Some(25),
                Some(24),
            ],
            green: [
                Some(23),
                Some(22),
                Some(21),
                Some(20),
                Some(19),
                Some(18),
                Some(17),
                Some(16),
            ],
            blue: [
                Some(15),
                Some(14),
                Some(13),
                Some(12),
                Some(11),
                Some(10),
                Some(9),
                Some(8),
            ],
            bits_per_pixel: 24,
        }
    }
}

struct FragmentShaderPipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    data_buffer: wgpu::Buffer,
}

impl FragmentShaderPipeline {
    fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        // let shader = unsafe {
        //     device.create_shader_module_spirv(
        //         //     wgpu::ShaderModuleDescriptor {
        //         //     label: Some("FragmentShaderPipeline shader"),
        //         //     source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
        //         //         "shader.wgsl"
        //         //     ))),
        //         // }
        //         &include_spirv_raw!("shader.spv"),
        //     )
        // };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("FragmentShaderPipeline shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "shader.wgsl"
            ))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("FragmentShaderPipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shader_quad uniform buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // let pixel_data = vec![0x000000ffu32; 2000 * 2000];
        let mut pixel_data = Vec::<u32>::new();
        for i in 0..=10_000_000 {
            pixel_data.push(i);
        }
        let pixel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(&pixel_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = pipeline.get_bind_group_layout(0);
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shader_quad uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: pixel_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            pipeline,
            uniform_buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
            data_buffer: pixel_buffer,
        }
    }

    fn update_uniforms(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        uniforms: &Uniforms,
        buffer: &[u32],
    ) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
        let pixel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(buffer),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        self.uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shader_quad uniform bind group"),
            layout: &self.uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: pixel_buffer.as_entire_binding(),
                },
            ],
        });
        self.data_buffer = pixel_buffer;
    }

    fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fill color test"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_viewport(
            viewport.x as f32,
            viewport.y as f32,
            viewport.width as f32,
            viewport.height as f32,
            0.0,
            1.0,
        );
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);

        pass.draw(0..3, 0..1);
    }
}

#[derive(Debug)]
pub struct FragmentShaderPrimitive {
    target_width: u32,
    scale: u32,
    buffer: Arc<Vec<u32>>,
    decoding_scheme: DecodingScheme,
    bit_offset: u32,
}

impl FragmentShaderPrimitive {
    fn new(
        target_width: u32,
        scale: u32,
        buffer: Arc<Vec<u32>>,
        bit_offset: u32,
        decoding_scheme: DecodingScheme,
    ) -> Self {
        Self {
            target_width,
            scale,
            buffer,
            bit_offset,
            decoding_scheme,
        }
    }
}

impl shader::Primitive for FragmentShaderPrimitive {
    fn prepare(
        &self,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: Rectangle,
        _target_size: Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        if !storage.has::<FragmentShaderPipeline>() {
            storage.store(FragmentShaderPipeline::new(device, format));
        }

        let pipeline = storage.get_mut::<FragmentShaderPipeline>().unwrap();

        fn d(num: Option<u32>) -> i32 {
            num.and_then(|x| i32::try_from(x).ok()).unwrap_or(-1)
        }

        pipeline.update_uniforms(
            device,
            queue,
            &Uniforms {
                viewport_position: Vec2::new(bounds.x, bounds.y),
                viewport_resolution: Vec2::new(bounds.width, bounds.height),
                scale: self.scale,
                target_width: self.target_width,
                bit_offset: self.bit_offset,
                decoding_red0bit: d(self.decoding_scheme.red[0]),
                decoding_red1bit: d(self.decoding_scheme.red[1]),
                decoding_red2bit: d(self.decoding_scheme.red[2]),
                decoding_red3bit: d(self.decoding_scheme.red[3]),
                decoding_red4bit: d(self.decoding_scheme.red[4]),
                decoding_red5bit: d(self.decoding_scheme.red[5]),
                decoding_red6bit: d(self.decoding_scheme.red[6]),
                decoding_red7bit: d(self.decoding_scheme.red[7]),
                decoding_green0bit: d(self.decoding_scheme.green[0]),
                decoding_green1bit: d(self.decoding_scheme.green[1]),
                decoding_green2bit: d(self.decoding_scheme.green[2]),
                decoding_green3bit: d(self.decoding_scheme.green[3]),
                decoding_green4bit: d(self.decoding_scheme.green[4]),
                decoding_green5bit: d(self.decoding_scheme.green[5]),
                decoding_green6bit: d(self.decoding_scheme.green[6]),
                decoding_green7bit: d(self.decoding_scheme.green[7]),
                decoding_blue0bit: d(self.decoding_scheme.blue[0]),
                decoding_blue1bit: d(self.decoding_scheme.blue[1]),
                decoding_blue2bit: d(self.decoding_scheme.blue[2]),
                decoding_blue3bit: d(self.decoding_scheme.blue[3]),
                decoding_blue4bit: d(self.decoding_scheme.blue[4]),
                decoding_blue5bit: d(self.decoding_scheme.blue[5]),
                decoding_blue6bit: d(self.decoding_scheme.blue[6]),
                decoding_blue7bit: d(self.decoding_scheme.blue[7]),
                decoding_bits_per_pixel: self.decoding_scheme.bits_per_pixel,
            },
            self.buffer.as_slice(),
        );
    }

    fn render(
        &self,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        _target_size: Size<u32>,
        viewport: Rectangle<u32>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let pipeline = storage.get::<FragmentShaderPipeline>().unwrap();
        pipeline.render(target, encoder, viewport);
    }
}

pub struct FragmentShaderProgram {
    target_width: u32,
    scale: u32,
    buffer: Arc<Vec<u32>>,
    bit_offset: u32,
    decoding_scheme: DecodingScheme,
}

impl FragmentShaderProgram {
    pub fn new() -> Self {
        Self {
            target_width: 300,
            scale: 1,
            buffer: Arc::new(vec![0u32; 1]),
            bit_offset: 0,
            decoding_scheme: Default::default(),
        }
    }

    pub fn set_target_width(&mut self, target_width: u32) {
        self.target_width = target_width;
    }

    pub fn target_width(&self) -> u32 {
        self.target_width
    }

    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale;
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }

    pub fn set_buffer(&mut self, mut buffer: Vec<u32>) {
        if buffer.is_empty() {
            buffer.push(0u32);
        }
        self.buffer = Arc::new(buffer);
    }

    pub fn set_bit_offset(&mut self, bit_offset: u32) {
        self.bit_offset = bit_offset;
    }

    pub fn set_decoding_scheme(&mut self, decoding_scheme: DecodingScheme) {
        self.decoding_scheme = decoding_scheme;
    }

    pub fn decoding_scheme(&self) -> &DecodingScheme {
        &self.decoding_scheme
    }
}

impl shader::Program<super::AppMessage> for FragmentShaderProgram {
    type State = ();
    type Primitive = FragmentShaderPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        FragmentShaderPrimitive::new(
            self.target_width,
            self.scale,
            self.buffer.clone(),
            self.bit_offset,
            self.decoding_scheme.clone(),
        )
    }
}
