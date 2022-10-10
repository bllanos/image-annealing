use crate::compute::device::{DeviceManager, DevicePollType};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

type MapResult = Result<(), wgpu::BufferAsyncError>;

pub struct BufferSliceMapFuture<'a> {
    device_manager: &'a DeviceManager,
    poll_type: DevicePollType,
    _receiver: futures_intrusive::channel::shared::GenericOneshotReceiver<
        parking_lot::RawMutex,
        MapResult,
    >,
    future: Pin<
        Box<
            futures_intrusive::channel::shared::ChannelReceiveFuture<
                parking_lot::RawMutex,
                MapResult,
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
        let future = Box::pin(receiver.receive());

        Self {
            device_manager,
            poll_type,
            _receiver: receiver,
            future,
        }
    }
}

impl<'a> Future for BufferSliceMapFuture<'a> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.future.as_mut().poll(cx) {
            Poll::Ready(result) => Poll::Ready(result.unwrap().unwrap()),
            Poll::Pending => {
                self.device_manager.poll_device(self.poll_type, cx);
                Poll::Pending
            }
        }
    }
}
