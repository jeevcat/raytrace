use cgmath::Vector3;
use image::Rgb;

pub type Color = Rgb<u8>;

const VW: f32 = 1.;
const VH: f32 = 1.;
const PROJECTION_PLANE_D: f32 = 1.;

pub struct Canvas {
    imgbuf: image::ImageBuffer<Color, Vec<<Color as image::Pixel>::Subpixel>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            imgbuf: image::ImageBuffer::new(width, height),
        }
    }

    /// x: canvas x
    /// y: canvas y
    pub fn put_pixel(&mut self, x: i32, y: i32, pixel: Color) {
        // we can think of the canvas as having its coordinate origin at the center,
        // with x increasing to the right and y increasing to the top of the screen.
        let range_x = (self.imgbuf.width() / 2) as i32;
        let range_y = (self.imgbuf.height() / 2) as i32;
        if x < -range_x || x >= range_x || y <= -range_y || y > range_y {
            return;
        }
        let screen_x = (range_x + x) as u32;
        let screen_y = (range_y - y) as u32;
        self.imgbuf.put_pixel(screen_x, screen_y, pixel)
    }

    pub fn canvas_to_viewport(&self, x: i32, y: i32) -> Vector3<f32> {
        Vector3 {
            x: x as f32 * VW / self.imgbuf.width() as f32,
            y: y as f32 * VH / self.imgbuf.height() as f32,
            z: PROJECTION_PLANE_D,
        }
    }

    pub fn save(&self) {
        self.imgbuf.save("out.png").unwrap();
    }
}
