use crate::ImageDimensions;
use std::convert::TryInto;

#[derive(Copy, Clone)]
pub struct TexturePaddingDimensions {
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl TexturePaddingDimensions {
    /// From https://github.com/gfx-rs/wgpu/blob/master/wgpu/examples/capture/main.rs
    fn new(sz: &ImageDimensions, bytes_per_pixel: usize) -> Self {
        let unpadded_bytes_per_row = sz.width() * bytes_per_pixel;
        let align: usize = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT.try_into().unwrap();
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }

    pub fn unpadded_bytes_per_row(&self) -> usize {
        self.unpadded_bytes_per_row
    }

    pub fn padded_bytes_per_row(&self) -> usize {
        self.padded_bytes_per_row
    }
}

#[derive(Copy, Clone)]
pub struct BufferDimensions {
    width: usize,
    height: usize,
    bytes_per_element: usize,
    padding: Option<TexturePaddingDimensions>,
}

impl BufferDimensions {
    pub fn new_buffer(count: usize, bytes_per_element: usize) -> Self {
        Self {
            width: count,
            height: 1,
            bytes_per_element,
            padding: None,
        }
    }

    pub fn new_texture_copy(sz: &ImageDimensions, bytes_per_pixel: usize) -> Self {
        Self {
            width: sz.width(),
            height: sz.height(),
            bytes_per_element: bytes_per_pixel,
            padding: Some(TexturePaddingDimensions::new(sz, bytes_per_pixel)),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn padding(&self) -> Option<&TexturePaddingDimensions> {
        self.padding.as_ref()
    }

    pub fn byte_size(&self) -> usize {
        match self.padding {
            Some(padding) => padding.padded_bytes_per_row() * self.height,
            None => self.height * self.width * self.bytes_per_element,
        }
    }
}
