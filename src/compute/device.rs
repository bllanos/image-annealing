use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DeviceRequestError;

impl fmt::Display for DeviceRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error requesting device adapter")
    }
}

impl Error for DeviceRequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct DeviceManager {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl DeviceManager {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
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

    pub fn wait_for_device(&self) {
        self.device.poll(wgpu::Maintain::Wait);
    }
}
