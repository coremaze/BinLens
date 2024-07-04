use std::{fmt::Display, fs, path::PathBuf};

use iced::{
    advanced::{graphics::core::event, mouse, Widget},
    widget::{responsive, scrollable::Direction, Scrollable},
    window, Application, Event, Subscription,
};
mod preview;
use preview::{Pixel, Preview};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

struct FileInfo {
    data: Vec<u8>,
    path: PathBuf,
}
struct ImageViewApp {
    pixel_mode: PixelMode,
    file: Option<FileInfo>,
    preview: Preview,
}
#[derive(Debug, Clone)]
enum AppMessage {
    PixelModeSelected(PixelMode),
    ImageWidthSelected(u32),
    OpenFileDialog,
    ImageScroll(u32),
    ImageScale(u32),
    ByteOffset(u32),
    WindowResize { width: u32, height: u32 },
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PixelMode {
    RGB,
    BGR,
    BPP8,
}
impl PixelMode {
    pub const ALL: &'static [Self] = &[Self::RGB, Self::BGR, Self::BPP8];
}
impl Display for PixelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PixelMode::RGB => "rgb",
            PixelMode::BGR => "bgr",
            PixelMode::BPP8 => "8bpp",
        })
    }
}

impl ImageViewApp {
    pub fn update_pixel_decoding(&mut self) {
        match &self.file {
            Some(file) => {
                let pixels = match self.pixel_mode {
                    PixelMode::RGB => file
                        .data
                        .chunks_exact(3)
                        .map(|data| Pixel {
                            red: data[0],
                            green: data[1],
                            blue: data[2],
                        })
                        .collect::<Vec<Pixel>>(),
                    PixelMode::BGR => file
                        .data
                        .chunks_exact(3)
                        .map(|data| Pixel {
                            blue: data[0],
                            green: data[1],
                            red: data[2],
                        })
                        .collect::<Vec<Pixel>>(),
                    PixelMode::BPP8 => file
                        .data
                        .iter()
                        .map(|&data| Pixel {
                            red: data,
                            green: data,
                            blue: data,
                        })
                        .collect::<Vec<Pixel>>(),
                };

                self.preview.set_pixels(pixels);
            }
            None => {
                self.preview.clear();
            }
        }
    }
}

impl iced::Application for ImageViewApp {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                pixel_mode: PixelMode::RGB,
                file: None,
                preview: Preview::default(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        match &self.file {
            Some(file) => format!("BinLens - {}", file.path.to_string_lossy()),
            None => "BinLens".to_owned(),
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        // dbg!("Got message {:?}", &message);

        match message {
            AppMessage::PixelModeSelected(pixel_mode) => {
                self.pixel_mode = pixel_mode;
                self.update_pixel_decoding();
                self.preview.update_image();
            }
            AppMessage::ImageWidthSelected(image_width) => {
                self.preview.set_width(image_width);
                self.preview.update_image();
            }
            AppMessage::OpenFileDialog => {
                let file_info = get_file();
                if file_info.is_some() {
                    self.file = file_info;
                }
                self.update_pixel_decoding();
                self.preview.update_image();
            }
            AppMessage::ImageScroll(scroll) => {
                println!("scroll {scroll}");
                if let Some(file) = &self.file {
                    self.preview.set_starting_line(scroll);
                    self.preview.update_image();
                }
            }
            AppMessage::WindowResize { width, height } => {
                self.preview.set_frame_height(height);
                self.preview.set_frame_width(width);
                self.preview.update_image();
            }
            AppMessage::ImageScale(scale) => {
                self.preview.set_scale(scale);
                self.preview.update_image();
            }
            AppMessage::ByteOffset(offset) => {
                self.preview.set_byte_offset(offset);
                self.preview.update_image();
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        use iced::widget::{
            button, canvas, column, container, horizontal_rule, image::Handle, pick_list, row,
            scrollable, slider, text, vertical_rule,
        };
        use iced::Length;

        let preview = preview(self);
        let controls = controls(self);
        row![preview, vertical_rule(2), controls].into()
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        iced::event::listen_with(|event, _| match event {
            Event::Window(_, window_event) => {
                let new_size: Option<(u32, u32)> = match window_event {
                    window::Event::Opened { position: _, size } => {
                        Some((size.width as u32, size.height as u32))
                    }
                    window::Event::Resized { width, height } => Some((width, height)),
                    _ => None,
                };

                new_size.map(|(width, height)| AppMessage::WindowResize { width, height })
            }
            _ => None,
        })
    }
}

fn get_file() -> Option<FileInfo> {
    let Some(path) = rfd::FileDialog::new().pick_file() else {
        return None;
    };

    let Ok(data) = fs::read(&path) else {
        return None;
    };

    return Some(FileInfo { data, path });
}

fn preview(app: &ImageViewApp) -> iced::Element<AppMessage> {
    use iced::ContentFit;
    use iced::Length;
    use iced::Padding;

    use iced::widget::{
        button, canvas, column, container, horizontal_rule, image::Handle, pick_list, row,
        scrollable, text, vertical_rule, vertical_slider, Canvas,
    };

    let scrollbar = vertical_slider(
        0u32..=app.preview.lines(),
        app.preview.starting_line(),
        AppMessage::ImageScroll,
    );

    let dir = scrollable::Direction::Both {
        vertical: scrollable::Properties::new()
            .margin(0)
            .scroller_width(0)
            .width(0),
        horizontal: scrollable::Properties::new()
            .margin(0)
            .scroller_width(0)
            .width(0),
    };

    // canvas()

    row!(
        // scrollable(container(iced::widget::Image::new(
        //     app.preview.image_handle()
        // )))
        container(row!(app.preview.clone()))
            // .direction(dir)
            .width(Length::Fill)
            .height(Length::Fill),
        container(scrollbar).padding(Padding {
            top: 0.,
            right: 0.,
            bottom: 0.,
            left: 10.,
        })
    )
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn controls(app: &ImageViewApp) -> iced::Element<AppMessage> {
    use iced::{border::Radius, Border, Color, Length};

    use iced::widget::{
        button, canvas, column, container, horizontal_rule, image::Handle, pick_list, row,
        scrollable, scrollable::Scrollbar, scrollable::Scroller, slider, text, vertical_rule,
    };

    let controls = container(
        column!(
            open_button(app),
            pick_list(
                PixelMode::ALL,
                Some(&app.pixel_mode),
                AppMessage::PixelModeSelected
            )
            .width(Length::Fill),
            horizontal_rule(1),
            column!(
                text(format!("Image width: {}", app.preview.width())),
                slider(1..=256, app.preview.width(), AppMessage::ImageWidthSelected)
            ),
            column!(
                text(format!("Scale: {}x", app.preview.scale())),
                slider(1..=10, app.preview.scale(), AppMessage::ImageScale)
            ),
            column!(
                text(format!("Byte offset: {}", app.preview.byte_offset())),
                slider(
                    0..=app.preview.width(),
                    app.preview.byte_offset(),
                    AppMessage::ByteOffset
                )
            ),
            horizontal_rule(1),
            text("Controls"),
            text("Controls!"),
            text("Controls!!"),
        )
        .width(200)
        .height(Length::Fill)
        .padding(10),
    );
    controls.into()
}

fn open_button(app: &ImageViewApp) -> iced::Element<AppMessage> {
    use iced::Length;

    use iced::widget::{
        button, canvas, column, container, horizontal_rule, image::Handle, pick_list, row,
        scrollable, slider, text, vertical_rule,
    };

    let button = iced::widget::Button::new("Open")
        .on_press(AppMessage::OpenFileDialog)
        .width(Length::Fill);

    button.into()
}

pub fn main() -> iced::Result {
    ImageViewApp::run(iced::Settings::default())
}
