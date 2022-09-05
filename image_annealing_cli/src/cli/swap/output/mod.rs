use super::TaggedPermutation;
use crate::config::PermutationPath;
use futures_intrusive::buffer::ArrayBuf;
use futures_intrusive::channel::{self, TrySendError};
use image_annealing::compute::format::{ImageFileWriter, ImageFileWriterSaveResult};
use std::sync::mpsc;
use std::thread;

fn save_tagged_permutation(
    tagged_permutation: TaggedPermutation,
    path_prefix: &PermutationPath,
) -> ImageFileWriterSaveResult {
    let path_no_extension = format!(
        "{}_round_{}_pass_{}_{}",
        path_prefix,
        tagged_permutation.round_index,
        tagged_permutation.pass_index,
        tagged_permutation.pass.snake_case_name()
    );
    tagged_permutation
        .permutation
        .save_add_extension(path_no_extension)
}

type PathChannelBuffer = ArrayBuf<
    ImageFileWriterSaveResult,
    [ImageFileWriterSaveResult; TaggedPermutationWriter::CHANNEL_CAPACITY],
>;

/// A manager for a background thread that saves image files
///
/// This struct is inspired by the discussion at
/// https://github.com/image-rs/image/issues/1713
pub struct TaggedPermutationWriter {
    permutation_sender: Option<mpsc::SyncSender<TaggedPermutation>>,
    path_receiver: channel::shared::GenericReceiver<
        parking_lot::RawMutex,
        ImageFileWriterSaveResult,
        PathChannelBuffer,
    >,
    // TODO Use futures instead of threads if the `image` crate becomes more async-friendly
    //      (see https://github.com/image-rs/image/issues/1397).
    //
    //     To help with asynchronous operations, consider adding a method to `ImageFileWriter`
    //     to asynchronously encode images into a buffer, and a similar method to `ImageFileReader`,
    //     but for decoding.
    worker: Option<thread::JoinHandle<Result<(), TrySendError<ImageFileWriterSaveResult>>>>,
}

impl TaggedPermutationWriter {
    const CHANNEL_CAPACITY: usize = 1;

    pub fn new(path_prefix: &PermutationPath) -> Self {
        let path_prefix_clone = path_prefix.clone();
        let (permutation_sender, permutation_receiver) = mpsc::sync_channel(Self::CHANNEL_CAPACITY);
        let (path_sender, path_receiver) = channel::shared::generic_channel(Self::CHANNEL_CAPACITY);

        let thread = thread::spawn(move || loop {
            let message = permutation_receiver.recv();

            match message {
                Ok(tagged_permutation) => {
                    let result = save_tagged_permutation(tagged_permutation, &path_prefix_clone);
                    // `TaggedPermutationWriter` currently assumes that at most one `TaggedPermutation`
                    // is being saved at a time, and that the caller always collects the
                    // filepath of the saved permutation before submitting another permutation.
                    if let Err(err) = path_sender.try_send(result) {
                        break Err(err);
                    }
                }
                Err(_) => {
                    break Ok(());
                }
            }
        });

        Self {
            permutation_sender: Some(permutation_sender),
            path_receiver,
            worker: Some(thread),
        }
    }

    pub async fn save(&self, tagged_permutation: TaggedPermutation) -> ImageFileWriterSaveResult {
        let sender = self.permutation_sender.as_ref().unwrap();
        // `TaggedPermutationWriter` currently assumes that at most one `TaggedPermutation`
        // is being saved at a time.
        sender.try_send(tagged_permutation).unwrap();
        self.path_receiver.receive().await.unwrap()
    }
}

impl Drop for TaggedPermutationWriter {
    fn drop(&mut self) {
        drop(self.permutation_sender.take());
        let join_handle = self.worker.take().unwrap();
        // Any errors sending `ImageFileWriterSaveResult` back to the main thread will be unwrapped here.
        join_handle.join().unwrap().unwrap();
    }
}
