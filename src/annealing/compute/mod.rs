use super::{TextureComponent, TextureImageBuffer, N_TEXTURE_COMPONENTS};
use std::error::Error;
use std::fmt;

const IMAGE_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Uint;
const IMAGE_TEXTURE_COMPONENT_SIZE: usize = std::mem::size_of::<TextureComponent>();
const IMAGE_TEXTURE_PIXEL_SIZE: usize = N_TEXTURE_COMPONENTS * IMAGE_TEXTURE_COMPONENT_SIZE;

const WORKGROUP_SIZE: (u32, u32) = (32, 32);

#[derive(Debug, Clone)]
pub struct DeviceError;

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error requesting device adapter")
    }
}

impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDimensionError;

impl fmt::Display for InvalidDimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid dimension supplied")
    }
}

impl Error for InvalidDimensionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// From https://github.com/gfx-rs/wgpu-rs/blob/master/examples/capture/main.rs
struct BufferDimensions {
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl BufferDimensions {
    fn new(sz: (u32, u32), bytes_per_pixel: usize) -> Self {
        let width = sz.0 as usize;
        let height = sz.1 as usize;
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

fn to_workgroup_size(sz: (u32, u32)) -> Result<(u32, u32), InvalidDimensionError> {
    if sz.0 == 0 || sz.1 == 0 {
        Err(InvalidDimensionError)
    } else {
        let remainder = (sz.0 % WORKGROUP_SIZE.0, sz.1 % WORKGROUP_SIZE.1);
        Ok((
            if remainder.0 == 0 {
                sz.0
            } else {
                (sz.0 - remainder.0)
                    .checked_add(WORKGROUP_SIZE.0)
                    .ok_or(InvalidDimensionError)?
            },
            if remainder.1 == 0 {
                sz.1
            } else {
                (sz.1 - remainder.1)
                    .checked_add(WORKGROUP_SIZE.1)
                    .ok_or(InvalidDimensionError)?
            },
        ))
    }
}

pub async fn execute_gpu(
    img: &TextureImageBuffer,
    numbers: Vec<u32>,
) -> Result<(TextureImageBuffer, Vec<u32>), Box<dyn Error>> {
    use std::convert::TryInto;
    use wgpu::util::DeviceExt;

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .ok_or(DeviceError)?;

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

    let cs_module1 =
        device.create_shader_module(wgpu::include_spirv!("../shader/annealing.comp.spv"));
    let cs_module2 =
        device.create_shader_module(wgpu::include_spirv!("../shader/image_test.comp.spv"));

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

    let texture_size = wgpu::Extent3d {
        width: img.width(),
        height: img.height(),
        depth: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("image_inout"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: IMAGE_TEXTURE_FORMAT,
        usage: wgpu::TextureUsage::STORAGE
            | wgpu::TextureUsage::COPY_DST
            | wgpu::TextureUsage::COPY_SRC,
    });

    queue.write_texture(
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        bytemuck::cast_slice(img.as_raw().as_slice()),
        wgpu::TextureDataLayout {
            offset: 0,
            bytes_per_row: (IMAGE_TEXTURE_PIXEL_SIZE as u32 * img.width()),
            rows_per_image: img.height(),
        },
        texture_size,
    );

    let buffer_dimensions = BufferDimensions::new(img.dimensions(), IMAGE_TEXTURE_PIXEL_SIZE);
    let staging_buffer2 = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging_buffer2"),
        size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
        usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        mapped_at_creation: false,
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

    let bind_group_layout2 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("image_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                dimension: wgpu::TextureViewDimension::D2,
                format: IMAGE_TEXTURE_FORMAT,
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

    let bind_group2 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind_group2"),
        layout: &bind_group_layout2,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&texture.create_view(
                &wgpu::TextureViewDescriptor {
                    label: Some("texture_view_descriptor"),
                    ..Default::default()
                },
            )),
        }],
    });

    let pipeline_layout1 = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("layout1"),
        bind_group_layouts: &[&bind_group_layout1],
        push_constant_ranges: &[],
    });

    let pipeline_layout2 = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("layout2"),
        bind_group_layouts: &[&bind_group_layout2],
        push_constant_ranges: &[],
    });

    let compute_pipeline1 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute_pipeline1"),
        layout: Some(&pipeline_layout1),
        compute_stage: wgpu::ProgrammableStageDescriptor {
            module: &cs_module1,
            entry_point: "main",
        },
    });

    let compute_pipeline2 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute_pipeline1"),
        layout: Some(&pipeline_layout2),
        compute_stage: wgpu::ProgrammableStageDescriptor {
            module: &cs_module2,
            entry_point: "main",
        },
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("command_encoder"),
    });
    {
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&compute_pipeline1);
        cpass.set_bind_group(0, &bind_group1, &[]);
        cpass.insert_debug_marker("compute_annealing_pass");
        cpass.dispatch(numbers.len() as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer1, 0, size);
    {
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&compute_pipeline2);
        cpass.set_bind_group(0, &bind_group2, &[]);
        cpass.insert_debug_marker("image_test_pass");
        let dispatch_dimensions = to_workgroup_size((texture_size.width, texture_size.height))?;
        cpass.dispatch(dispatch_dimensions.0, dispatch_dimensions.1, 1);
    }
    encoder.copy_texture_to_buffer(
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::BufferCopyView {
            buffer: &staging_buffer2,
            layout: wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: buffer_dimensions.padded_bytes_per_row as u32,
                rows_per_image: 0,
            },
        },
        texture_size,
    );

    queue.submit(Some(encoder.finish()));

    let buffer_slice1 = staging_buffer1.slice(..);
    let buffer_future1 = buffer_slice1.map_async(wgpu::MapMode::Read);

    let buffer_slice2 = staging_buffer2.slice(..);
    let buffer_future2 = buffer_slice2.map_async(wgpu::MapMode::Read);

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    buffer_future1.await?;
    let data1 = buffer_slice1.get_mapped_range();
    let result1 = data1
        .chunks_exact(4) // 4 bytes per chunk
        .map(|b| -> Result<u32, Box<dyn Error>> { Ok(u32::from_ne_bytes(b.try_into()?)) })
        .collect::<Result<_, _>>()?;

    drop(data1); // Drop all mapped views before unmapping the buffer
    staging_buffer1.unmap(); // Free host memory

    buffer_future2.await?;
    let data2 = buffer_slice2.get_mapped_range();
    let result2: Vec<TextureComponent> = data2
        .chunks(buffer_dimensions.padded_bytes_per_row)
        .flat_map(|c| {
            c[..buffer_dimensions.unpadded_bytes_per_row].chunks_exact(IMAGE_TEXTURE_COMPONENT_SIZE)
        })
        .map(|b| -> Result<TextureComponent, Box<dyn Error>> {
            Ok(TextureComponent::from_ne_bytes(b.try_into()?))
        })
        .collect::<Result<_, _>>()?;

    drop(data2); // Drop all mapped views before unmapping the buffer
    staging_buffer2.unmap(); // Free host memory

    let output_image = TextureImageBuffer::from_vec(img.width(), img.height(), result2).unwrap();

    Ok((output_image, result1))
}
