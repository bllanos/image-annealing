use crate::compute::device::{DeviceManager, DevicePollType};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct BufferSliceMapFuture<'a> {
    device_manager: &'a DeviceManager,
    poll_type: DevicePollType,
    future: Pin<
        Box<
            futures_intrusive::channel::shared::ChannelReceiveFuture<
                parking_lot::RawMutex,
                Result<(), wgpu::BufferAsyncError>,
            >,
        >,
    >,
}

impl<'a> BufferSliceMapFuture<'a> {
    pub fn new(
        buffer_slice: &wgpu::BufferSlice,
        device_manager: &'a DeviceManager,
        poll_type: DevicePollType,
    ) -> Self {
        let (sender, receiver) = futures_intrusive::channel::shared::generic_oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).unwrap()
        });

        Self {
            device_manager,
            poll_type,
            future: Box::pin(receiver.receive()),
        }
    }
}

impl<'a> Future for BufferSliceMapFuture<'a> {
    type Output = Result<(), Box<dyn Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.future.as_mut().poll(cx) {
            Poll::Ready(result) => Poll::Ready(
                result
                    .unwrap()
                    .map_err(|err| Box::new(err) as Box<dyn Error>),
            ),
            Poll::Pending => {
                self.device_manager.poll_device(self.poll_type, cx);
                Poll::Pending
            }
        }
    }
}
