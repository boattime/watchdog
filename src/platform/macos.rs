use crate::{Frame, WatchDogError};
use core_graphics::access::ScreenCaptureAccess;
use core_graphics::display::CGDisplay;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};

pub struct MacOSCapture {
    access: bool,
}

impl MacOSCapture {
    pub fn new() -> Result<Self, WatchDogError> {
        let access = ScreenCaptureAccess.request();

        if !access {
            return Err(WatchDogError::PermissionDenied);
        }

        Ok(Self { access })
    }

    pub fn capture_full_screen(&self) -> Result<Frame, WatchDogError> {
        let display = CGDisplay::main();
        let display_bounds = display.bounds();
        let capture_rect = CGRect::new(
            &CGPoint::new(0.0, 0.0),
            &CGSize::new(display_bounds.size.width, display_bounds.size.height),
        );

        let cg_image = display.image_for_rect(capture_rect).ok_or_else(|| {
            WatchDogError::CaptureFailed(
                "Failed to capture screen image - possibly due to permissions".to_string(),
            )
        })?;

        let width = cg_image.width();
        let height = cg_image.height();
        let data = cg_image.data();
        let raw_bytes = data.bytes();

        // TODO: look into storing as a byte array without creating a vec
        let bgra_data = raw_bytes.to_vec();
        let bytes_per_row = cg_image.bytes_per_row();

        Ok(Frame::new_bgra(
            bgra_data,
            width as u32,
            height as u32,
            bytes_per_row,
        ))
    }
}
