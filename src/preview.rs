use iced::{
    advanced::{image, layout, Widget},
    widget::{
        canvas::{self, Frame, Geometry},
        image::{FilterMethod, Handle},
    },
    Element, Length, Rectangle, Renderer, Size, Theme, Transformation,
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::shader::DecodingScheme;

use super::shader::FragmentShaderProgram;
use std::{hash::Hash, sync::Arc};

// #[derive(Clone)]
pub struct Preview {
    byte_offset: u32,
    scroll_row: u32,
    scroll_bit_offset: u32,
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
            scroll_row: 0,
            scroll_bit_offset: 0,
            byte_offset: 0,
            program: FragmentShaderProgram::new(),
            file_data: Arc::new(Vec::<u8>::new()),
        }
    }
}

impl Preview {
    pub fn set_byte_offset(&mut self, offset: u32) {
        self.byte_offset = offset;
        self.program
            .set_bit_offset(self.byte_offset * 8 + self.scroll_bit_offset);
    }

    pub fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    fn set_scroll_bit_offset(&mut self, offset: u32) {
        self.scroll_bit_offset = offset;
        self.set_byte_offset(self.byte_offset());
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

    pub fn set_starting_line(&mut self, line: u32) {
        self.scroll_row = line;
        self.update_program_buffer();
    }

    pub fn starting_line(&self) -> u32 {
        self.scroll_row
    }

    pub fn lines(&self) -> u32 {
        let bits = self.file_data.len() * 8;
        let bits_per_line = self.target_width() * self.decoding_scheme().bits_per_pixel;
        let lines = bits.checked_div(bits_per_line as usize).unwrap_or(0);
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
        let pixels_per_row = self.target_width();
        let row_offset = self.starting_line();

        let start_bit = bits_per_pixel * pixels_per_row * row_offset;
        let start_byte = start_bit / 8;
        let scroll_bit_offset = start_bit % 8;

        let start = start_byte as usize;
        let max_size = (((self.frame_height * self.frame_width * bits_per_pixel) + 1) / 8) as usize;

        let buf_beginning = self.file_data.get(start..).unwrap_or(&self.file_data);
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

        self.set_scroll_bit_offset(scroll_bit_offset);
        self.program.set_buffer(program_buffer);
    }
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
