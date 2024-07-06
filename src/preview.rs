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
    starting_row: u32,
    frame_height: u32,
    frame_width: u32,
    pub program: FragmentShaderProgram,
}

impl Default for Preview {
    fn default() -> Self {
        Self {
            frame_height: 0,
            frame_width: 0,
            starting_row: 0,
            byte_offset: 0,
            program: FragmentShaderProgram::new(),
        }
    }
}

impl Preview {
    pub fn set_byte_offset(&mut self, offset: u32) {
        self.byte_offset = offset;
        self.program.set_bit_offset(self.byte_offset * 8);
    }

    pub fn byte_offset(&self) -> u32 {
        self.byte_offset
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
        self.starting_row = line;
    }

    pub fn starting_line(&self) -> u32 {
        self.starting_row
    }

    pub fn set_file_data(&mut self, data: &[u8]) {
        let buffer = data
            // .get(self.byte_offset as usize..)
            // .unwrap_or_default()
            .chunks(4)
            .map(|bytes| {
                let a = bytes.get(0).unwrap_or(&0);
                let b = bytes.get(1).unwrap_or(&0);
                let c = bytes.get(2).unwrap_or(&0);
                let d = bytes.get(3).unwrap_or(&0);

                (u32::from(*a) << 24) | (u32::from(*b) << 16) | (u32::from(*c) << 8) | u32::from(*d)
            })
            .collect::<Vec<u32>>();

        self.program.set_buffer(buffer);
    }

    pub fn clear(&mut self) {
        self.program.set_buffer(vec![0u32; 1]);
    }

    pub fn set_decoding_scheme(&mut self, decoding_scheme: &DecodingScheme) {
        self.program.set_decoding_scheme(decoding_scheme.clone());
    }
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
