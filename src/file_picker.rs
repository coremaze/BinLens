#[derive(Default)]
pub struct FilePicker {}

use std::path::PathBuf;

use iced::{advanced::Hasher, futures::FutureExt};
use iced_futures::{futures, subscription::EventStream};

impl iced_futures::subscription::Recipe for FilePicker {
    type Output = Option<PathBuf>;

    fn hash(&self, state: &mut Hasher) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: EventStream,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        let f = futures::stream::once(
            rfd::AsyncFileDialog::new()
                .pick_file()
                .map(|handle| handle.map(|h| h.path().to_owned())),
        );
        Box::pin(f)
    }
}
