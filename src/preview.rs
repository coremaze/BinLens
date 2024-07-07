use iced::{
    advanced::{image, layout, Widget},
    widget::{
        canvas::{self, Frame, Geometry},
        image::{FilterMethod, Handle},
    },
    Element, Length, Rectangle, Renderer, Size, Theme, Transformation,
};

use crate::shader::DecodingScheme;

use super::shader::FragmentShaderProgram;
use std::{hash::Hash, sync::Arc};

// #[derive(Clone)]
pub struct Preview {
    start_bit: u64,
    frame_height: u32,
    frame_width: u32,
    file_data: Arc<Vec<u8>>,
    pub program: FragmentShaderProgram,
}

impl Default for Preview {
    fn default() -> Self {
        Self {
            frame_height: 0,
            frame_width: 0,
            start_bit: 0,
            program: FragmentShaderProgram::new(),
            file_data: Arc::new(Vec::<u8>::new()),
        }
    }
}

impl Preview {
    pub fn set_start_bit(&mut self, offset: u64) {
        self.start_bit = offset;
        self.update_program_buffer();
    }

    pub fn start_bit(&self) -> u64 {
        self.start_bit
    }

    pub fn file_data(&self) -> &[u8] {
        &self.file_data
    }

    pub fn bits_per_line(&self) -> u64 {
        u64::from(self.target_width()) * u64::from(self.decoding_scheme().bits_per_pixel)
    }

    pub fn scale(&self) -> u32 {
        self.program.scale()
    }

    pub fn set_scale(&mut self, scale: u32) {
        self.program.set_scale(scale)
    }

    pub fn set_target_width(&mut self, width: u32) {
        self.program.set_target_width(width)
    }

    pub fn target_width(&self) -> u32 {
        self.program.target_width()
    }

    pub fn set_frame_height(&mut self, frame_height: u32) {
        self.frame_height = frame_height;
    }

    pub fn set_frame_width(&mut self, frame_width: u32) {
        self.frame_width = frame_width;
    }

    pub fn lines(&self) -> u32 {
        let bits = self.file_data.len() * 8;
        let lines = bits.checked_div(self.bits_per_line() as usize).unwrap_or(0);
        lines as u32
    }

    pub fn set_file_data(&mut self, data: Arc<Vec<u8>>) {
        self.file_data = data;
        self.update_program_buffer();
    }

    pub fn clear(&mut self) {
        self.set_file_data(Arc::new(vec![]));
    }

    pub fn set_decoding_scheme(&mut self, decoding_scheme: &DecodingScheme) {
        self.program.set_decoding_scheme(decoding_scheme.clone());
    }

    pub fn decoding_scheme(&self) -> &DecodingScheme {
        self.program.decoding_scheme()
    }

    fn update_program_buffer(&mut self) {
        let bits_per_pixel = self.decoding_scheme().bits_per_pixel;

        let start_byte = self.start_bit / 8;
        let bit_offset = (self.start_bit % 8) as u32;

        let start = start_byte as usize;
        let max_size = (((self.frame_height * self.frame_width * bits_per_pixel) + 1) / 8) as usize;

        let buf_beginning = self.file_data.get(start..).unwrap_or_default();
        let buf_limited = buf_beginning.get(..max_size).unwrap_or(buf_beginning);

        let program_buffer = buf_limited
            .chunks(4)
            .map(|bytes| {
                let a = bytes.get(0).unwrap_or(&0);
                let b = bytes.get(1).unwrap_or(&0);
                let c = bytes.get(2).unwrap_or(&0);
                let d = bytes.get(3).unwrap_or(&0);

                (u32::from(*a) << 24) | (u32::from(*b) << 16) | (u32::from(*c) << 8) | u32::from(*d)
            })
            .collect::<Vec<u32>>();

        self.program.set_bit_offset(bit_offset);
        self.program.set_buffer(program_buffer);
    }
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
