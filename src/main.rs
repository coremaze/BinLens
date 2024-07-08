use std::{fmt::Display, fs, path::PathBuf, sync::Arc};

use iced::{
    mouse::ScrollDelta,
    widget::{checkbox, row},
    window, Application, Event, Subscription,
};
mod preview;
use preview::Preview;
use shader::DecodingScheme;

mod shader;

struct FileInfo {
    data: Arc<Vec<u8>>,
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
    ScrollWheel(ScrollDelta),
    BitOffset(u32),
    WindowResize { width: u32, height: u32 },
    ToggleGrid(bool),
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PixelMode {
    RGB,
    BGR,
    BPP8,
    G3B5R5G3,
}
impl PixelMode {
    pub const ALL: &'static [Self] = &[Self::RGB, Self::BGR, Self::BPP8, Self::G3B5R5G3];

    pub fn decoding_scheme(&self) -> &'static DecodingScheme {
        match &self {
            PixelMode::RGB => &DecodingScheme {
                red: [
                    Some(23),
                    Some(22),
                    Some(21),
                    Some(20),
                    Some(19),
                    Some(18),
                    Some(17),
                    Some(16),
                ],
                green: [
                    Some(15),
                    Some(14),
                    Some(13),
                    Some(12),
                    Some(11),
                    Some(10),
                    Some(9),
                    Some(8),
                ],
                blue: [
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                    Some(0),
                ],
                bits_per_pixel: 24,
            },
            PixelMode::BGR => &DecodingScheme {
                red: [
                    Some(15),
                    Some(14),
                    Some(13),
                    Some(12),
                    Some(11),
                    Some(10),
                    Some(9),
                    Some(8),
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
                    Some(31),
                    Some(30),
                    Some(29),
                    Some(28),
                    Some(27),
                    Some(26),
                    Some(25),
                    Some(24),
                ],
                bits_per_pixel: 24,
            },
            PixelMode::BPP8 => &DecodingScheme {
                red: [
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                    Some(0),
                ],
                green: [
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                    Some(0),
                ],
                blue: [
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                    None,
                ],
                bits_per_pixel: 8,
            },
            PixelMode::G3B5R5G3 => &DecodingScheme {
                red: [
                    None,
                    None,
                    None,
                    Some(12),
                    Some(11),
                    Some(10),
                    Some(9),
                    Some(8),
                ],
                green: [
                    None,
                    None,
                    Some(2),
                    Some(1),
                    Some(0),
                    Some(15),
                    Some(14),
                    Some(13),
                ],
                blue: [
                    None,
                    None,
                    None,
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(3),
                ],
                bits_per_pixel: 16,
            },
        }
    }
}
impl Display for PixelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PixelMode::RGB => "rgb",
            PixelMode::BGR => "bgr",
            PixelMode::BPP8 => "8bpp",
            PixelMode::G3B5R5G3 => "G3B5R5G3",
        })
    }
}

impl ImageViewApp {
    pub fn update_pixel_decoding(&mut self) {
        match &self.file {
            Some(file) => {
                // let pixels = match self.pixel_mode {
                //     PixelMode::RGB => file
                //         .data
                //         .chunks_exact(3)
                //         .map(|data| Pixel {
                //             red: data[0],
                //             green: data[1],
                //             blue: data[2],
                //         })
                //         .collect::<Vec<Pixel>>(),
                //     PixelMode::BGR => file
                //         .data
                //         .chunks_exact(3)
                //         .map(|data| Pixel {
                //             blue: data[0],
                //             green: data[1],
                //             red: data[2],
                //         })
                //         .collect::<Vec<Pixel>>(),
                //     PixelMode::BPP8 => file
                //         .data
                //         .iter()
                //         .map(|&data| Pixel {
                //             red: data,
                //             green: data,
                //             blue: data,
                //         })
                //         .collect::<Vec<Pixel>>(),
                // };

                self.preview.set_file_data(file.data.clone());
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
                // self.update_pixel_decoding();
                self.preview
                    .set_decoding_scheme(self.pixel_mode.decoding_scheme());
            }
            AppMessage::ImageWidthSelected(image_width) => {
                self.preview.set_target_width(image_width);
            }
            AppMessage::OpenFileDialog => {
                let file_info = get_file();
                if file_info.is_some() {
                    self.file = file_info;
                }
                self.update_pixel_decoding();
            }
            AppMessage::ImageScroll(scroll) => {
                let scroll = u32::MAX - scroll;
                let ratio = f64::from(u32::MAX) / self.preview.total_lines() as f64;
                let new_line: u64 = (f64::from(scroll) / ratio).round() as u64;
                self.preview.go_to_line(new_line);
            }
            AppMessage::WindowResize { width, height } => {
                self.preview.set_frame_height(height);
                self.preview.set_frame_width(width);
            }
            AppMessage::ImageScale(scale) => {
                self.preview.set_scale(scale);
            }
            AppMessage::BitOffset(offset) => {
                self.preview.set_start_bit(offset as u64);
            }
            AppMessage::ScrollWheel(delta) => {
                let lines_scrolled = match delta {
                    iced::mouse::ScrollDelta::Lines { x: _, y } => y * 5.0,
                    iced::mouse::ScrollDelta::Pixels { x: _, y } => {
                        //(y + (self.preview.scale() - 1) as f32) / self.preview.scale() as f32
                        y * 5.0
                    }
                }
                .round() as i64;

                let forward = lines_scrolled.is_negative();
                let amount = lines_scrolled.abs() as u64;

                let go_to_line = if forward {
                    self.preview.current_line().saturating_add(amount)
                } else {
                    self.preview.current_line().saturating_sub(amount)
                };

                self.preview.go_to_line(go_to_line);
            }
            AppMessage::ToggleGrid(grid) => {
                self.preview.set_grid(grid);
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        use iced::widget::{row, vertical_rule};

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
            Event::Mouse(iced::mouse::Event::WheelScrolled { delta }) => {
                Some(AppMessage::ScrollWheel(delta))
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

    Some(FileInfo {
        data: Arc::new(data),
        path,
    })
}

fn preview(app: &ImageViewApp) -> iced::Element<AppMessage> {
    use iced::Length;
    use iced::Padding;

    use iced::widget::{container, row, scrollable, vertical_slider};

    let file_len_bits = app.preview.file_data().len() * 8;
    let ratio = file_len_bits as f64 / u32::MAX as f64;
    let scroll_offset = (app.preview.start_bit() as f64 / ratio).round() as u32;

    let scrollbar = vertical_slider(
        0u32..=u32::MAX,
        u32::MAX - scroll_offset,
        AppMessage::ImageScroll,
    );

    let _dir = scrollable::Direction::Both {
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
        // container(row!(app.preview.clone()))
        //     // .direction(dir)
        container(
            iced::widget::shader(&app.preview.program)
                .width(Length::Fill)
                .height(Length::Fill)
        ),
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
    use iced::Length;

    use iced::widget::{column, container, horizontal_rule, pick_list, slider, text};

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
                text(format!("Image width: {}", app.preview.target_width())),
                slider(
                    1..=400,
                    app.preview.target_width(),
                    AppMessage::ImageWidthSelected
                )
            ),
            column!(
                text(format!("Scale: {}x", app.preview.scale())),
                slider(1..=10, app.preview.scale(), AppMessage::ImageScale)
            ),
            column!(
                text(format!(
                    "Start bit: {} ({} bytes, {} bits)",
                    app.preview.start_bit(),
                    app.preview.start_bit() / 8,
                    app.preview.start_bit() % 8
                )),
                slider(
                    0..=(24 * 8),
                    app.preview.start_bit() as u32,
                    AppMessage::BitOffset
                )
            ),
            checkbox("Grid", app.preview.grid())
                .on_toggle(|checked| { AppMessage::ToggleGrid(checked) }),
            // horizontal_rule(1),
            // text("Controls"),
            // text("Controls!"),
            // text("Controls!!"),
        )
        .width(400)
        .height(Length::Fill)
        .padding(10),
    );
    controls.into()
}

fn open_button(_app: &ImageViewApp) -> iced::Element<AppMessage> {
    use iced::Length;

    let button = iced::widget::Button::new("Open")
        .on_press(AppMessage::OpenFileDialog)
        .width(Length::Fill);

    button.into()
}

pub fn main() -> iced::Result {
    let settings = iced::Settings::default();
    ImageViewApp::run(settings)
}
