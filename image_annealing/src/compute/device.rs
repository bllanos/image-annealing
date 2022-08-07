use std::error::Error;
use std::fmt;
use std::task::Context;

#[derive(Debug, Clone)]
pub struct DeviceRequestError;

impl fmt::Display for DeviceRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error requesting device adapter")
    }
}

impl Error for DeviceRequestError {}

#[derive(Clone, Copy)]
pub enum DevicePollType {
    Wait,
    Poll,
}

impl From<DevicePollType> for wgpu::Maintain {
    fn from(value: DevicePollType) -> Self {
        match value {
            DevicePollType::Wait => Self::Wait,
            DevicePollType::Poll => Self::Poll,
        }
    }
}

pub struct DeviceManager {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl DeviceManager {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or(DeviceRequestError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    ..Default::default()
                },
                None,
            )
            .await?;

        Ok(Self { device, queue })
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn poll_device(&self, poll_type: DevicePollType, cx: &mut Context<'_>) {
        self.device.poll(poll_type.into());
        cx.waker().wake_by_ref();
    }
}
