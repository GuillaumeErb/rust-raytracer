mod utils;

use wasm_bindgen::prelude::*;

use raytracer_engine::engine::get_object;
use raytracer_engine::engine::render;
use raytracer_engine::engine::Scene;
use raytracer_engine::geometry::Vector3;
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
    selected_object: Option<usize>,
}

#[wasm_bindgen]
impl Screen {
    pub fn new() -> Screen {
        utils::set_panic_hook();

        let width = 600u16;
        let height = 600u16;

        let pixels = vec![0u8; width as usize * height as usize * 3];

        let mut scene = get_transparent_sphere_in_sphere();
        scene.camera.x_resolution = width;
        scene.camera.y_resolution = height;

        Screen {
            width,
            height,
            pixels,
            scene: scene,
            selected_object: None,
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
        log!("rendering ...");
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
        log!("done.");
    }

    pub fn click(&mut self, x: u16, y: u16) {
        log!("Click {:?}", (x, y));
        self.selected_object = get_object(&self.scene, y, x);
        log!("Got {:?}", self.selected_object);
    }

    pub fn keydown(&mut self, keycode: u8) {
        log!("Keycode {:?}", keycode);
        log!("Selected {:?}", self.selected_object);
        match self.selected_object {
            Some(id) => match keycode {
                keycodes::KEY_M => {
                    self.scene.objects[id].geometry.translate(&Vector3 {
                        x: 1f64,
                        y: 0f64,
                        z: 0f64,
                    });
                    self.render();
                }
                keycodes::KEY_K => {
                    self.scene.objects[id].geometry.translate(&Vector3 {
                        x: -1f64,
                        y: 0f64,
                        z: 0f64,
                    });
                    self.render();
                }
                keycodes::KEY_O => {
                    self.scene.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: 1f64,
                        z: 0f64,
                    });
                    self.render();
                }
                keycodes::KEY_L => {
                    self.scene.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: -1f64,
                        z: 0f64,
                    });
                    self.render();
                }
                keycodes::KEY_I => {
                    self.scene.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: 1f64,
                    });
                    self.render();
                }
                keycodes::KEY_P => {
                    self.scene.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: -1f64,
                    });
                    self.render();
                }
                _ => (),
            },
            None => (),
        }
    }
}
