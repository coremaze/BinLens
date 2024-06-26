use std::{fmt::Display, fs, path::PathBuf};

use iced::Application;

struct FileInfo {
    data: Vec<u8>,
    path: PathBuf,
}
struct ImageViewApp {
    pixel_mode: PixelMode,
    image_width: u32,
    file: Option<FileInfo>,
}
#[derive(Debug, Clone)]
enum AppMessage {
    PixelModeSelected(PixelMode),
    ImageWidthSelected(u32),
    OpenFileDialog,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PixelMode {
    RGB,
}
impl PixelMode {
    pub const ALL: &'static [Self] = &[Self::RGB];
}
impl Display for PixelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PixelMode::RGB => "rgb",
        })
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
                image_width: 256,
                file: None,
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
            AppMessage::PixelModeSelected(pixel_mode) => self.pixel_mode = pixel_mode,
            AppMessage::ImageWidthSelected(image_width) => self.image_width = image_width,
            AppMessage::OpenFileDialog => {
                let file_info = get_file();
                if file_info.is_some() {
                    self.file = file_info;
                }
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
    use iced::Length;

    use iced::widget::{
        button, canvas, column, container, horizontal_rule, image::Handle, pick_list, row,
        scrollable, slider, text, vertical_rule,
    };

    let width = app.image_width;

    let image = match &app.file {
        Some(file) => {
            let width = app.image_width;
            let height = (file.data.len() / width as usize) as u32;

            let mut pixel_data = vec![0u8; width as usize * height as usize * 4];

            for (dst, src) in pixel_data
                .chunks_exact_mut(4)
                .zip(file.data.chunks_exact(4))
            {
                dst[0] = src[0];
                dst[1] = src[1];
                dst[2] = src[2];
                dst[3] = 0xFF;
            }

            iced::widget::Image::<Handle>::new(Handle::from_pixels(width, height, pixel_data))
        }
        None => {
            let mut pixels = vec![0u8; 1024 * 1024 * 4];
            for (i, e) in pixels.chunks_exact_mut(4).enumerate() {
                let y = i / 1024;
                let x = i % 1024;

                let r = (y / 4) as u8;
                let g = (x / 4) as u8;
                let b = r.saturating_add(g) / 2;

                e[0] = r;
                e[1] = g;
                e[2] = b;
                e[3] = 255;
            }

            iced::widget::Image::<Handle>::new(Handle::from_pixels(1024, 1024, pixels))
        }
    };

    let dir = scrollable::Direction::Both {
        vertical: scrollable::Properties::default(),
        horizontal: scrollable::Properties::default(),
    };

    let preview = container(
        scrollable(image)
            .direction(dir)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .padding(10);

    preview.into()
}

fn controls(app: &ImageViewApp) -> iced::Element<AppMessage> {
    use iced::Length;

    use iced::widget::{
        button, canvas, column, container, horizontal_rule, image::Handle, pick_list, row,
        scrollable, slider, text, vertical_rule,
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
                text(format!("Image width: {}", app.image_width)),
                slider(0..=512, app.image_width, AppMessage::ImageWidthSelected)
            ),
            horizontal_rule(1),
            text("Controls"),
            text("Controls!"),
            text("Controls!!")
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
