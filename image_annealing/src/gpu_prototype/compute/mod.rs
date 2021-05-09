use super::{TextureComponent, TextureImageBuffer, N_TEXTURE_COMPONENTS};
use std::error::Error;
use std::fmt;

pub async fn execute_gpu(
    img: &TextureImageBuffer,
    numbers: Vec<u32>,
) -> Result<(TextureImageBuffer, Vec<u32>), Box<dyn Error>> {
    use std::convert::TryInto;
    use wgpu::util::DeviceExt;

    let slice_size = numbers.len() * std::mem::size_of::<u32>();
    let size = slice_size as wgpu::BufferAddress;
    let staging_buffer1 = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging_buffer1"),
        size,
        usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });

    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("storage_buffer"),
        contents: bytemuck::cast_slice(&numbers),
        usage: wgpu::BufferUsage::STORAGE
            | wgpu::BufferUsage::COPY_DST
            | wgpu::BufferUsage::COPY_SRC,
    });

    let bind_group_layout1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("storage_buffer_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::COMPUTE,
            ty: wgpu::BindingType::StorageBuffer {
                dynamic: false,
                min_binding_size: wgpu::BufferSize::new(4),
                readonly: false,
            },
            count: None,
        }],
    });

    let bind_group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind_group1"),
        layout: &bind_group_layout1,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(storage_buffer.slice(..)),
        }],
    });

    {
        cpass.dispatch(numbers.len() as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer1, 0, size);

    let result1 = data1
        .chunks_exact(4) // 4 bytes per chunk
        .map(|b| -> Result<u32, Box<dyn Error>> { Ok(u32::from_ne_bytes(b.try_into()?)) })
        .collect::<Result<_, _>>()?;

    Ok((output_image, result1))
}
