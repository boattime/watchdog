use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelFormat {
    Rgb,
    Bgra,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: Instant,
    pub format: PixelFormat,
    pub stride: usize,
}

impl Frame {
    pub fn new_rgb(data: Vec<u8>, width: u32, height: u32) -> Self {
        let stride = width as usize * 3;
        Self {
            data,
            width,
            height,
            timestamp: Instant::now(),
            format: PixelFormat::Rgb,
            stride,
        }
    }

    pub fn new_bgra(data: Vec<u8>, width: u32, height: u32, stride: usize) -> Self {
        Self {
            data,
            width,
            height,
            timestamp: Instant::now(),
            format: PixelFormat::Bgra,
            stride,
        }
    }

    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self::new_rgb(data, width, height)
    }

    pub fn pixel_count(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn bytes_per_pixel(&self) -> usize {
        match self.format {
            PixelFormat::Rgb => 3,
            PixelFormat::Bgra => 4,
        }
    }

    pub fn expected_data_size(&self) -> usize {
        self.stride * self.height as usize
    }

    pub fn get_pixel_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let offset = (y as usize * self.stride) + (x as usize * self.bytes_per_pixel());
        if offset < self.data.len() {
            Some(offset)
        } else {
            None
        }
    }

    pub fn get_pixel_bgra(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        if self.format != PixelFormat::Bgra {
            return None;
        }

        let offset = self.get_pixel_offset(x, y)?;
        if offset + 3 < self.data.len() {
            Some((
                self.data[offset],
                self.data[offset + 1],
                self.data[offset + 2],
                self.data[offset + 3],
            ))
        } else {
            None
        }
    }

    pub fn get_pixel_rgb(&self, x: u32, y: u32) -> Option<(u8, u8, u8)> {
        if self.format != PixelFormat::Rgb {
            return None;
        }

        let offset = self.get_pixel_offset(x, y)?;
        if offset + 2 < self.data.len() {
            Some((
                self.data[offset],
                self.data[offset + 1],
                self.data[offset + 2],
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct CaptureRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl CaptureRegion {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn full_screen() -> Self {
        Self::new(0, 0, 0, 0)
    }
}
