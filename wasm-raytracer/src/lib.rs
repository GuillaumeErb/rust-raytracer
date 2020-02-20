mod utils;

use wasm_bindgen::prelude::*;

use raytracer_engine::camera::ViewRay;
use raytracer_engine::color::Color;
use raytracer_engine::engine::get_object;
use raytracer_engine::engine::render;
use raytracer_engine::engine::render_pixel;
use raytracer_engine::engine::Scene;
use raytracer_engine::geometry::Vector3;
use raytracer_engine::parser::deserialize_scene;
use raytracer_engine::sample::*;

const SUBDIVISIONS: &[usize] = &[13, 11, 9, 7, 5, 4, 3, 2];

fn eligible_to_step(x: usize, y: usize, step: usize) -> bool {
    if step >= SUBDIVISIONS.len() {
        return eligible_to_last_step_for(x) && eligible_to_last_step_for(y);
    }
    return eligible_to_step_for(x, step) || eligible_to_step_for(y, step);
}

fn eligible_to_step_for(x: usize, step: usize) -> bool {
    for (i, item) in SUBDIVISIONS.iter().enumerate() {
        if i < step && x % item == 0 {
            return false;
        }
        if i == step {
            return x % item == 0;
        }
    }
    false
}

fn eligible_to_last_step_for(x: usize) -> bool {
    for item in SUBDIVISIONS.iter() {
        if x % item == 0 {
            return false;
        }
    }
    true
}

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
    step_rendering: Option<StepRendering>,
}

pub struct StepRendering {
    viewport: Vec<ViewRay>,
}

#[wasm_bindgen]
impl Screen {
    pub fn new(scene_string: String) -> Screen {
        utils::set_panic_hook();

        let mut scene = deserialize_scene(&scene_string);
        //scene.objects.build_kd_tree();
        let width = scene.camera.x_resolution;
        let height = scene.camera.y_resolution;

        let pixels = vec![0u8; width as usize * height as usize * 3];

        Screen {
            width,
            height,
            pixels,
            scene: scene,
            selected_object: None,
            step_rendering: None,
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

    fn initialize_step_rendering(&mut self, step: usize) {
        match step {
            0 => {
                let x = StepRendering {
                    viewport: self.scene.camera.generate_viewport(),
                };
                self.step_rendering = Some(x);
            }
            _ => (),
        }
    }

    #[wasm_bindgen(js_name = renderStep)]
    pub fn render_step(&mut self, step: usize) {
        self.initialize_step_rendering(step);
        let step_rendering = self.step_rendering.as_mut().unwrap();
        for view_ray in step_rendering.viewport.iter() {
            let x = view_ray.x as usize;
            let y = view_ray.y as usize;

            if eligible_to_step(x, y, step) {
                let result = render_pixel(&self.scene, view_ray.ray.clone());
                print_pixel(&mut self.pixels, self.width, x, y, result);
            }
        }
    }

    pub fn render(&mut self) {
        let screen = render(&self.scene);
        for ((xr, yr), color) in screen {
            let x = xr as usize;
            let y = yr as usize;
            print_pixel(&mut self.pixels, self.width, x, y, color);
        }
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
                    self.scene.objects.objects[id].geometry.translate(&Vector3 {
                        x: 1f64,
                        y: 0f64,
                        z: 0f64,
                    });
                }
                keycodes::KEY_K => {
                    self.scene.objects.objects[id].geometry.translate(&Vector3 {
                        x: -1f64,
                        y: 0f64,
                        z: 0f64,
                    });
                }
                keycodes::KEY_O => {
                    self.scene.objects.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: 1f64,
                        z: 0f64,
                    });
                }
                keycodes::KEY_L => {
                    self.scene.objects.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: -1f64,
                        z: 0f64,
                    });
                }
                keycodes::KEY_I => {
                    self.scene.objects.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: 1f64,
                    });
                }
                keycodes::KEY_P => {
                    self.scene.objects.objects[id].geometry.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: -1f64,
                    });
                }
                _ => (),
            },
            None => (),
        }
    }
}

fn print_pixel(pixels: &mut Vec<u8>, width: u16, x: usize, y: usize, color: Color) {
    let r = ((color.red as f32) * 255.0) as u8;
    let g = ((color.green as f32) * 255.0) as u8;
    let b = ((color.blue as f32) * 255.0) as u8;

    let w = width as usize;

    let start = (y * w + x) * 3;

    pixels[start] = r;
    pixels[start + 1] = g;
    pixels[start + 2] = b;
}
