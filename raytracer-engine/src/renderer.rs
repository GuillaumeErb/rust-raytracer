use crate::color::Color;
use crate::engine::render;
use crate::engine::Scene;

#[allow(unused)]
pub fn render_scene_file(scene: &Scene) -> Result<(), String> {
    let screen = render(scene);
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(
        scene.camera.x_resolution as u32,
        scene.camera.y_resolution as u32,
    );

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = screen[&(x as u16, y as u16)].into();
    }
    imgbuf.save("output.png").unwrap();

    Ok(())
}

impl From<Color> for image::Rgb<u8> {
    fn from(item: Color) -> Self {
        image::Rgb([
            ((item.red as f32) * 255.0) as u8,
            ((item.green as f32) * 255.0) as u8,
            ((item.blue as f32) * 255.0) as u8,
        ])
    }
}
