use iced::{
    advanced::{image, layout, Widget},
    widget::{
        canvas::{self, Frame, Geometry},
        image::{FilterMethod, Handle},
    },
    Element, Length, Rectangle, Renderer, Size, Theme, Transformation,
};

use std::hash::Hash;

#[derive(Clone)]
pub struct Preview {
    pixels: Vec<u8>,
    width: u32,
    starting_row: u32,
    frame_height: u32,
    frame_width: u32,
    image_handle: Handle,
    scale: f32,
}

impl Default for Preview {
    fn default() -> Self {
        let handle = Handle::from_pixels(0, 0, []);

        Self {
            pixels: Vec::<u8>::new(),
            width: 0,
            frame_height: 0,
            frame_width: 0,
            image_handle: handle,
            starting_row: 0,
            scale: 1.,
        }
    }
}

impl Preview {
    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_frame_height(&mut self, frame_height: u32) {
        self.frame_height = frame_height;
    }

    pub fn frame_height(&self) -> u32 {
        self.frame_height
    }

    pub fn set_frame_width(&mut self, frame_width: u32) {
        self.frame_width = frame_width;
    }

    pub fn frame_width(&self) -> u32 {
        self.frame_width
    }

    pub fn lines(&self) -> u32 {
        (self.pixels.len() / 4)
            .checked_div(self.width as usize)
            .unwrap_or(0) as u32
    }

    pub fn set_starting_line(&mut self, line: u32) {
        self.starting_row = line;
    }

    pub fn starting_line(&self) -> u32 {
        self.starting_row
    }

    pub fn set_pixels(&mut self, pixels: Vec<Pixel>) {
        self.pixels = pixels
            .iter()
            .flat_map(|pixel| [pixel.red, pixel.green, pixel.blue, 0xFF])
            .collect();
    }

    pub fn image_handle(&self) -> Handle {
        self.image_handle.clone()
    }

    pub fn update_image(&mut self) {
        let bytes_needed = self.width as usize * self.frame_height as usize * 4;
        let byte_offset = self.starting_row as usize * self.width as usize * 4;

        let pixel_data_beginnning = match self.pixels.get(byte_offset..) {
            Some(data) => data,
            None => &self.pixels,
        };

        let pixel_data = match pixel_data_beginnning.get(..bytes_needed) {
            Some(data) => data,
            None => pixel_data_beginnning,
        };

        let pixels_got = pixel_data.len() / 4;
        let image_height = self
            .frame_height
            .min((pixels_got as u32).checked_div(self.width).unwrap_or(0));

        let handle = Handle::from_pixels(self.width, image_height, pixel_data.to_owned());

        self.image_handle = handle;
    }

    pub fn clear(&mut self) {
        self.pixels.clear();
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Preview
where
    Renderer: image::Renderer<Handle = iced::advanced::image::Handle>,
    // Handle: Clone + Hash,
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let size = renderer.dimensions(&self.image_handle);
        let size = Size::new(size.width as f32, size.height as f32);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        println!("bounds {bounds:#?}");

        renderer.with_layer(bounds, |renderer| {
            renderer.draw(self.image_handle.clone(), FilterMethod::Nearest, bounds);
        });
    }
}

impl<'a, Message, Theme, Renderer> From<Preview> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + image::Renderer<Handle = iced::advanced::image::Handle>,
    Message: 'a,
    Handle: Clone + Hash + 'a,
{
    fn from(preview: Preview) -> Element<'a, Message, Theme, Renderer> {
        Element::new(preview)
    }
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
