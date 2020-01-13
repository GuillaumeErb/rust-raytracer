mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

use raytracer_engine::color::Color;
use raytracer_engine::engine::render;
use raytracer_engine::engine::Scene;
use raytracer_engine::sample::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Screen {
    width: u16,
    height: u16,
    pixels: Vec<u8>,
    scene: Scene,
}

#[wasm_bindgen]
impl Screen {

    pub fn new() -> Screen {
        utils::set_panic_hook();
        //log!("hey");
        let width = 600u16;
        let height = 600u16;

        let pixels = vec![0u8; (width as usize * height as usize * 3)];

        let mut scene = get_transparent_sphere_in_sphere();
        scene.camera.x_resolution = width;
        scene.camera.y_resolution = height;

        Screen {
            width,
            height,
            pixels,
            scene: scene
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    pub fn render(&mut self) {
        let screen = render(&self.scene);
        for ((xr, yr), color) in screen {

            let x = xr as usize;
            let y = yr as usize;

            let r = ((color.red as f32) * 255.0) as u8;
            let g = ((color.green as f32) * 255.0) as u8;
            let b = ((color.blue as f32) * 255.0) as u8;

            let w = self.width as usize;

            let start = (y * w + x) * 3;

            self.pixels[start] = r;
            self.pixels[start + 1] = g;
            self.pixels[start + 2] = b;
        }
    }
}
